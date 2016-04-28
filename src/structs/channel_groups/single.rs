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
pub struct Single {
    pub channel1: Arc<Mutex<Channel>>,
    pub active_value_collections: Vec<(usize, ChannelGroupValue)>
}

impl Single {
    pub fn new(channel1: Arc<Mutex<Channel>>) -> Single {
        Single {
            channel1: channel1,
            active_value_collections: Vec::new()
        }
    }
    pub fn fade_simple(&mut self, curve: FadeCurve, time: FadeTime, end_value: DmxValue) {
        let start_value = {self.channel1.lock().unwrap().value};
        self.fade(curve, time, start_value, end_value, false);
    }

    pub fn fade(&mut self, curve: FadeCurve, time: FadeTime, start_value: DmxValue, end_value: DmxValue, preheat: bool) {
        let steps = time*FADE_TICKS/1000;
        let channel1 = self.channel1.clone();

        thread::spawn(move || {
            let mut channel1_locked = channel1.lock().unwrap();
            for value in get_fade_steps_int(start_value, end_value, steps, curve) {
                if preheat {
                        channel1_locked.set_preheat(value);
                }
                else {
                    channel1_locked.set(value);
                }
                sleep(Duration::from_millis((time/steps) as u64));
            }
        });
    }

    pub fn activate_preheat(&mut self, curve: FadeCurve, time: FadeTime) {
        let start_value = {self.channel1.lock().unwrap().preheat_value};
        let end_value = {self.channel1.lock().unwrap().max_preheat_value};
        self.fade(curve, time, start_value, end_value, true);



        // channel1_locked.preheat_state = 1;
        // let new_value = channel1_locked.preheat_value;
        // let current_value = channel1_locked.get();
        // println!("{:?}", current_value);
        // println!("{:?}", new_value);
        // if new_value > current_value {
        //     self.fade(curve, time, current_value, new_value);
        // }
        // sleep(Duration::from_millis(time as u64));
        // sleep(Duration::from_millis(2000));
        // println!("fade finished: {:?}: {:?}", time as u64, Duration::from_millis(time as u64));
        // channel1_locked.preheat_state = 2;
    }

    pub fn deactivate_preheat(&mut self, curve: FadeCurve, time: FadeTime) {
        let start_value = {self.channel1.lock().unwrap().preheat_value};
        let end_value = 0;
        self.fade(curve, time, start_value, end_value, true);
        // let channel1 = self.channel1.clone();
        // let mut channel1_locked = channel1.lock().unwrap();
        // channel1_locked.preheat_state = 1;
        // let new_value = channel1_locked.value;
        // let current_value = channel1_locked.get();
        // self.fade(curve, time, current_value, new_value);
        // sleep(Duration::from_millis(time as u64));
        // channel1_locked.preheat_state = 0;
    }
}
