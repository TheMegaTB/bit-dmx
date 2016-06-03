extern crate find_folder;

use std::sync::{Arc, Mutex};
use std::sync::mpsc;
use std::collections::HashMap;
use std::time::Duration;
use std::thread::{self, sleep};
use rustc_serialize::json;
use std::io::prelude::*;
use std::io::{BufReader, BufWriter};
use std::fs::File;
use std::path::PathBuf;

use super::io::config::{get_config_path, Config};

use DmxAddress;
use DmxValue;
use FadeTime;

use Fixture;
use ChannelGroup;
use Channel;

use ChannelGroupValue;
use Switch;
use FadeCurve;

use networking::UDPSocket;

use Chaser;
use super::ui::frontend_data::FrontendData;

#[derive(Debug)]
pub struct Stage {
    pub name: String,
    pub channels: Vec<Arc<Mutex<Channel>>>,
    pub fixtures: Vec<Fixture>,
    switches: Vec<Switch>,
    dmx_tx: mpsc::Sender<(DmxAddress, DmxValue)>,
    chasers: HashMap<String, Chaser>
}

impl Stage {
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

    pub fn get_frontend_data(&self) -> FrontendData {
        FrontendData {
            name: self.name.clone(),
            max_dmx_address: self.channels.len() as DmxAddress,
            fixtures: self.fixtures.iter().map(|x| x.to_empty_fixture()).collect(),
            switches: self.switches.iter().map(|x| x.with_json_hashmap()).collect(),
            chasers: self.chasers.iter().map(|(name, data)| (name.clone(), data.get_frontend_data())).collect()
        }
    }

    fn get_config_filename(&self) -> PathBuf {
        get_config_path(Config::Server, &self.name).join(self.name.clone()  + ".server.dmx")
    }

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

    pub fn save_config(&self) {
        let path = self.get_config_filename();
        let file = File::create(path).expect("no such file");
        let mut buf = BufWriter::new(file);
        buf.write_all(self.get_frontend_data().get_json_string().as_bytes()).unwrap();
    }

    pub fn from_frontend_data(&mut self, frontend_data: FrontendData) {

        self.switches = frontend_data.switches.iter().map(|x| Switch::load_from_json_switch(x.clone())).collect();

        for (_, chaser) in self.chasers.iter_mut() {
            chaser.stop_chaser();
        }
        self.chasers = frontend_data.chasers.iter().map(|(name, data)| (name.clone(), Chaser::from_frontend_data(data.clone()))).collect()

    }

    pub fn add_fixture(&mut self, fixture: Fixture) -> usize {
        self.fixtures.push(fixture);
        self.fixtures.len() - 1
    }

    fn add_fixture_to_switch_group(&mut self, switch_id:usize, chaser_id: String) {
        if !self.chasers.contains_key(&chaser_id) {
            self.chasers.insert(chaser_id.clone(), Chaser::new());
        }
        self.chasers.get_mut(&chaser_id).unwrap().switches.push(switch_id);
    }

    pub fn add_switch(&mut self, switch: Switch) -> usize {
        let id = self.switches.len();
        self.add_fixture_to_switch_group(id, switch.chaser_id.clone());
        self.switches.push(switch);

        id
    }

    pub fn deactivate_group_of_switch(&mut self, switch_id: usize, kill_others: bool) {
        let switches = self.chasers.get(&self.switches[switch_id].chaser_id).unwrap().switches.iter().filter(|&x| *x != switch_id).map(|&x| x).collect::<Vec<usize>>();

        for switch_id in switches.iter() {
            self.set_switch(*switch_id, 0.0, kill_others);
        };
    }

    pub fn set_switch(&mut self, switch_id: usize, dimmer_value: f64, kill_others: bool) {
        self.switches[switch_id].dimmer_value = dimmer_value;
        if dimmer_value == 0.0 {
            self.deactivate_switch(switch_id, kill_others);
        }
        else {
            for (&(fixture_id, channel_group_id), data) in self.switches[switch_id].channel_groups.iter() {
                let new_values: Vec<_> = data.values.iter().map(|a| (*a as f64 * (dimmer_value / 255.0)) as DmxValue).collect();
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
                    }
                }
            }
        }
        let addr_high = (switch_id >> 8) as u8;
        let addr_low = switch_id as u8;
        UDPSocket::new().start_frontend_client().send_to_multicast(&[1, addr_high, addr_low, dimmer_value as u8]);
    }

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
                }
            }
        }
    }

    pub fn get_channel_object(&mut self, channel: DmxAddress) -> Arc<Mutex<Channel>> {
        for i in self.channels.len() as u16..channel as u16 {
            self.channels.push(Arc::new(Mutex::new(Channel::new(i + 1, 0, 0, self.dmx_tx.clone()))));
            trace!("Created channel {}", i + 1);
        }
        self.channels[channel as usize - 1].clone()
    }
}


pub fn start_chaser_of_switch(stage: Arc<Mutex<Stage>>, switch_id: usize, dimmer_value: f64) {

    let addr_high = (switch_id >> 8) as u8;
    let addr_low = switch_id as u8;
    UDPSocket::new().start_frontend_client().send_to_multicast(&[2, addr_high, addr_low, dimmer_value as u8]);
    let (chaser, rx) = {
        let mut stage_locked = stage.lock().expect("Failed to lock Arc!");
        let chaser_id = stage_locked.switches[switch_id].clone().chaser_id;
        let mut chaser = stage_locked.chasers.get_mut(&chaser_id).unwrap();
            chaser.stop_chaser();
        if dimmer_value == 0.0 {
            return
        }
        let (tx, rx) = mpsc::channel();
        chaser.current_thread = Some(tx);
        (chaser.clone(), rx)
    };
    thread::spawn(move || {
        let mut current_switch_id_in_chaser: usize = 0; //TODO use switch_id
        loop {
            {stage.lock().expect("Failed to lock Arc!").deactivate_group_of_switch(chaser.switches[current_switch_id_in_chaser], false);}
            {stage.lock().expect("Failed to lock Arc!").set_switch(chaser.switches[current_switch_id_in_chaser], dimmer_value, true);}
            let sleep_time = {
                let stage_locked = stage.lock().expect("Failed to lock Arc!");
                stage_locked.switches[chaser.switches[current_switch_id_in_chaser]].before_chaser as u64
            };
            sleep(Duration::from_millis(sleep_time));
            if rx.try_recv().is_ok() { return };
            current_switch_id_in_chaser = (current_switch_id_in_chaser + 1) % chaser.switches.len();
        }
    });
}

fn remove_from_active_switches(active_switches: &mut Vec<(usize, ChannelGroupValue)>, switch_id: usize) -> bool {
    if active_switches.len() > 0 { //TODO: Replace this workaround.
        let last_index = active_switches.len() - 1;
        let last_id = active_switches[last_index].0;
        active_switches.retain(|&(x, _)| x != switch_id);
        last_id == switch_id
    } else { false }
}

fn extract_new_values(active_switches: &mut Vec<(usize, ChannelGroupValue)>, default_values: Vec<DmxValue>, old_curve: FadeCurve, old_time: FadeTime) -> (Vec<DmxValue>, FadeCurve, FadeTime) {
    if active_switches.len() == 0 {
        (default_values, old_curve, old_time)
    }
    else {
        let last_index = active_switches.len() - 1;
        (active_switches[last_index].1.values.clone(), active_switches[last_index].1.curve_in.clone(), active_switches[last_index].1.time_in)
    }
}
