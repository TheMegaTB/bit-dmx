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



#[derive(Debug)]
pub struct Fixture {
    channel_groups: Vec<ChannelGroup>
}

#[derive(Debug)]
pub enum ChannelGroup {
    Single(Single),
    RGB(RGB),
    RGBA(RGBA)
}
