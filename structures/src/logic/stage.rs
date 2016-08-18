extern crate find_folder;

use std::sync::{Arc, Mutex};
use std::sync::mpsc;
use std::collections::HashMap;
use rustc_serialize::json;
use std::io::prelude::*;
use std::io::{BufReader, BufWriter};
use std::fs::File;
use std::path::PathBuf;

use io::config::{get_config_path, Config};

use logic::channel::DmxAddress;
use logic::channel::DmxValue;
use logic::fade::FadeTime;

use logic::Fixture;
use logic::ChannelGroup;
use logic::Channel;

use logic::ChannelGroupValue;
use logic::Switch;
use logic::fade::FadeCurve;

use networking::UDPSocket;

use logic::Chaser;
use ui::frontend_data::FrontendData;

#[derive(Debug)]
/// The implementation of a Stage.
pub struct Stage {
    /// The name of the stage.
    pub name: String,
    /// The list of all channels.
    pub channels: Vec<Arc<Mutex<Channel>>>,
    /// The list if all fixtures.
    pub fixtures: Vec<Fixture>,
    /// The list of all switches.
    pub switches: Vec<Switch>,
    /// The sander for the interface.
    dmx_tx: mpsc::Sender<(DmxAddress, DmxValue)>,
    /// The list of all chasers
    pub chasers: HashMap<String, Chaser>
}

impl Stage {
    /// Generate a Stage with default values.
    pub fn new(name: String, dmx_tx: mpsc::Sender<(DmxAddress, DmxValue)>) -> Stage {
        Stage {
            name: name,
            channels: Vec::new(),
            fixtures: Vec::new(),
            switches: Vec::new(),
            chasers: HashMap::new(),
            dmx_tx: dmx_tx
        }
    }

    /// Convert a stage into FrontendData
    pub fn get_frontend_data(&self) -> FrontendData {
        FrontendData {
            name: self.name.clone(),
            max_dmx_address: self.channels.len() as DmxAddress,
            fixtures: self.fixtures.iter().map(|x| x.to_empty_fixture()).collect(),
            switches: self.switches.iter().map(|x| x.with_json_hashmap()).collect(),
            chasers: self.chasers.iter().map(|(name, data)| (name.clone(), data.get_frontend_data())).collect()
        }
    }
    /// Return the path to the project configuration file.
    fn get_config_filename(&self) -> PathBuf {
        get_config_path(Config::Server(self.name.clone())).join(self.name.clone()  + ".server.dmx")
    }
    /// Load the project configuration
    pub fn load_config(&mut self) {
        let path = self.get_config_filename();
        match File::open(path) {
            Ok(file) => {
                let mut buf = BufReader::new(file);
                let mut json_data: String = String::new();
                let _ = buf.read_to_string(&mut json_data);
                let frontend_data = json::decode(&json_data).unwrap();
                self.from_frontend_data(frontend_data);
            },
            _ => {}
        }
    }
    /// Save the project configuration
    pub fn save_config(&self) {
        let path = self.get_config_filename();
        let file = File::create(path).expect("no such file");
        let mut buf = BufWriter::new(file);
        buf.write_all(self.get_frontend_data().get_json_string().as_bytes()).unwrap();
    }
    /// Convert FrontendData into a stage
    pub fn from_frontend_data(&mut self, frontend_data: FrontendData) {

        self.switches = frontend_data.switches.iter().map(|x| Switch::load_from_json_switch(x.clone())).collect();

        for (_, chaser) in self.chasers.iter_mut() {
            chaser.stop_chaser();
        }
        self.chasers = frontend_data.chasers.iter().map(|(name, data)| (name.clone(), Chaser::from_frontend_data(data.clone()))).collect()

    }
    /// Add a fixture to the stage.
    pub fn add_fixture(&mut self, fixture: Fixture) -> usize {
        self.fixtures.push(fixture);
        self.fixtures.len() - 1
    }
    /// Add a switch to a chaser.
    fn add_switch_to_chaser(&mut self, switch_id:usize, chaser_name: String) {
        if !self.chasers.contains_key(&chaser_name) {
            self.chasers.insert(chaser_name.clone(), Chaser::new());
        }
        self.chasers.get_mut(&chaser_name).unwrap().switches.push(switch_id);
    }
    /// Add  a switch to the stage.
    pub fn add_switch(&mut self, switch: Switch) -> usize {
        let id = self.switches.len();
        self.add_switch_to_chaser(id, switch.chaser_name.clone());
        self.switches.push(switch);

        id
    }
    /// Deactivate all switches in a chaser of a given switch except of this switch.
    pub fn deactivate_group_of_switch(&mut self, switch_id: usize, kill_others: bool) {
        let switches = self.chasers.get(&self.switches[switch_id].chaser_name).unwrap().switches.iter().filter(|&x| *x != switch_id).map(|&x| x).collect::<Vec<usize>>();

        for switch_id in switches.iter() {
            self.set_switch(*switch_id, 0.0, kill_others);
        };
    }

    /// Set the dimmer value of a switch. To deactivate a switch set the dimmer value to 0.
    pub fn set_switch(&mut self, switch_id: usize, dimmer_value: f64, kill_others: bool) {
        self.switches[switch_id].dimmer_value = dimmer_value;
        if dimmer_value == 0.0 {
            self.deactivate_switch(switch_id, kill_others);
        }
        else {
            for (&(fixture_id, channel_group_id), data) in self.switches[switch_id].channel_groups.iter() {
                let new_values: Vec<_> = data.values.iter().map(|a| (*a as f64 * (dimmer_value / 255.0)) as DmxValue).collect();
                println!("new_values: {:?}   {:?}", new_values, data);
                match self.fixtures[fixture_id].channel_groups[channel_group_id] {
                    //TODO Check if there are enough values in new_values
                    ChannelGroup::Single(ref mut group) => {
                        group.active_switches.push((switch_id, ChannelGroupValue::from_tuple((new_values.clone(), (data.curve_in.clone(), data.time_in), (data.curve_out.clone(), data.time_out)))));
                        group.fade_simple(data.curve_in.clone(), data.time_in, new_values[0], kill_others);
                    },
                    ChannelGroup::RGB(ref mut group) => {
                        group.active_switches.push((switch_id, ChannelGroupValue::from_tuple((new_values.clone(), (data.curve_in.clone(), data.time_in), (data.curve_out.clone(), data.time_out)))));
                        group.fade_simple(data.curve_in.clone(), data.time_in, new_values[0], new_values[1], new_values[2], kill_others);
                    },
                    ChannelGroup::RGBA(ref mut group) => {
                        group.active_switches.push((switch_id, ChannelGroupValue::from_tuple((new_values.clone(), (data.curve_in.clone(), data.time_in), (data.curve_out.clone(), data.time_out)))));
                        group.fade_simple(data.curve_in.clone(), data.time_in, new_values[0], new_values[1], new_values[2], new_values[3], kill_others);
                    },
                    ChannelGroup::Moving2D(ref mut group) => {
                        group.active_switches.push((switch_id, ChannelGroupValue::from_tuple((new_values.clone(), (data.curve_in.clone(), data.time_in), (data.curve_out.clone(), data.time_out)))));
                        group.fade_simple(data.curve_in.clone(), data.time_in, new_values[0], new_values[1], kill_others);
                    },
                    ChannelGroup::Moving2D16(ref mut group) => {
                        group.active_switches.push((switch_id, ChannelGroupValue::from_tuple((new_values.clone(), (data.curve_in.clone(), data.time_in), (data.curve_out.clone(), data.time_out)))));
                        group.fade_simple(data.curve_in.clone(), data.time_in, new_values[0], new_values[1], new_values[2], new_values[3], kill_others);
                    }
                }
            }
        }
        let addr_high = (switch_id >> 8) as u8;
        let addr_low = switch_id as u8;
        UDPSocket::new().start_frontend_client().send_to_multicast(&[1, addr_high, addr_low, dimmer_value as u8]);
    }

    /// Deactivate a switch
    fn deactivate_switch(&mut self, switch_id: usize, kill_others: bool) {
        for (&(fixture_id, channel_group_id), data) in self.switches[switch_id].channel_groups.iter() {
            match self.fixtures[fixture_id].channel_groups[channel_group_id] {
                ChannelGroup::Single(ref mut group) => {
                    if remove_from_active_switches(&mut group.active_switches, switch_id) {
                        let (new_values, new_curve, new_time) = extract_new_values(&mut group.active_switches, vec![0], data.curve_out.clone(), data.time_out);
                        group.fade_simple(new_curve, new_time, new_values[0], kill_others);
                    }
                },
                ChannelGroup::RGB(ref mut group) => {
                    if remove_from_active_switches(&mut group.active_switches, switch_id) {
                        let (new_values, new_curve, new_time) = extract_new_values(&mut group.active_switches, vec![0, 0, 0], data.curve_out.clone(), data.time_out);
                        group.fade_simple(new_curve, new_time, new_values[0], new_values[1], new_values[2], kill_others);
                    }
                },
                ChannelGroup::RGBA(ref mut group) => {
                    if remove_from_active_switches(&mut group.active_switches, switch_id) {
                        let (new_values, new_curve, new_time) = extract_new_values(&mut group.active_switches, vec![0, 0, 0, 0], data.curve_out.clone(), data.time_out);
                        group.fade_simple(new_curve, new_time, new_values[0], new_values[1], new_values[2], new_values[3], kill_others);
                    }
                },
                ChannelGroup::Moving2D(ref mut group) => {
                    if remove_from_active_switches(&mut group.active_switches, switch_id) {
                        let (new_values, new_curve, new_time) = extract_new_values(&mut group.active_switches, vec![0, 0], data.curve_out.clone(), data.time_out);
                        group.fade_simple(new_curve, new_time, new_values[0], new_values[1], kill_others);
                    }
                },
                ChannelGroup::Moving2D16(ref mut group) => {
                    if remove_from_active_switches(&mut group.active_switches, switch_id) {
                        let (new_values, new_curve, new_time) = extract_new_values(&mut group.active_switches, vec![0, 0, 0, 0], data.curve_out.clone(), data.time_out);
                        group.fade_simple(new_curve, new_time, new_values[0], new_values[1], new_values[2], new_values[3], kill_others);
                    }
                }
            }
        }
    }
    /// Return a channel of a given dmx address.
    pub fn get_channel_object(&mut self, channel: DmxAddress) -> Arc<Mutex<Channel>> {
        for i in self.channels.len() as u16..channel as u16 {
            self.channels.push(Arc::new(Mutex::new(Channel::new(i + 1, 0, self.dmx_tx.clone()))));
            trace!("Created channel {}", i + 1);
        }
        self.channels[channel as usize - 1].clone()
    }
}
/// Removes a switch from a list of activated switches.
fn remove_from_active_switches(active_switches: &mut Vec<(usize, ChannelGroupValue)>, switch_id: usize) -> bool {
    if active_switches.len() > 0 { //TODO: Replace this workaround.
        let last_index = active_switches.len() - 1;
        let last_id = active_switches[last_index].0;
        active_switches.retain(|&(x, _)| x != switch_id);
        last_id == switch_id
    } else { false }
}
/// Generate new values for a channel group based of the history of activated switches.
fn extract_new_values(active_switches: &mut Vec<(usize, ChannelGroupValue)>, default_values: Vec<DmxValue>, old_curve: FadeCurve, old_time: FadeTime) -> (Vec<DmxValue>, FadeCurve, FadeTime) {
    if active_switches.len() == 0 {
        (default_values, old_curve, old_time)
    }
    else {
        let last_index = active_switches.len() - 1;
        (active_switches[last_index].1.values.clone(), active_switches[last_index].1.curve_in.clone(), active_switches[last_index].1.time_in)
    }
}
