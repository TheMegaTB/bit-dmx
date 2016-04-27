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
pub struct Single {
    channel1: Arc<Mutex<Channel>>
}

impl Single {
    pub fn new(channel1: Arc<Mutex<Channel>>) -> Single {
        Single {
            channel1: channel1
        }
    }
    pub fn fade_simple(&mut self, curve: FadeCurve, time: FadeTime, end_value: DmxValue) {
        let steps = time*FADE_TICKS/1000;
        let start_value;
        {start_value = self.channel1.lock().unwrap().get();}
        for value in get_fade_steps_int(start_value, end_value, steps, curve) {
            {self.channel1.lock().unwrap().set(value);}
            sleep(Duration::from_millis((time/steps) as u64));
        }
    }

    pub fn activate_preheat(&mut self, curve: FadeCurve, time: FadeTime) {
        let new_value = self.channel1.lock().unwrap().preheat_value;
        self.fade_simple(curve, time, new_value);
        self.channel1.lock().unwrap().preheat_state = true;
    }

    pub fn deactivate_preheat(&mut self, curve: FadeCurve, time: FadeTime) {
        let new_value = self.channel1.lock().unwrap().preheat_value;
        self.channel1.lock().unwrap().preheat_state = false;
        self.fade_simple(curve, time, new_value);
    }
}
