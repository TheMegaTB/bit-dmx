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
/// The channel group to move all these awesome moving heads
pub struct Moving2D {
    /// The channel that is used for the x coordinates
    channel_x: Arc<Mutex<Channel>>,
    /// The channel that is used for the y coordinates
    channel_y: Arc<Mutex<Channel>>,
    /// List of activated switches to activate them again in the reverse order
    pub active_switches: Vec<(usize, ChannelGroupValue)>
}

impl Moving2D {
    /// Create an empty moving head channel group with 2 channels
    pub fn new(channel_x: Arc<Mutex<Channel>>, channel_y: Arc<Mutex<Channel>>) -> Moving2D {
        Moving2D {
            channel_x: channel_x,
            channel_y: channel_y,
            active_switches: Vec::new()
        }
    }

    /// fade between the current state and a given state defind by a curve the time to fade and the final channel values
    pub fn fade_simple(&mut self, curve: FadeCurve, time: FadeTime, end_x: DmxValue, end_y: DmxValue, kill_others: bool) {
        let steps = get_step_number(time);
        let (tx, rx) = mpsc::channel();

        if try_stop_fades(vec!(&self.channel_x, &self.channel_y), tx, kill_others) {

            let channel_x = self.channel_x.clone();
            let channel_y = self.channel_y.clone();

            thread::spawn(move || {
                let start_x = {channel_x.lock().expect("Failed to lock Arc!").get()};
                let start_y = {channel_y.lock().expect("Failed to lock Arc!").get()};
                for (&x, &y) in get_fade_steps_int(start_x, end_x, steps, curve.clone()).iter().zip(get_fade_steps_int(start_y, end_y, steps, curve.clone()).iter()) {
                    {
                        if rx.try_recv().is_ok() { return }
                        channel_x.lock().expect("Failed to lock Arc!").set(x);
                        channel_y.lock().expect("Failed to lock Arc!").set(y);
                    }
                    sleep(Duration::from_millis((time/steps) as u64));
                }
            });
        }
    }

    /// A function to get a vector of the DMX addresses used by this channel group
    pub fn get_addresses(&self) -> Vec<DmxAddress> {
        vec![
            self.channel_x.lock().expect("Failed to lock Arc!").address,
            self.channel_y.lock().expect("Failed to lock Arc!").address
        ]
    }
}
