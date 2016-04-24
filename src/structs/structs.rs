
extern crate net2;

extern crate meval;
pub use meval::*;

pub type FadeTime = usize;
pub type DmxChannel = u16;
pub type DmxValue = u8;
pub const FADE_TICKS: FadeTime = 30;

pub mod helpers;
pub use helpers::*;

pub mod udp_socket;
pub use udp_socket::*;

pub mod fixture;
pub use fixture::*;

pub mod stage;
pub use stage::*;

pub mod fade_curve;
pub use fade_curve::*;

pub mod fixtures;
pub use fixtures::single::*;



// #[test]
// #[should_panic]
// fn test_fade_curve() {
//     //let curve = FadeCurve::Custom("sin(2*x)".to_string());
//     let curve = FadeCurve::Squared;
//     for a in helpers::get_fade_steps(0, 200, 150, curve) {
//         println!("{:?}", a);
//     } //fade from 0 to 255 in 5s with 30fps
// }
