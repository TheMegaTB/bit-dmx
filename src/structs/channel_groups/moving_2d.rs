use std::sync::{Arc, Mutex};
use std::time::Duration;
use std::thread::{self, sleep};

use DmxValue;
use FadeTime;
use FADE_TICKS;
use ChannelGroupValue;

use Channel;
use FadeCurve;

use get_fade_steps_int;

#[derive(Debug)]
pub struct Moving2D {
    channel_x: Arc<Mutex<Channel>>,
    channel_y: Arc<Mutex<Channel>>,
    pub active_value_collections: Vec<(usize, ChannelGroupValue)>
}

impl Moving2D {
    pub fn new(channel_x: Arc<Mutex<Channel>>, channel_y: Arc<Mutex<Channel>>) -> Moving2D {
        Moving2D {
            channel_x: channel_x,
            channel_y: channel_y,
            active_value_collections: Vec::new()
        }
    }

    pub fn fade_simple(&mut self, curve: FadeCurve, time: FadeTime, end_x: DmxValue, end_y: DmxValue) {
        let steps = time*FADE_TICKS/1000;
        let channel_x = self.channel_x.clone();
        let channel_y = self.channel_y.clone();
        thread::spawn(move || {
            let mut channel_x_locked = channel_x.lock().unwrap();
            let mut channel_y_locked = channel_y.lock().unwrap();

            for (&x, &y) in get_fade_steps_int(channel_x_locked.get(), end_x, steps, curve.clone()).iter().zip(get_fade_steps_int(channel_y_locked.get(), end_y, steps, curve.clone()).iter()) {
                channel_x_locked.set(x);
                channel_y_locked.set(y);
                sleep(Duration::from_millis((time/steps) as u64));
            }
        });
    }
}
