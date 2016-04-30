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
pub struct Moving2D {
    channel_x: Arc<Mutex<Channel>>,
    channel_y: Arc<Mutex<Channel>>,
    pub active_switches: Vec<(usize, ChannelGroupValue)>
}

impl Moving2D {
    pub fn new(channel_x: Arc<Mutex<Channel>>, channel_y: Arc<Mutex<Channel>>) -> Moving2D {
        Moving2D {
            channel_x: channel_x,
            channel_y: channel_y,
            active_switches: Vec::new()
        }
    }

    pub fn fade_simple(&mut self, curve: FadeCurve, time: FadeTime, end_x: DmxValue, end_y: DmxValue) {
        let steps = time*FADE_TICKS/1000;
        let (tx, rx) = mpsc::channel();
        let channel_x = self.channel_x.clone();
        let channel_y = self.channel_y.clone();
        stop_fade(channel_x.clone(), tx.clone());
        stop_fade(channel_y.clone(), tx.clone());

        thread::spawn(move || {
            let start_x = {channel_x.lock().unwrap().get()};
            let start_y = {channel_y.lock().unwrap().get()};
            for (&x, &y) in get_fade_steps_int(start_x, end_x, steps, curve.clone()).iter().zip(get_fade_steps_int(start_y, end_y, steps, curve.clone()).iter()) {
                {
                    if rx.try_recv().is_ok() { return }
                    channel_x.lock().unwrap().set(x);
                    channel_y.lock().unwrap().set(y);
                }
                sleep(Duration::from_millis((time/steps) as u64));
            }
        });
    }

    pub fn get_addresses(&self) -> Vec<DmxAddress> {
        vec![
            self.channel_x.lock().unwrap().address,
            self.channel_y.lock().unwrap().address
        ]
    }
}
