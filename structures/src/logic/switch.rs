use std::collections::HashMap;

use logic::ChannelGroupValue;
use rustc_serialize::json;
use logic::fade::FadeTime;
use piston_window::keyboard::Key;


#[derive(Debug, Clone, RustcDecodable, RustcEncodable)]
/// The version of a switch that is encodable for json
pub struct JsonSwitch {
    /// The list of channel groups with values in this switch.
    pub channel_groups: HashMap<String, ChannelGroupValue>,
    /// The name of the chaser.
    pub chaser_name: String,
    /// The current dimmer value of this switch. If the switch is disabled this value is 0.
    pub dimmer_value: f64,
    /// The time this switch is activated while the chaser is running.
    pub before_chaser: FadeTime,
    /// The name of the switch.
    pub name: String,
    /// The keybinding for the frontend.
    pub keybinding: Option<Key>
}

impl JsonSwitch {
    /// Generate a Json switch with default values.
    pub fn new(name: String, chaser_name: String) -> JsonSwitch {
        JsonSwitch {
            channel_groups: HashMap::new(),
            chaser_name: chaser_name,
            dimmer_value: 0.0,
            before_chaser: 500,
            name: name,
            keybinding: None
        }
    }
    /// Get the keybinding as String to display it.
    pub fn get_keybinding_as_text(&self) -> Option<String> {
        match self.keybinding {
            Some(keybinding) => Some(format!("{:?}", keybinding)),
            None => None
        }
    }
}

#[derive(Debug, Clone)]
/// The representation of a Switch in the backend.
pub struct Switch {
    /// The list of channel groups with values in this switch.
    pub channel_groups: HashMap<(usize, usize), ChannelGroupValue>,
    /// The name of the chaser.
    pub chaser_name: String,
    /// The current dimmer value of this switch. If the switch is disabled this value is 0.
    pub dimmer_value: f64,
    /// The time this switch is activated while the chaser is running.
    pub before_chaser: FadeTime,
    /// The name of the switch.
    name: String,
    /// The keybinding for the frontend.
    keybinding: Option<Key>
}

impl Switch {
    /// Generate a Switch from the given information.
    pub fn new(name: String, channel_groups: HashMap<(usize, usize), ChannelGroupValue>, chaser_name: String, before_chaser: FadeTime) -> Switch {
        Switch {
            channel_groups: channel_groups,
            chaser_name: chaser_name,
            dimmer_value: 0.0,
            before_chaser: before_chaser,
            name: name,
            keybinding: None
        }
    }

    /// Convert Switch to JsonSwitch
    pub fn with_json_hashmap(&self) -> JsonSwitch {
        JsonSwitch {
            channel_groups: self.channel_groups.iter().map(|(k, v)| (json::encode(k).unwrap(), v.clone())).collect::<HashMap<String, ChannelGroupValue>>(),
            chaser_name: self.chaser_name.clone(),
            dimmer_value: self.dimmer_value,
            before_chaser: self.before_chaser,
            name: self.name.clone(),
            keybinding: self.keybinding.clone()
        }
    }

    /// Convert JsonSwitch to Switch
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
            chaser_name: json_switch.chaser_name.clone(),
            dimmer_value: json_switch.dimmer_value,
            before_chaser: json_switch.before_chaser,
            name: json_switch.name.clone(),
            keybinding: json_switch.keybinding.clone()
        }
    }
}
