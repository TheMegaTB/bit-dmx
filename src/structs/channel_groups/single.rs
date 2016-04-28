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
        let start_value = {self.channel1.lock().unwrap().value};
        self.fade(curve, time, start_value, end_value);
    }

    pub fn fade(&mut self, curve: FadeCurve, time: FadeTime, start_value: DmxValue, end_value: DmxValue) {
        let steps = time*FADE_TICKS/1000;
        for value in get_fade_steps_int(start_value, end_value, steps, curve) {
            {self.channel1.lock().unwrap().set(value);}
            sleep(Duration::from_millis((time/steps) as u64));
        }
    }

    pub fn activate_preheat(&mut self, curve: FadeCurve, time: FadeTime) {
        {self.channel1.lock().unwrap().preheating = true};
        let new_value = {self.channel1.lock().unwrap().preheat_value};
        let current_value = {self.channel1.lock().unwrap().get()};
        if new_value > current_value {
            self.fade(curve, time, current_value, new_value);
        }
        {self.channel1.lock().unwrap().preheat_state = true;}
        {self.channel1.lock().unwrap().preheating = false};
    }

    pub fn deactivate_preheat(&mut self, curve: FadeCurve, time: FadeTime) {
        {self.channel1.lock().unwrap().preheat_state = false;}
        {self.channel1.lock().unwrap().preheating = true};
        let new_value = self.channel1.lock().unwrap().value;
        let current_value = {self.channel1.lock().unwrap().get()};
        self.fade(curve, time, current_value, new_value);
        {self.channel1.lock().unwrap().preheating = false};
    }
}
