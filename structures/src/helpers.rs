use logic::channel::DmxValue;

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
