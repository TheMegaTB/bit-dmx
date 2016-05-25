use std::collections::HashMap;

use ChannelGroupValue;
use rustc_serialize::json;
use FadeTime;

#[derive(Debug, Clone, RustcDecodable, RustcEncodable)]
pub struct JsonSwitch {
    pub channel_groups: HashMap<String, ChannelGroupValue>,
    pub chaser_id: String,
    pub dimmer_value: f64,
    pub before_chaser: FadeTime,
    pub name: String
}

impl JsonSwitch {
    pub fn new(name: String, chaser_id: String) -> JsonSwitch {
        JsonSwitch {
            channel_groups: HashMap::new(),
            chaser_id: chaser_id,
            dimmer_value: 255.0,
            before_chaser: 0,
            name: name
        }
    }
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

    pub fn load_from_json_switch(json_switch: JsonSwitch) -> Switch {
        Switch {
            channel_groups: json_switch.channel_groups.iter().map(|(k, v)| {
                let mut id_vector: Vec<String> = k.split(",").map(|x| x.to_string()).collect();
                id_vector[0].remove(0);
                id_vector[1].pop();
                let fixture_id = id_vector[0].parse::<usize>().unwrap();
                let channel_group_id = id_vector[1].parse::<usize>().unwrap();
                ((fixture_id, channel_group_id), v.clone())

            }).collect::<HashMap<(usize, usize), ChannelGroupValue>>(),
            chaser_id: json_switch.chaser_id.clone(),
            dimmer_value: json_switch.dimmer_value,
            before_chaser: json_switch.before_chaser,
            name: json_switch.name.clone()
        }
    }
}
