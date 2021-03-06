use std::sync::{Arc, Mutex};
use std::time::Duration;
use std::thread::{self, sleep};
use std::sync::mpsc;

use logic::channel::DmxValue;
use logic::channel::DmxAddress;
use logic::fade::FadeTime;
use logic::ChannelGroupValue;

use logic::Channel;
use logic::fade::FadeCurve;

use logic::fade::get_step_number;
use logic::fade::get_fade_steps_int;
use logic::fade::try_stop_fades;

#[derive(Debug)]
/// Simple channel group to control a single channel
pub struct Single {
    /// The first channel
    pub channel1: Arc<Mutex<Channel>>,
    /// List of activated switches to activate them again in the reverse order
    pub active_switches: Vec<(usize, ChannelGroupValue)>
}

impl Single {
    /// Create an empty channel group with one channel
    pub fn new(channel1: Arc<Mutex<Channel>>) -> Single {
        Single {
            channel1: channel1,
            active_switches: Vec::new()
        }
    }

    /// A simple fade function that does not need to know the start value.
    pub fn fade_simple(&mut self, curve: FadeCurve, time: FadeTime, end_value: DmxValue, kill_others: bool) {
        let start_value = {lock!(self.channel1).get()};
        self.fade(curve, time, start_value, end_value, kill_others);
    }

    /// Fade between the current state and a given state defind by a curve the time to fade and the final channel values
    pub fn fade(&mut self, curve: FadeCurve, time: FadeTime, start_value: DmxValue, end_value: DmxValue, kill_others: bool) {
        let steps = get_step_number(time);
        let (tx, rx) = mpsc::channel();
        let channel1 = self.channel1.clone();

        if try_stop_fades(vec!(&self.channel1), tx, kill_others) {
            thread::spawn(move || {

                for value in get_fade_steps_int(start_value, end_value, steps, curve) {
                    {
                        if rx.try_recv().is_ok() { return }
                        let mut channel1_locked = lock!(channel1);

                        channel1_locked.set(value);
                    }
                    sleep(Duration::from_millis((time/steps) as u64));
                }
                lock!(channel1).current_thread = None;
            });
        }
    }
    // pub fn activate_preheat(&mut self, curve: FadeCurve, time: FadeTime) {
    //     let preheat_value = {self.channel1.lock().expect("Failed to lock Arc!").preheat_value};
    //     let max_preheat_value = {self.channel1.lock().expect("Failed to lock Arc!").max_preheat_value};
    //     let value = {self.channel1.lock().expect("Failed to lock Arc!").value};
    //     if max_preheat_value > value {
    //         self.fade(curve, time, preheat_value, max_preheat_value, true, true);
    //     }
    //     else {
    //         self.channel1.lock().expect("Failed to lock Arc!").set_preheat(max_preheat_value);
    //     }
    // }
    //
    // pub fn deactivate_preheat(&mut self, curve: FadeCurve, time: FadeTime) {
    //     let preheat_value = {self.channel1.lock().expect("Failed to lock Arc!").preheat_value};
    //     let value = {self.channel1.lock().expect("Failed to lock Arc!").value};
    //     if preheat_value > value {
    //         self.fade(curve, time, preheat_value, 0, true, true);
    //     }
    //     else {
    //         self.channel1.lock().expect("Failed to lock Arc!").set_preheat(0);
    //     }
    // }

    /// Get a vector of the DMX addresses used by this channel group
    pub fn get_addresses(&self) -> Vec<DmxAddress> {
        vec![
            lock!(self.channel1).address
        ]
    }
}
