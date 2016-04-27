use std::sync::{Arc, Mutex};
use std::time::Duration;
use std::thread::sleep;

use DmxValue;
use FadeTime;
use FADE_TICKS;

use Channel;
use FadeCurve;

use get_fade_steps_int;

#[derive(Debug)]
pub struct Moving2D {
    channel_x: Arc<Mutex<Channel>>,
    channel_y: Arc<Mutex<Channel>>
}

impl Moving2D {
    pub fn new(channel_x: Arc<Mutex<Channel>>, channel_y: Arc<Mutex<Channel>>) -> Moving2D {
        Moving2D {
            channel_x: channel_x,
            channel_y: channel_y
        }
    }

    pub fn fade_simple(&mut self, curve: FadeCurve, time: FadeTime, end_x: DmxValue, end_y: DmxValue) {
        let steps = time*FADE_TICKS/1000;
        let (start_x, start_y);
        {start_x = self.channel_x.lock().unwrap().get()}
        {start_y = self.channel_y.lock().unwrap().get()}
        for (&x, &y) in get_fade_steps_int(start_x, end_x, steps, curve.clone()).iter().zip(get_fade_steps_int(start_y, end_y, steps, curve.clone()).iter()) {
            {self.channel_x.lock().unwrap().set(x);}
            {self.channel_y.lock().unwrap().set(y);}
            sleep(Duration::from_millis((time/steps) as u64));
        }
    }
}
