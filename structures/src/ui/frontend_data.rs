use std::collections::HashMap;
use rustc_serialize::json;

use logic::channel::DmxAddress;
use logic::channel::DmxValue;

use logic::fixture::EmptyFixture;

use logic::ChannelGroupValue;
use logic::fade::FadeCurve;
use logic::switch::JsonSwitch;

use logic::chaser::FrontendChaser;

#[derive(Debug, Clone, RustcDecodable, RustcEncodable)]
/// The struct to save all necessary information for the frontend about the stage
pub struct FrontendData {
    /// The name of the current project
    pub name: String,
    /// The the number of dmx addresses used in the project
    pub max_dmx_address: DmxAddress,
    /// The list of all fixtures
    pub fixtures: Vec<EmptyFixture>,
    /// The list of all switches that are configured for this project
    pub switches: Vec<JsonSwitch>,
    /// The list of all chasers. The id is a json string.
    pub chasers: HashMap<String, FrontendChaser>
}

impl FrontendData {
    /// Creates an empty frontend data set
    pub fn new(name: String) -> FrontendData {
        FrontendData {
            name: name,
            max_dmx_address: 0,
            fixtures: Vec::new(),
            switches: Vec::new(),
            chasers: HashMap::new()
        }
    }
    /// Reads frontend data from a json string
    pub fn from_json(json: String) -> Result<FrontendData, json::DecoderError> {
        json::decode(&json)
    }
    /// Encodes the data to a json string
    pub fn get_json_string(&self) -> String {
        json::encode(self).unwrap()
    }
    /// Creates a vector of dmx values for a channel group of a given fixture and returns it
    pub fn get_empty_data(&self, fixture_id: usize, channel_group_id: usize) -> Vec<DmxValue> {
        match self.fixtures[fixture_id].channel_groups[channel_group_id].0 { //ids are defind in fixtures.rs::55
            0 => vec!(0),
            1 => vec!(0, 0, 0),
            2 => vec!(0, 0, 0, 0),
            3 => vec!(128, 128),
            _ => vec!()
        }
    }
    /// Change the channel group of a channel group in a switch and sets default data for the new channel group. This function returns false, if the requested channelgroup does not exists.
    pub fn change_channel_group(&mut self, switch_id: usize, old_id: String, new_fixture_id: usize, new_channel_group_id: usize) -> bool {
        let new_id = json::encode(&(new_fixture_id, new_channel_group_id)).unwrap();
        if !self.switches[switch_id].channel_groups.contains_key(&new_id) {
            trace!("{:?}", self.switches[switch_id].channel_groups);
            trace!("{:?} -> {:?}", old_id, new_id);
            let new_values = self.get_empty_data(new_fixture_id, new_channel_group_id);
            let mut new_data = self.switches[switch_id].channel_groups.get(&old_id).unwrap().clone();
            new_data.values = new_values;
            self.switches[switch_id].channel_groups.remove(&old_id);
            self.switches[switch_id].channel_groups.insert(new_id, new_data);
            trace!("{:?}", self.switches[switch_id].channel_groups);
            true
        }
        else {
            false
        }
    }
    /// Removes a channel group from a switch
    pub fn remove_channel_group(&mut self, switch_id: usize, old_id: String) {
        self.switches[switch_id].channel_groups.remove(&old_id);
    }
    /// Adds a channel group to a switch
    pub fn add_channel_group(&mut self, switch_id: usize) {
        let mut new_id = None;
        'outer: for (fixture_index, fixture) in self.fixtures.iter().enumerate() {
            for (channel_group_index, _) in fixture.channel_groups.iter().enumerate() {
                let tmp_id = json::encode(&(fixture_index, channel_group_index)).unwrap();
                if !self.switches[switch_id].channel_groups.contains_key(&tmp_id) {
                    new_id = Some((tmp_id, self.get_empty_data(fixture_index, channel_group_index)));
                    break 'outer;
                }
            }
        }
        match new_id {
            Some((id, new_values)) => {
                self.switches[switch_id].channel_groups.insert(id.clone(), ChannelGroupValue::from_tuple((new_values, (FadeCurve::Linear, 1000), (FadeCurve::Linear, 1000))));
            },
            _ => {}
        }
    }
    /// Removes a switch from the frontend switch list and from its chaser
    pub fn remove_switch_with_id(&mut self, switch_id: usize) {
        for (_, chaser) in self.chasers.iter_mut() {
            chaser.remove_switch_with_id(switch_id);
        }
        trace!("{:?}", switch_id);
        trace!("{:?}", self.switches);
        self.switches.remove(switch_id);
    }
    /// Adds a switch to a chaser
    fn add_switch_to_chaser(&mut self, switch_id:usize, chaser_name: String) {
        self.chasers.get_mut(&chaser_name).unwrap().switches.push(switch_id);
    }

    /// Adds a switch to the frontend switch list and returns the new switch id
    pub fn add_switch(&mut self, switch: JsonSwitch) -> usize {
        let id = self.switches.len();
        self.add_switch_to_chaser(id, switch.chaser_name.clone());
        self.switches.push(switch);

        id
    }

    /// Deletes a chaser and all the switches contained in this chaser from the frontend lists
    pub fn delete_chaser(&mut self, chaser_name: String) {

        while !self.chasers.clone().get(&chaser_name).unwrap().switches.is_empty() {
            let switch_id = self.chasers.get_mut(&chaser_name).unwrap().switches[0];
            self.remove_switch_with_id(switch_id);
        }
        self.chasers.remove(&chaser_name);
    }

    /// Adds a chaser to the frontend list. This function searches for a unused chaser name of the form 'Untitled X' and returns this name
    pub fn add_chaser(&mut self) -> String {
        let mut name = "Untitled".to_string();
        let mut i = 1;
        while self.chasers.contains_key(&name.clone()) {
            i += 1;
            name = "Untitled ".to_string() + &i.to_string();
        }
        self.chasers.insert(name.clone(), FrontendChaser::new());
        name
    }

    /// Renames a chaser. THis function also adapts the link from all switches to this chaser
    pub fn rename_chaser(&mut self, old_name: String, new_name: String) -> bool {
        if !self.chasers.contains_key(&new_name) {
            let data = self.chasers.get(&old_name).unwrap().clone();
            for &index in data.switches.iter() {
                self.switches[index].chaser_name = new_name.clone();
            }
            self.chasers.insert(new_name, data);
            self.chasers.remove(&old_name);
            true
        }
        else {
            false
        }
    }
}
