use std::sync::{Arc, Mutex};
use std::sync::mpsc;

use DmxAddress;
use DmxValue;
use FadeTime;

use Fixture;
use ChannelGroup;
use Channel;

use ValueCollection;
use ChannelGroupValue;
use FadeCurve;



#[derive(Debug)]
pub struct Stage {
    pub channels: Vec<Arc<Mutex<Channel>>>,
    pub fixtures: Vec<Fixture>,
    switches: Vec<ValueCollection>,
    dmx_tx: mpsc::Sender<(DmxAddress, DmxValue)>
}

impl Stage {
    pub fn new(dmx_tx: mpsc::Sender<(DmxAddress, DmxValue)>) -> Stage {
        Stage {
            channels: Vec::new(),
            fixtures: Vec::new(),
            switches: Vec::new(),
            dmx_tx: dmx_tx
        }
    }
    pub fn add_fixture(&mut self, fixture: Fixture) -> usize {
        self.fixtures.push(fixture);
        self.fixtures.len() - 1
    }

    pub fn add_switch(&mut self, switch: ValueCollection) -> usize {
        self.switches.push(switch);
        self.switches.len() - 1
    }

    pub fn activate_switch(&mut self, switch_id: usize, dimmer_value: f64) {
        for (&(fixture_id, channel_group_id), &(ref values, ref curve, time)) in self.switches[switch_id].channel_groups.iter() {
            match self.fixtures[fixture_id].channel_groups[channel_group_id] {
                ChannelGroup::Single(ref mut group) => {
                    group.active_value_collections.push((switch_id, (values.iter().map(|a| (*a as f64 * (dimmer_value / 255.0)) as DmxValue).collect(), curve.clone(), time)));
                    group.fade_simple(curve.clone(), time, values[0]);
                },
                ChannelGroup::RGB(ref mut group) => {
                    group.active_value_collections.push((switch_id, (values.iter().map(|a| (*a as f64 * (dimmer_value / 255.0)) as DmxValue).collect(), curve.clone(), time)));
                    group.fade_simple(curve.clone(), time, values[0], values[1], values[2]);
                },
                ChannelGroup::RGBA(ref mut group) => {
                    group.active_value_collections.push((switch_id, (values.iter().map(|a| (*a as f64 * (dimmer_value / 255.0)) as DmxValue).collect(), curve.clone(), time)));
                    group.fade_simple(curve.clone(), time, values[0], values[1], values[2], values[3]);
                },
                ChannelGroup::Moving2D(ref mut group) => {
                    group.active_value_collections.push((switch_id, (values.iter().map(|a| (*a as f64 * (dimmer_value / 255.0)) as DmxValue).collect(), curve.clone(), time)));
                    group.fade_simple(curve.clone(), time, values[0], values[1]);
                }
            }
        }
    }

    pub fn deactivate_switch(&mut self, switch_id: usize) {
        for (&(fixture_id, channel_group_id), &(_, ref curve, time)) in self.switches[switch_id].channel_groups.iter() {
            match self.fixtures[fixture_id].channel_groups[channel_group_id] {
                ChannelGroup::Single(ref mut group) => {
                    if remove_from_value_collections(&mut group.active_value_collections, switch_id) {
                        let (new_values, new_curve, new_time) = extract_new_values(&mut group.active_value_collections, vec![0], curve.clone(), time);
                        group.fade_simple(new_curve, new_time, new_values[0]);
                    }
                },
                ChannelGroup::RGB(ref mut group) => {
                    if remove_from_value_collections(&mut group.active_value_collections, switch_id) {
                        let (new_values, new_curve, new_time) = extract_new_values(&mut group.active_value_collections, vec![0, 0, 0], curve.clone(), time);
                        group.fade_simple(new_curve, new_time, new_values[0], new_values[1], new_values[2]);
                    }
                },
                ChannelGroup::RGBA(ref mut group) => {
                    if remove_from_value_collections(&mut group.active_value_collections, switch_id) {
                        let (new_values, new_curve, new_time) = extract_new_values(&mut group.active_value_collections, vec![0, 0, 0, 0], curve.clone(), time);
                        group.fade_simple(new_curve, new_time, new_values[0], new_values[1], new_values[2], new_values[3]);
                    }
                },
                ChannelGroup::Moving2D(ref mut group) => {
                    if remove_from_value_collections(&mut group.active_value_collections, switch_id) {
                        let (new_values, new_curve, new_time) = extract_new_values(&mut group.active_value_collections, vec![0, 0], curve.clone(), time);
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

fn remove_from_value_collections(active_value_collections: &mut Vec<(usize, ChannelGroupValue)>, switch_id: usize) -> bool {
    if active_value_collections.len() > 0 { //TODO: Replace this workaround.
        let last_index = active_value_collections.len() - 1;
        let last_id = active_value_collections[last_index].0;
        active_value_collections.retain(|&(x, _)| x != switch_id);
        last_id == switch_id
    } else { false }
}

fn extract_new_values(active_value_collections: &mut Vec<(usize, ChannelGroupValue)>, default_values: Vec<DmxValue>, old_curve: FadeCurve, old_time: FadeTime) -> (Vec<DmxValue>, FadeCurve, FadeTime) {
    if active_value_collections.len() == 0 {
        (default_values, old_curve, old_time)
    }
    else {
        let last_index = active_value_collections.len() - 1;
        ((active_value_collections[last_index].1).0.clone(), (active_value_collections[last_index].1).1.clone(), (active_value_collections[last_index].1).2)
    }
}
