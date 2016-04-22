use std::thread;
use std::sync::mpsc;

use FadeCurve;
use FadeTime;
use DmxChannel;
use DmxValue;

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

fn fade(channel: DmxChannel, curve: FadeCurve, start: DmxValue, end: DmxValue) {
    thread::spawn(move || {
        let delta = (end as i16) - (start as i16);
        let mut value = start;
        println!("start: {}, end: {}, delta: {}", start, end, delta);
        for _ in 0..delta.abs() {
            // sleep(Duration::new(0, 10000));
            value = if delta.is_negative() { value.saturating_sub(1) } else { value.saturating_add(1) };
            //TODO: Implement curve

        }
    });
}

#[derive(Debug)]
pub struct Single {
    channel: DmxChannel,
    value: DmxValue,
    dmx_tx: mpsc::Sender<(DmxChannel, DmxValue)>
}

impl Single {
    fn fade(&mut self, curve: FadeCurve, start: DmxValue, end: DmxValue, time: FadeTime) {

    }
}

#[derive(Debug)]
pub struct RGB;

#[derive(Debug)]
pub struct RGBA;


// Fixture {
//     name("rgbtest")
//     rgba(0, 1, 2, 3)
//     single(4)
//     preheat
// }
//
//
// rgbtest -> 5
