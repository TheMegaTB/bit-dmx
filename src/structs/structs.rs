
extern crate meval;
pub use meval::*;

pub type FadeTime = u16;
pub type DmxChannel = u16;
pub type DmxValue = u8;

pub mod fixture;
pub use fixture::*;

pub mod stage;
pub use stage::*;

pub mod fade_curve;
pub use fade_curve::*;

#[test]
#[should_panic]
fn test_fade_curve() {
    let curve_fn = &*FadeCurve::Custom("sin(x)".to_string()).to_function();
    println!("curve value @ 3.0 {:?}", curve_fn(3f64));
    test_fade(100, 200, 5000, 30, curve_fn); //fade from 0 to 255 in 5s with 30fps
}

//time in ms
#[allow(dead_code)]
#[allow(unused_variables)]
fn fake_delay(time: FadeTime) {

}

//deltat in ms
#[allow(dead_code)]
fn test_fade(start_value: DmxValue, target_value: DmxValue, deltat: FadeTime, ticks_per_second: FadeTime, curve_fn: &Fn(f64) -> f64) {
    let delay = 1000/ticks_per_second;
    let total_steps = deltat/1000*ticks_per_second;

    let y_offset = curve_fn(0f64);
    let y_scale = 1f64/(curve_fn(1f64)-y_offset);


    for step in 0..total_steps + 1 {
        let value = start_value as f64 + ((target_value-start_value) as f64 * curve_fn(step as f64/total_steps as f64) - y_offset) * y_scale;
        println!("{:?}: {:?}", step, value);
        fake_delay(delay); //TODO add functional delay
    }
}
