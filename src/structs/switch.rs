use std::collections::HashMap;

use ChannelGroupValue;
use rustc_serialize::json;

#[derive(Debug, RustcDecodable, RustcEncodable)]
pub struct JsonSwitch {
    pub channel_groups: HashMap<String, ChannelGroupValue>,
    pub switch_group: usize,
    pub dimmer_value: f64,
    pub name: String
}

#[derive(Debug, Clone)]
pub struct Switch {
    pub channel_groups: HashMap<(usize, usize), ChannelGroupValue>,
    pub switch_group: usize,
    pub dimmer_value: f64,
    name: String
}

impl Switch {
    pub fn new(name: String, channel_groups: HashMap<(usize, usize), ChannelGroupValue>, switch_group: usize) -> Switch {
        Switch {
            channel_groups: channel_groups,
            switch_group: switch_group,
            dimmer_value: 0.0,
            name: name
        }
    }

    pub fn with_json_hashmap(&self) -> JsonSwitch {
        JsonSwitch {
            channel_groups: self.channel_groups.iter().map(|(k, v)| (json::encode(k).unwrap(), v.clone())).collect::<HashMap<String, ChannelGroupValue>>(),
            switch_group: self.switch_group,
            dimmer_value: self.dimmer_value,
            name: self.name.clone()
        }
    }
}
