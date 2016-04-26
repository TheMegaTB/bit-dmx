// use std::thread;
// use std::sync::mpsc;
//
// use FadeCurve;
// use FadeTime;
// use DmxChannel;
// use DmxValue;

use fixtures::single::*;
use fixtures::rgb::*;
use fixtures::rgba::*;
use fixtures::moving_2d::*;



#[derive(Debug)]
pub struct Fixture {
    channel_groups: Vec<ChannelGroup>
}

impl Fixture {
    pub fn new(channel_groups: Vec<ChannelGroup>) -> Fixture {
        Fixture {
            channel_groups: channel_groups
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
