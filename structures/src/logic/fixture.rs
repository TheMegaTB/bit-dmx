use logic::ChannelGroup;

use logic::channel::DmxAddress;


#[derive(Debug, Clone, RustcDecodable, RustcEncodable)]
/// A struct to save a fixture as json encodeable.
pub struct EmptyFixture {
    /// The list of channel groups.
    pub channel_groups: Vec<(u8, Vec<DmxAddress>)>,
    /// The name of the fixture.
    pub name: String,
}

impl EmptyFixture {
    /// Generate an empty EmptyFixture.
    pub fn new() -> EmptyFixture {
        EmptyFixture {
            channel_groups: Vec::new(),
            name: String::new()
        }
    }
}

#[derive(Debug)]
/// A struct to save a fixture in the backend.
pub struct Fixture {
    /// The list of channel groups.
    pub channel_groups: Vec<ChannelGroup>,
    /// The name of the fixture.
    name: String
}

impl Fixture {
    /// Generates a fixture from the given values
    pub fn new(name: String, channel_groups: Vec<ChannelGroup>) -> Fixture {
        Fixture {
            channel_groups: channel_groups,
            name: name
        }
    }
    /// Return channel group as vector of channel group id and vector of dmx addresses
    pub fn channel_groups_as_id(&self) -> Vec<(u8, Vec<DmxAddress>)> {
        self.channel_groups.iter().map(|x| channel_group_to_id(x)).collect()
    }
    /// Convert a Fixture to a json encodeable EmptyFixture
    pub fn to_empty_fixture(&self) -> EmptyFixture {
        EmptyFixture {
            channel_groups: self.channel_groups_as_id(),
            name: self.name.clone()
        }
    }
}

/// converts a channel group to a id.
fn channel_group_to_id(c: &ChannelGroup) -> (u8, Vec<DmxAddress>) {
    match c {
        &ChannelGroup::Single(ref group) => (0, group.get_addresses()),
        &ChannelGroup::RGB(ref group) => (1, group.get_addresses()),
        &ChannelGroup::RGBA(ref group) => (2, group.get_addresses()),
        &ChannelGroup::Moving2D(ref group) => (3, group.get_addresses()),
        &ChannelGroup::Moving2D16(ref group) => (4, group.get_addresses())
    }
}
