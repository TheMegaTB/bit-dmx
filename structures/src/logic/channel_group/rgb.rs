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
/// Simple channel group to control rgb lights
pub struct RGB {
    /// The channel that is used for the red light
    channel_r: Arc<Mutex<Channel>>,
    /// The channel that is used for the green light
    channel_g: Arc<Mutex<Channel>>,
    /// The channel that is used for the blue light
    channel_b: Arc<Mutex<Channel>>,
    /// List of activated switches to activate them again in the reverse order
    pub active_switches: Vec<(usize, ChannelGroupValue)>
}

impl RGB {
    /// Create an empty rgb channel group
    pub fn new(channel_r: Arc<Mutex<Channel>>, channel_g: Arc<Mutex<Channel>>, channel_b: Arc<Mutex<Channel>>) -> RGB {
        RGB {
            channel_r: channel_r,
            channel_g: channel_g,
            channel_b: channel_b,
            active_switches: Vec::new()
        }
    }

    // pub fn fade_rgb(&mut self, curve: FadeCurve, time: FadeTime, end_r: DmxValue, end_g: DmxValue, end_b: DmxValue) {
    //     let steps = time*FADE_TICKS/1000;
    //     let (tx, rx) = mpsc::channel();
    //     let channel_r = self.channel_r.clone();
    //     let channel_g = self.channel_g.clone();
    //     let channel_b = self.channel_b.clone();
    //     stop_fade(channel_r.clone(), tx.clone());
    //     stop_fade(channel_g.clone(), tx.clone());
    //     stop_fade(channel_b.clone(), tx.clone());
    //
    //     thread::spawn(move || {
    //         let mut channel_r_locked = channel_r.lock().expect("Failed to lock Arc!");
    //         let mut channel_g_locked = channel_g.lock().expect("Failed to lock Arc!");
    //         let mut channel_b_locked = channel_b.lock().expect("Failed to lock Arc!");
    //         let (start_h, start_s, start_v) = rgb_to_hsv(channel_r_locked.get(), channel_g_locked.get(), channel_b_locked.get());
    //         let (end_h, end_s, end_v) = rgb_to_hsv(end_r, end_g, end_b);
    //         for ((&h, &s), &v) in get_fade_steps(start_h, end_h, steps, curve.clone()).iter().zip(get_fade_steps(start_s, end_s, steps, curve.clone()).iter()).zip(get_fade_steps(start_v, end_v, steps, curve.clone()).iter()) {
    //             let (r, g, b) = hsv_to_rgb(h, s, v);
    //             channel_r_locked.set(r);
    //             channel_g_locked.set(g);
    //             channel_b_locked.set(b);
    //             sleep(Duration::from_millis((time/steps) as u64));
    //         }
    //     });
    // }

    /// Fade between the current state and a given state defind by a curve the time to fade and the final channel values
    pub fn fade_simple(&mut self, curve: FadeCurve, time: FadeTime, end_r: DmxValue, end_g: DmxValue, end_b: DmxValue, kill_others: bool) {
        let steps = get_step_number(time);
        let (tx, rx) = mpsc::channel();


        if try_stop_fades(vec!(&self.channel_r, &self.channel_g, &self.channel_b), tx, kill_others) {

            let channel_r = self.channel_r.clone();
            let channel_g = self.channel_g.clone();
            let channel_b = self.channel_b.clone();


            thread::spawn(move || {
                let start_r = lock!(channel_r).get();
                let start_g = lock!(channel_g).get();
                let start_b = lock!(channel_b).get();
                for ((&r, &g), &b) in get_fade_steps_int(start_r, end_r, steps, curve.clone()).iter().zip(get_fade_steps_int(start_g, end_g, steps, curve.clone()).iter()).zip(get_fade_steps_int(start_b, end_b, steps, curve.clone()).iter()) {
                    {
                        if rx.try_recv().is_ok() { return }
                        lock!(channel_r).set(r);
                        lock!(channel_g).set(g);
                        lock!(channel_b).set(b);
                    }
                    sleep(Duration::from_millis((time/steps) as u64));
                }
            });
        }
    }

    /// Get a vector of the DMX addresses used by this channel group
    pub fn get_addresses(&self) -> Vec<DmxAddress> {
        vec![
            lock!(self.channel_r).address,
            lock!(self.channel_g).address,
            lock!(self.channel_b).address
        ]
    }
}
