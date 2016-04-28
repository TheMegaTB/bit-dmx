use std::sync::{Arc, Mutex};
use std::time::Duration;
use std::thread::{self, sleep};

use DmxValue;
use FadeTime;
use FADE_TICKS;

use Channel;
use FadeCurve;
use ChannelGroupValue;

use rgb_to_hsv;
use hsv_to_rgb;

use get_fade_steps;
use get_fade_steps_int;

#[derive(Debug)]
pub struct RGBA{
    channel_r: Arc<Mutex<Channel>>,
    channel_g: Arc<Mutex<Channel>>,
    channel_b: Arc<Mutex<Channel>>,
    channel_a: Arc<Mutex<Channel>>,
    pub active_value_collections: Vec<(usize, ChannelGroupValue)>
}

impl RGBA {
    pub fn new(channel_r: Arc<Mutex<Channel>>, channel_g: Arc<Mutex<Channel>>, channel_b: Arc<Mutex<Channel>>, channel_a: Arc<Mutex<Channel>>) -> RGBA {
        RGBA{
            channel_r: channel_r,
            channel_g: channel_g,
            channel_b: channel_b,
            channel_a: channel_a,
            active_value_collections: Vec::new()
        }
    }

    pub fn fade_rgb(&mut self, curve: FadeCurve, time: FadeTime, end_r: DmxValue, end_g: DmxValue, end_b: DmxValue, end_a: DmxValue) {
        let steps = time*FADE_TICKS/1000;
        let channel_r = self.channel_r.clone();
        let channel_g = self.channel_g.clone();
        let channel_b = self.channel_b.clone();
        let channel_a = self.channel_a.clone();
        thread::spawn(move || {
            let mut channel_r_locked = channel_r.lock().unwrap();
            let mut channel_g_locked = channel_g.lock().unwrap();
            let mut channel_b_locked = channel_b.lock().unwrap();
            let mut channel_a_locked = channel_a.lock().unwrap();
            let (start_h, start_s, start_v) = rgb_to_hsv(channel_r_locked.get(), channel_g_locked.get(), channel_b_locked.get());
            let start_a = channel_a_locked.get();
            let (end_h, end_s, end_v) = rgb_to_hsv(end_r, end_g, end_b);
            for (((&h, &s), &v), &a) in get_fade_steps(start_h, end_h, steps, curve.clone()).iter().zip(get_fade_steps(start_s, end_s, steps, curve.clone()).iter()).zip(get_fade_steps(start_v, end_v, steps, curve.clone()).iter()).zip(get_fade_steps_int(start_a, end_a, steps, curve.clone()).iter()) {
                let (r, g, b) = hsv_to_rgb(h, s, v);
                channel_r_locked.set(r);
                channel_g_locked.set(g);
                channel_b_locked.set(b);
                channel_a_locked.set(a);
                sleep(Duration::from_millis((time/steps) as u64));
            }
        });
    }

    pub fn fade_simple(&mut self, curve: FadeCurve, time: FadeTime, end_r: DmxValue, end_g: DmxValue, end_b: DmxValue, end_a: DmxValue) {
        let steps = time*FADE_TICKS/1000;
        let channel_r = self.channel_r.clone();
        let channel_g = self.channel_g.clone();
        let channel_b = self.channel_b.clone();
        let channel_a = self.channel_a.clone();

        thread::spawn(move || {
            let mut channel_r_locked = channel_r.lock().unwrap();
            let mut channel_g_locked = channel_g.lock().unwrap();
            let mut channel_b_locked = channel_b.lock().unwrap();
            let mut channel_a_locked = channel_a.lock().unwrap();
            for (((&r, &g), &b), &a) in get_fade_steps_int(channel_r_locked.get(), end_r, steps, curve.clone()).iter().zip(get_fade_steps_int(channel_g_locked.get(), end_g, steps, curve.clone()).iter()).zip(get_fade_steps_int(channel_b_locked.get(), end_b, steps, curve.clone()).iter()).zip(get_fade_steps_int(channel_a_locked.get(), end_a, steps, curve.clone()).iter()) {
                channel_r_locked.set(r);
                channel_g_locked.set(g);
                channel_b_locked.set(b);
                channel_a_locked.set(a);
                sleep(Duration::from_millis((time/steps) as u64));
            }
        });
    }
}
