use channel_groups::single::*;
use channel_groups::rgb::*;
use channel_groups::rgba::*;
use channel_groups::moving_2d::*;



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
}

#[derive(Debug)]
pub enum ChannelGroup {
    Single(Single),
    RGB(RGB),
    RGBA(RGBA),
    Moving2D(Moving2D)
}
