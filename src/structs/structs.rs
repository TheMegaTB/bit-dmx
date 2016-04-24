use std::time::Duration;
use std::thread::sleep;

extern crate meval;
pub use meval::*;

pub type FadeTime = u16;
pub type DmxChannel = u16;
pub type DmxValue = u8;

pub mod helpers;
pub use helpers::*;

pub mod fixture;
pub use fixture::*;

pub mod stage;
pub use stage::*;

pub mod fade_curve;
pub use fade_curve::*;

pub mod fixtures;
pub use fixtures::*;



#[test]
#[should_panic]
fn test_fade_curve() {
    let curve_fn = &*FadeCurve::Custom("sin(2*x)".to_string()).to_function();
    println!("curve value @ 3.0 {:?}", curve_fn(3f64));
    for a in helpers::fade(100, 200, 150, curve_fn) {
        println!("{:?}", a);
    } //fade from 0 to 255 in 5s with 30fps
}
