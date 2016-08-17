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
/// The channel group to control all these awesome moving heads
pub struct Moving3D {
    /// The channel that is used for the first 8 bits of the x coordinate
    channel_x1: Arc<Mutex<Channel>>,
    /// The channel that is used for the last 8 bits of the x coordinate
    channel_x2: Arc<Mutex<Channel>>,
    /// The channel that is used for the first 8 bits of the y coordinate
    channel_y1: Arc<Mutex<Channel>>,
    /// The channel that is used for the last 8 bits of the y coordinate
    channel_y2: Arc<Mutex<Channel>>,
    /// List of activated switches to activate them again in the reverse order
    pub active_switches: Vec<(usize, ChannelGroupValue)>
}

impl Moving3D {
    /// Create an empty moving head channel group with 2 channels
    pub fn new(channel_x1: Arc<Mutex<Channel>>, channel_x2: Arc<Mutex<Channel>>, channel_y1: Arc<Mutex<Channel>>, channel_y2: Arc<Mutex<Channel>>) -> Moving3D {
        Moving3D {
            channel_x1: channel_x1,
            channel_x2: channel_x2
            channel_y1: channel_y1,
            channel_y2: channel_y2,
            active_switches: Vec::new()
        }
    }

    /// Fade between the current state and a given state defind by a curve the time to fade and the final channel values
    pub fn fade_simple(&mut self, curve: FadeCurve, time: FadeTime, end_x1: DmxValue, end_x2: DmxValue, end_y1: DmxValue, end_y2: DmxValue, kill_others: bool) {
        let steps = get_step_number(time);
        let (tx, rx) = mpsc::channel();

        if try_stop_fades(vec!(&self.channel_x1, &self.channel_x2, &self.channel_y1, &self.channel_y2), tx, kill_others) {

            let channel_x1 = self.channel_x1.clone();
            let channel_x2 = self.channel_x2.clone();
            let channel_y1 = self.channel_y1.clone();
            let channel_y2 = self.channel_y2.clone();

            thread::spawn(move || {
                let start_x1 = {lock!(channel_x1).get()};
                let start_x2 = {lock!(channel_x2).get()};
                let start_y1 = {lock!(channel_y1).get()};
                let start_y2 = {lock!(channel_y2).get()};
                for (&x1, &y1, &x2, &y2) in get_fade_steps_int(start_x, end_x, steps, curve.clone()).iter().zip(get_fade_steps_int(start_y, end_y, steps, curve.clone()).iter())
                    .zip(get_fade_steps_int(start_x2, end_x2, steps, curve.clone()).iter()).zip(get_fade_steps_int(start_y2, end_y2, steps, curve.clone())) {
                    {
                        if rx.try_recv().is_ok() { return }
                        lock!(channel_x1).set(x1);
                        lock!(channel_x2).set(x2);
                        lock!(channel_y1).set(y1);
                        lock!(channel_y2).set(y2);
                    }
                    sleep(Duration::from_millis((time/steps) as u64));
                }
            });
        }
    }

    pub fn fade_16bit(&mut self, curve: FadeCurve, time: FadeTime, end_x: DmxValue16, end_y: DmxValue16, kill_others: bool) {

    }

    /// Get a vector of the DMX addresses used by this channel group
    pub fn get_addresses(&self) -> Vec<DmxAddress> {
        vec![
            lock!(self.channel_x1).address,
            lock!(self.channel_x2).address,
            lock!(self.channel_y1).address,
            lock!(self.channel_y2).address
        ]
    }
}
