use logic::ChannelGroup;

use logic::channel::DmxAddress;


#[derive(Debug, Clone, RustcDecodable, RustcEncodable)]
pub struct EmptyFixture {
    pub channel_groups: Vec<(u8, Vec<DmxAddress>)>,
    pub name: String,
}

impl EmptyFixture {
    pub fn new() -> EmptyFixture {
        EmptyFixture {
            channel_groups: Vec::new(),
            name: String::new()
        }
    }
}

#[derive(Debug)]
pub struct Fixture {
    pub channel_groups: Vec<ChannelGroup>,
    name: String
}

impl Fixture {
    pub fn new(name: String, channel_groups: Vec<ChannelGroup>) -> Fixture {
        Fixture {
            channel_groups: channel_groups,
            name: name
        }
    }

    pub fn channel_groups_as_id(&self) -> Vec<(u8, Vec<DmxAddress>)> {
        self.channel_groups.iter().map(|x| channel_group_to_id(x)).collect()
    }

    pub fn to_empty_fixture(&self) -> EmptyFixture {
        EmptyFixture {
            channel_groups: self.channel_groups_as_id(),
            name: self.name.clone()
        }
    }
}

fn channel_group_to_id(c: &ChannelGroup) -> (u8, Vec<DmxAddress>) {
    match c {
        &ChannelGroup::Single(ref group) => (0, group.get_addresses()),
        &ChannelGroup::RGB(ref group) => (1, group.get_addresses()),
        &ChannelGroup::RGBA(ref group) => (2, group.get_addresses()),
        &ChannelGroup::Moving2D(ref group) => (3, group.get_addresses())
    }
}
