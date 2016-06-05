//! Generic helper functions for all kind of tasks
use logic::channel::DmxValue;

/// Panic with a given error code and print an optional message
/// # Examples
///
/// ```should_panic
/// # #[macro_use] extern crate structures;
/// # #[macro_use] extern crate log;
/// # fn main() {
/// // An error code is required
/// exit!(1);
/// # }
/// ```
///
/// ```should_panic
/// # #[macro_use] extern crate structures;
/// # #[macro_use] extern crate log;
/// # fn main() {
/// // Additionally you can provide an error message
/// exit!(1, "Some random generic error.");
/// # }
/// ```
///
/// ```should_panic
/// # #[macro_use] extern crate structures;
/// # #[macro_use] extern crate log;
/// # fn main() {
/// // It's even possible to use format arguments
/// exit!(1, "Some random generic error. And some nice arguments are possible as well: {}", 5);
/// # }
/// ```
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

// #[macro_export]
// macro_rules! hashmap {
//     ($( $key: expr => $val: expr ),*) => {{
//          let mut map = ::std::collections::HashMap::new();
//          $( map.insert($key, $val); )*
//          map
//     }}
// }

fn max3(a: f64, b: f64, c: f64) -> f64 {
    a.max(b).max(c)
}

fn min3(a: f64, b: f64, c: f64) -> f64 {
    a.min(b).min(c)
}

/// Convert RGB values to HSV values
///
/// # Examples
/// ```
/// use structures::rgb_to_hsv;
///
/// let hsv = rgb_to_hsv(255, 100, 255);
///
/// # assert_eq!(hsv.0, -60.0);
/// # assert_eq!(hsv.1, 0.607843137254902);
/// # assert_eq!(hsv.2, 1.0);
/// ```
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

/// Convert HSV values back to RGB values
///
/// # Examples
/// ```
/// use structures::hsv_to_rgb;
///
/// let rgb = hsv_to_rgb(-60.0, 0.607843137254902, 1.0);
///
/// # assert_eq!(rgb.0, 255);
/// # assert_eq!(rgb.1, 100);
/// # assert_eq!(rgb.2, 255);
/// ```
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
