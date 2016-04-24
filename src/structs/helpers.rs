use DmxValue;
use FadeCurve;


fn max3(a: f64, b: f64, c: f64) -> f64 {
    a.max(b).max(c)
}

fn min3(a: f64, b: f64, c: f64) -> f64 {
    a.min(b).min(c)
}

#[allow(dead_code)]
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

#[allow(dead_code)]
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


pub fn get_fade_steps(start_value: DmxValue, target_value: DmxValue, steps: usize, curve: FadeCurve) -> Vec<DmxValue> {
    let curve_fn = &*curve.to_function();
    let y_offset = curve_fn(0f64);
    let y_scale = 1f64/(curve_fn(1f64)-y_offset);
    // for a in (1..steps + 1).map(|step| (start_value as f64 + ((target_value as f64 - start_value as f64)  * curve_fn(step as f64/steps as f64) - y_offset) * y_scale).max(0f64).min(255f64)) {
    //     println!("nv: {:?}", a);
    // }
    // println!("fade from {:?}", start_value);
    // println!("fade to {:?}", target_value);
    // println!("y_scale {:?}", y_scale);

    (1..steps + 1).map(|step| (start_value as f64 + ((target_value as f64 - start_value as f64) * curve_fn(step as f64/steps as f64) - y_offset) * y_scale).max(0f64).min(255f64) as DmxValue).collect()
}
