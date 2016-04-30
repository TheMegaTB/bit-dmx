use std::sync::{Arc, Mutex};
use std::time::Duration;
use std::thread::{self, sleep};
use std::sync::mpsc;

use DmxValue;
use DmxAddress;
use FadeTime;
use FADE_TICKS;
use ChannelGroupValue;

use Channel;
use FadeCurve;

use get_fade_steps_int;
use stop_fade;

#[derive(Debug)]
pub struct Single {
    pub channel1: Arc<Mutex<Channel>>,
    pub active_switches: Vec<(usize, ChannelGroupValue)>
}

impl Single {
    pub fn new(channel1: Arc<Mutex<Channel>>) -> Single {
        Single {
            channel1: channel1,
            active_switches: Vec::new()
        }
    }
    pub fn fade_simple(&mut self, curve: FadeCurve, time: FadeTime, end_value: DmxValue) {
        let start_value = {self.channel1.lock().unwrap().value};
        self.fade(curve, time, start_value, end_value, false);
    }

    pub fn fade(&mut self, curve: FadeCurve, time: FadeTime, start_value: DmxValue, end_value: DmxValue, preheat: bool) {
        let steps = time*FADE_TICKS/1000;
        let (tx, rx) = mpsc::channel();
        let channel1 = self.channel1.clone();
        stop_fade(channel1.clone(), tx.clone());
        thread::spawn(move || {

            for value in get_fade_steps_int(start_value, end_value, steps, curve) {
                {
                    if rx.try_recv().is_ok() { return }
                    let mut channel1_locked = channel1.lock().unwrap();

                    if preheat {
                            channel1_locked.set_preheat(value);
                    }
                    else {
                        channel1_locked.set(value);
                    }
                }
                sleep(Duration::from_millis((time/steps) as u64));
            }
            channel1.lock().unwrap().current_thread = None;
        });
    }

    pub fn activate_preheat(&mut self, curve: FadeCurve, time: FadeTime) {
        let preheat_value = {self.channel1.lock().unwrap().preheat_value};
        let max_preheat_value = {self.channel1.lock().unwrap().max_preheat_value};
        let value = {self.channel1.lock().unwrap().value};
        if max_preheat_value > value {
            self.fade(curve, time, preheat_value, max_preheat_value, true);
        }
        else {
            self.channel1.lock().unwrap().set_preheat(max_preheat_value);
        }
    }

    pub fn deactivate_preheat(&mut self, curve: FadeCurve, time: FadeTime) {
        let preheat_value = {self.channel1.lock().unwrap().preheat_value};
        let value = {self.channel1.lock().unwrap().value};
        if preheat_value > value {
            self.fade(curve, time, preheat_value, 0, true);
        }
        else {
            self.channel1.lock().unwrap().set_preheat(0);
        }
    }

    pub fn get_addresses(&self) -> Vec<DmxAddress> {
        vec![
            self.channel1.lock().unwrap().address
        ]
    }
}
