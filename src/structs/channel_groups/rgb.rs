use std::sync::{Arc, Mutex};
use std::time::Duration;
use std::thread::sleep;

use DmxValue;
use FadeTime;
use FADE_TICKS;

use Channel;
use FadeCurve;

use rgb_to_hsv;
use hsv_to_rgb;

use get_fade_steps;
use get_fade_steps_int;

#[derive(Debug)]
pub struct RGB {
    channel_r: Arc<Mutex<Channel>>,
    channel_g: Arc<Mutex<Channel>>,
    channel_b: Arc<Mutex<Channel>>
}

impl RGB {
    pub fn new(channel_r: Arc<Mutex<Channel>>, channel_g: Arc<Mutex<Channel>>, channel_b: Arc<Mutex<Channel>>) -> RGB {
        RGB {
            channel_r: channel_r,
            channel_g: channel_g,
            channel_b: channel_b
        }
    }

    pub fn fade_rgb(&mut self, curve: FadeCurve, time: FadeTime, end_r: DmxValue, end_g: DmxValue, end_b: DmxValue) {
        let steps = time*FADE_TICKS/1000;
        let (start_h, start_s, start_v);
        {
            let tuple = rgb_to_hsv(self.channel_r.lock().unwrap().get(), self.channel_g.lock().unwrap().get(), self.channel_b.lock().unwrap().get());
            start_h = tuple.0;
            start_s = tuple.1;
            start_v = tuple.2;
        }
        let (end_h, end_s, end_v) = rgb_to_hsv(end_r, end_g, end_b);
        for ((&h, &s), &v) in get_fade_steps(start_h, end_h, steps, curve.clone()).iter().zip(get_fade_steps(start_s, end_s, steps, curve.clone()).iter()).zip(get_fade_steps(start_v, end_v, steps, curve.clone()).iter()) {
            let (r, g, b) = hsv_to_rgb(h, s, v);

            {self.channel_r.lock().unwrap().set(r);}
            {self.channel_g.lock().unwrap().set(g);}
            {self.channel_b.lock().unwrap().set(b);}
            sleep(Duration::from_millis((time/steps) as u64));
        }
    }

    pub fn fade_simple(&mut self, curve: FadeCurve, time: FadeTime, end_r: DmxValue, end_g: DmxValue, end_b: DmxValue) {
        let steps = time*FADE_TICKS/1000;
        let (start_r, start_g, start_b);
        {start_r = self.channel_r.lock().unwrap().get();}
        {start_g = self.channel_g.lock().unwrap().get();}
        {start_b = self.channel_b.lock().unwrap().get();}
        for ((&r, &g), &b) in get_fade_steps_int(start_r, end_r, steps, curve.clone()).iter().zip(get_fade_steps_int(start_g, end_g, steps, curve.clone()).iter()).zip(get_fade_steps_int(start_b, end_b, steps, curve.clone()).iter()) {
            {self.channel_r.lock().unwrap().set(r);}
            {self.channel_g.lock().unwrap().set(g);}
            {self.channel_b.lock().unwrap().set(b);}
            sleep(Duration::from_millis((time/steps) as u64));
        }
    }
}
