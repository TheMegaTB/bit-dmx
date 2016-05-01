use std::collections::HashMap;

use ChannelGroupValue;
use rustc_serialize::json;
use FadeTime;

#[derive(Debug, RustcDecodable, RustcEncodable)]
pub struct JsonSwitch {
    pub channel_groups: HashMap<String, ChannelGroupValue>,
    pub chaser_id: String,
    pub dimmer_value: f64,
    pub before_chaser: FadeTime,
    pub name: String
}

#[derive(Debug, Clone)]
pub struct Switch {
    pub channel_groups: HashMap<(usize, usize), ChannelGroupValue>,
    pub chaser_id: String,
    pub dimmer_value: f64,
    pub before_chaser: FadeTime,
    name: String
}

impl Switch {
    pub fn new(name: String, channel_groups: HashMap<(usize, usize), ChannelGroupValue>, chaser_id: String, before_chaser: FadeTime) -> Switch {
        Switch {
            channel_groups: channel_groups,
            chaser_id: chaser_id,
            dimmer_value: 0.0,
            before_chaser: before_chaser,
            name: name
        }
    }

    pub fn with_json_hashmap(&self) -> JsonSwitch {
        JsonSwitch {
            channel_groups: self.channel_groups.iter().map(|(k, v)| (json::encode(k).unwrap(), v.clone())).collect::<HashMap<String, ChannelGroupValue>>(),
            chaser_id: self.chaser_id.clone(),
            dimmer_value: self.dimmer_value,
            before_chaser: self.before_chaser,
            name: self.name.clone()
        }
    }
}
