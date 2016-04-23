use std::thread;
use std::sync::mpsc;
use std::cmp;
use std::num;

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


fn max3(a: f64, b: f64, c: f64) -> f64 {
    a.max(b).max(c)
}

fn min3(a: f64, b: f64, c: f64) -> f64 {
    a.min(b).min(c)
}


fn rgb_to_hsv(r: DmxValue, g: DmxValue, b: DmxValue) -> (f64, f64, f64) {
    let r2 = r as f64/255f64;
    let g2 = g as f64/255f64;
    let b2 = b as f64/255f64;

    let cmax = max3(r2, g2, b2);
    let cmin = min3(r2, g2, b2);
    let delta = cmax-cmin;

    let h = if delta == 0f64 {
        0f64
    }
    else if cmax == r2 {
        60f64 * ((g2-b2)/delta % 6f64)
    }
    else if cmax == g2 {
        60f64 * ((b2-r2)/delta + 2f64)
    }
    else {
        60f64 * ((r2-g2)/delta + 4f64)
    };

    let s = if cmax == 0f64 {
        0f64
    }
    else {
        delta/cmax
    };

    (h, s, cmax)
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
