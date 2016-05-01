use std::sync::{Arc, Mutex};
use std::sync::mpsc;
use std::collections::HashMap;

use DmxAddress;
use DmxValue;
use FadeTime;

use Fixture;
use EmptyFixture;
use ChannelGroup;
use Channel;

use Switch;
use ChannelGroupValue;
use FadeCurve;
use JsonSwitch;


#[derive(Debug, RustcDecodable, RustcEncodable)]
pub struct FrontendData {
    pub max_dmx_address: DmxAddress,
    pub fixtures: Vec<EmptyFixture>,
    pub switches: Vec<JsonSwitch>,
    pub chasers: HashMap<String, Vec<usize>>
}

impl FrontendData {
    pub fn new() -> FrontendData {
        FrontendData {
            max_dmx_address: 0,
            fixtures: Vec::new(),
            switches: Vec::new(),
            chasers: HashMap::new()
        }
    }
}


#[derive(Debug)]
pub struct Stage {
    pub channels: Vec<Arc<Mutex<Channel>>>,
    pub fixtures: Vec<Fixture>,
    switches: Vec<Switch>,
    dmx_tx: mpsc::Sender<(DmxAddress, DmxValue)>,
    chasers: HashMap<String, Vec<usize>>
}

impl Stage {
    pub fn new(dmx_tx: mpsc::Sender<(DmxAddress, DmxValue)>) -> Stage {
        Stage {
            channels: Vec::new(),
            fixtures: Vec::new(),
            switches: Vec::new(),
            chasers: HashMap::new(),
            dmx_tx: dmx_tx
        }
    }

    pub fn get_frontend_data(&self) -> FrontendData {
        FrontendData {
            max_dmx_address: self.channels.len() as DmxAddress,
            fixtures: self.fixtures.iter().map(|x| x.to_empty_fixture()).collect(),
            switches: self.switches.iter().map(|x| x.with_json_hashmap()).collect(),
            chasers: self.chasers.clone()
        }
    }

    pub fn add_fixture(&mut self, fixture: Fixture) -> usize {
        self.fixtures.push(fixture);
        self.fixtures.len() - 1
    }

    fn add_fixture_to_switch_group(&mut self, switch_id:usize, chaser_id: String) {
        if !self.chasers.contains_key(&chaser_id) {
            self.chasers.insert(chaser_id.clone(), Vec::new());
        }
        self.chasers.get_mut(&chaser_id).unwrap().push(switch_id);
    }

    pub fn add_switch(&mut self, switch: Switch) -> usize {
        let id = self.switches.len();
        self.add_fixture_to_switch_group(id, switch.chaser_id.clone());
        self.switches.push(switch);

        id
    }

    pub fn deactivate_group_of_switch(&mut self, switch_id: usize) -> Vec<usize> {
        let switches = self.chasers.get(&self.switches[switch_id].chaser_id).unwrap().iter().filter(|&x| *x != switch_id).map(|&x| x).collect::<Vec<usize>>();

        for switch_id in switches.iter() {
            self.set_switch(*switch_id, 0.0);
        };
        switches
    }

    pub fn set_switch(&mut self, switch_id: usize, dimmer_value: f64) {
        self.switches[switch_id].dimmer_value = dimmer_value;
        if dimmer_value == 0.0 {
            self.deactivate_switch(switch_id);
        }
        else {
            for (&(fixture_id, channel_group_id), &(ref values, (ref curve_in, time_in), (ref curve_out, time_out))) in self.switches[switch_id].channel_groups.iter() {
                let new_values: Vec<_> = values.iter().map(|a| (*a as f64 * (dimmer_value / 255.0)) as DmxValue).collect();
                match self.fixtures[fixture_id].channel_groups[channel_group_id] {
                    ChannelGroup::Single(ref mut group) => {
                        group.active_switches.push((switch_id, (new_values.clone(), (curve_in.clone(), time_in), (curve_out.clone(), time_out))));
                        group.fade_simple(curve_in.clone(), time_in, new_values[0]);
                    },
                    ChannelGroup::RGB(ref mut group) => {
                        group.active_switches.push((switch_id, (new_values.clone(), (curve_in.clone(), time_in), (curve_out.clone(), time_out))));
                        group.fade_simple(curve_in.clone(), time_in, new_values[0], new_values[1], new_values[2]);
                    },
                    ChannelGroup::RGBA(ref mut group) => {
                        group.active_switches.push((switch_id, (new_values.clone(), (curve_in.clone(), time_in), (curve_out.clone(), time_out))));
                        group.fade_simple(curve_in.clone(), time_in, new_values[0], new_values[1], new_values[2], new_values[3]);
                    },
                    ChannelGroup::Moving2D(ref mut group) => {
                        group.active_switches.push((switch_id, (new_values.clone(), (curve_in.clone(), time_in), (curve_out.clone(), time_out))));
                        group.fade_simple(curve_in.clone(), time_in, new_values[0], new_values[1]);
                    }
                }
            }
        }
    }

    fn deactivate_switch(&mut self, switch_id: usize) {
        for (&(fixture_id, channel_group_id), &(_, _, (ref curve_out, time_out))) in self.switches[switch_id].channel_groups.iter() {
            match self.fixtures[fixture_id].channel_groups[channel_group_id] {
                ChannelGroup::Single(ref mut group) => {
                    if remove_from_active_switches(&mut group.active_switches, switch_id) {
                        let (new_values, new_curve, new_time) = extract_new_values(&mut group.active_switches, vec![0], curve_out.clone(), time_out);
                        group.fade_simple(new_curve, new_time, new_values[0]);
                    }
                },
                ChannelGroup::RGB(ref mut group) => {
                    if remove_from_active_switches(&mut group.active_switches, switch_id) {
                        let (new_values, new_curve, new_time) = extract_new_values(&mut group.active_switches, vec![0, 0, 0], curve_out.clone(), time_out);
                        group.fade_simple(new_curve, new_time, new_values[0], new_values[1], new_values[2]);
                    }
                },
                ChannelGroup::RGBA(ref mut group) => {
                    if remove_from_active_switches(&mut group.active_switches, switch_id) {
                        let (new_values, new_curve, new_time) = extract_new_values(&mut group.active_switches, vec![0, 0, 0, 0], curve_out.clone(), time_out);
                        group.fade_simple(new_curve, new_time, new_values[0], new_values[1], new_values[2], new_values[3]);
                    }
                },
                ChannelGroup::Moving2D(ref mut group) => {
                    if remove_from_active_switches(&mut group.active_switches, switch_id) {
                        let (new_values, new_curve, new_time) = extract_new_values(&mut group.active_switches, vec![0, 0], curve_out.clone(), time_out);
                        group.fade_simple(new_curve, new_time, new_values[0], new_values[1]);
                    }
                }
            }
        }
    }

    pub fn get_channel_object(&mut self, channel: DmxAddress) -> Arc<Mutex<Channel>> {
        for i in self.channels.len() as u16..channel as u16 {
            self.channels.push(Arc::new(Mutex::new(Channel::new(i + 1, 0, 0, self.dmx_tx.clone()))));
            trace!("Create channel {}", i + 1);
        }
        self.channels[channel as usize - 1].clone()
    }
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
        ((active_switches[last_index].1).0.clone(), ((active_switches[last_index].1).2).0.clone(), ((active_switches[last_index].1).2).1)
    }
}
