use meval::Expr;
use logic::channel::DmxValue;
use logic::Channel;
use std::sync::mpsc;
use std::sync::{Arc, Mutex};

/// The default time to save time
pub type FadeTime = usize;

/// The fade steps per second. In the future this will be replaced by a dynamic number
const FADE_TICKS: FadeTime = 30; //TODO replace this with a variable

#[derive(Debug, Clone)]
#[derive(RustcDecodable, RustcEncodable)]

/// An enum to describe all possible fade curves, that are suported by bit dmx
pub enum FadeCurve { //TODO add fadecurve from pointlist
    /// A simple linear curve
    Linear,
    /// A simple squared curve
    Squared,
    /// A simple square root curve
    SquareRoot,
    /// A sin curve with n hills
    Sin(u8),
    /// A custom curve represented by a String
    ///
    /// # Examples
    /// ```
    /// use structures::logic::fade::FadeCurve;

    /// let curve = FadeCurve::Custom("log(x)".to_string())
    /// ```
    Custom(String)
}

fn linear(x: f64) -> f64 {
    x
}

fn squared(x: f64) -> f64 {
    x * x
}

fn square_root(x: f64) -> f64 {
    x.sqrt()
}

impl FadeCurve {
    pub fn to_function(self) -> Box<Fn(f64) -> f64> {
        match self {
            FadeCurve::Linear => Box::new(linear),
            FadeCurve::Squared => Box::new(squared),
            FadeCurve::SquareRoot => Box::new(square_root),
            FadeCurve::Sin(i) => Expr::from_str("-cos(".to_string() + &i.to_string() + &".5*6.28318530718*x)*0.5+0.5".to_string()).unwrap().bind("x").unwrap(),
            FadeCurve::Custom(e) => Expr::from_str(e).unwrap().bind("x").unwrap()
        }
    }
    pub fn get_id(&self) -> usize {
        match *self {
            FadeCurve::Linear => 0,
            FadeCurve::Squared => 1,
            FadeCurve::SquareRoot => 2,
            FadeCurve::Custom(_) => 3,
            _ => 0
        }
    }

    pub fn get_by_id(id: usize, custom_string: String) -> FadeCurve {
        match id {
            0 => FadeCurve::Linear,
            1 => FadeCurve::Squared,
            2 => FadeCurve::SquareRoot,
            3 => FadeCurve::Custom(custom_string),
            _ => FadeCurve::Linear,
        }
    }
    pub fn get_string(&self) -> String {
        match *self {
            FadeCurve::Custom(ref e) => e.clone(),
            _ => "x".to_string()
        }
    }
}

pub fn get_step_number(time: FadeTime) -> usize {
    let steps = time*FADE_TICKS/1000;
    if steps > 0 {
        steps
    }
    else {
        1
    }
}

pub fn get_fade_steps_int(start_value: DmxValue, target_value: DmxValue, steps: usize, curve: FadeCurve) -> Vec<DmxValue> {
    get_fade_steps(start_value as f64, target_value as f64, steps, curve).iter().map(|x| *x as DmxValue).collect()
}

pub fn get_fade_steps(start_value: f64, target_value: f64, steps: usize, curve: FadeCurve) -> Vec<f64> {
    let curve_fn = &*curve.to_function();
    let y_offset = curve_fn(0f64);
    let y_scale = 1f64/(curve_fn(1f64)-y_offset);
    if target_value > start_value {
        (1..steps + 1).map(|step| (start_value + ((target_value - start_value) * curve_fn(                step as f64 /steps as f64) - y_offset) *  y_scale).max(0f64).min(255f64)).collect()
    }
    else {
        (1..steps + 1).map(|step| (target_value + ((target_value - start_value) * curve_fn((steps as f64 - step as f64)/steps as f64) - y_offset) * -y_scale).max(0f64).min(255f64)).collect()
    }
}

pub fn stop_fade(channel: &Arc<Mutex<Channel>>, tx: mpsc::Sender<()>) {
    let mut channel_locked = channel.lock().expect("Failed to lock Arc!");
    channel_locked.stop_fade();
    channel_locked.current_thread = Some(tx);
}

pub fn try_stop_fades(channels: Vec<&Arc<Mutex<Channel>>>, tx: mpsc::Sender<()>, kill_others: bool) -> bool {
    let mut channel_blocked = false;
    for channel in channels.iter() {
        let channel_locked = channel.lock().expect("Failed to lock Arc!");
        if channel_locked.current_thread.is_some() {
            channel_blocked = true;
        }
    }
    if !channel_blocked || (channel_blocked && kill_others) {
        for channel in channels.iter() {
            stop_fade(channel, tx.clone())
        }
        true
    }
    else {false}

}
