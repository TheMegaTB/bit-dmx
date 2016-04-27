use DmxAddress;
use DmxValue;
use FadeCurve;
use FadeTime;
use rgb_to_hsv;
use hsv_to_rgb;
use std::sync::mpsc;
use get_fade_steps;
use get_fade_steps_int;
use FADE_TICKS;

use std::time::Duration;
use std::thread::sleep;

#[derive(Debug)]
pub struct RGBA{
    channel: DmxAddress,
    value_r: DmxValue,
    value_g: DmxValue,
    value_b: DmxValue,
    value_a: DmxValue,
    dmx_tx: mpsc::Sender<(DmxAddress, DmxValue)>
}

impl RGBA {
pub fn new(channel: DmxAddress, dmx_tx: mpsc::Sender<(DmxAddress, DmxValue)>) -> RGBA {
    RGBA{
        channel: channel,
        value_r: 0,
        value_g: 0,
        value_b: 0,
        value_a: 0,
        dmx_tx: dmx_tx
    }
}

pub fn fade_rgb(&mut self, curve: FadeCurve, time: FadeTime, end_r: DmxValue, end_g: DmxValue, end_b: DmxValue, end_a: DmxValue) {
    let (start_h, start_s, start_v) = rgb_to_hsv(self.value_r, self.value_g, self.value_b);
    let (end_h, end_s, end_v) = rgb_to_hsv(end_r, end_g, end_b);
    let steps = time*FADE_TICKS/1000;
    for (((&h, &s), &v), &a) in get_fade_steps(start_h, end_h, steps, curve.clone()).iter().zip(get_fade_steps(start_s, end_s, steps, curve.clone()).iter()).zip(get_fade_steps(start_v, end_v, steps, curve.clone()).iter()).zip(get_fade_steps_int(self.value_a, end_a, steps, curve.clone()).iter()) {
        let (r, g, b) = hsv_to_rgb(h, s, v);

        self.dmx_tx.send((self.channel + 0, r)).unwrap();
        self.dmx_tx.send((self.channel + 1, g)).unwrap();
        self.dmx_tx.send((self.channel + 2, b)).unwrap();
        self.dmx_tx.send((self.channel + 3, a)).unwrap();
        self.value_r = r;
        self.value_g = g;
        self.value_b = b;
        self.value_a = a;
        sleep(Duration::from_millis((time/steps) as u64));
    }
}

pub fn fade_simple(&mut self, curve: FadeCurve, time: FadeTime, end_r: DmxValue, end_g: DmxValue, end_b: DmxValue, end_a: DmxValue) {
    let steps = time*FADE_TICKS/1000;
    for (((&r, &g), &b), &a) in get_fade_steps_int(self.value_r, end_r, steps, curve.clone()).iter().zip(get_fade_steps_int(self.value_g, end_g, steps, curve.clone()).iter()).zip(get_fade_steps_int(self.value_b, end_b, steps, curve.clone()).iter()).zip(get_fade_steps_int(self.value_a, end_a, steps, curve.clone()).iter()) {

        self.dmx_tx.send((self.channel + 0, r)).unwrap();
        self.dmx_tx.send((self.channel + 1, g)).unwrap();
        self.dmx_tx.send((self.channel + 2, b)).unwrap();
        self.dmx_tx.send((self.channel + 3, a)).unwrap();
        self.value_r = r;
        self.value_g = g;
        self.value_b = b;
        self.value_a = a;
        sleep(Duration::from_millis((time/steps) as u64));
    }
}
}
