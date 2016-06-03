use DmxValue;
use FadeCurve;
use Channel;
use std::sync::mpsc;
use std::sync::{Arc, Mutex};

use FadeTime;
use FADE_TICKS;

#[macro_export]
macro_rules! exit {
    () => {exit!(1)};
    ($code:expr) => {
        // TODO Save all that important work
        ::std::process::exit($code);
    };
    ($code:expr, $res:expr) => {
        error!("{}", $res);
        exit!($code);
    };
    ($code:expr, $res:expr, $($arg:tt)*) => {
        exit!($code, format!($res, $($arg)*));
    };
}

#[macro_export]
macro_rules! hashmap {
    ($( $key: expr => $val: expr ),*) => {{
         let mut map = ::std::collections::HashMap::new();
         $( map.insert($key, $val); )*
         map
    }}
}

fn max3(a: f64, b: f64, c: f64) -> f64 {
    a.max(b).max(c)
}

fn min3(a: f64, b: f64, c: f64) -> f64 {
    a.min(b).min(c)
}

pub fn rgb_to_hsv(r: DmxValue, g: DmxValue, b: DmxValue) -> (f64, f64, f64) {
    let r2 = r as f64/255f64;
    let g2 = g as f64/255f64;
    let b2 = b as f64/255f64;

    let cmax = max3(r2, g2, b2);
    let cmin = min3(r2, g2, b2);
    let delta = cmax-cmin;

    let h = if delta == 0f64 {
        0f64
    }
    else if cmax == r2 {
        60f64 * ((g2-b2)/delta % 6f64)
    }
    else if cmax == g2 {
        60f64 * ((b2-r2)/delta + 2f64)
    }
    else {
        60f64 * ((r2-g2)/delta + 4f64)
    };

    let s = if cmax == 0f64 {
        0f64
    }
    else {
        delta/cmax
    };

    (h, s, cmax)
}

pub fn hsv_to_rgb(h: f64, s: f64, v: f64) -> (DmxValue, DmxValue, DmxValue) {
    let c = v * s;
    let x = c * (1f64 - ((h/60f64) % 2f64 - 1f64).abs());
    let m = v - c;
    let (r2, g2, b2) = if h < 60f64 {
        (c, x, 0f64)
    }
    else if h < 120f64 {
        (x, c, 0f64)
    }
    else if h < 180f64 {
        (0f64, c, x)
    }
    else if h < 240f64 {
        (0f64, x, c)
    }
    else if h < 300f64 {
        (x, 0f64, c)
    }
    else {
        (c, 0f64, x)
    };
    (((r2+m)*255f64) as DmxValue, ((g2+m)*255f64) as DmxValue, ((b2+m)*255f64) as DmxValue)
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
