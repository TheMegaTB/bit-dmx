#[macro_use] extern crate log;
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

pub mod dmx_parser;
pub use dmx_parser::*;

pub mod fixture;
pub use fixture::*;

pub mod stage;
pub use stage::*;

pub mod fade_curve;
pub use fade_curve::*;

pub mod channel_groups;
pub use channel_groups::single::*;
pub use channel_groups::rgb::*;
pub use channel_groups::rgba::*;
pub use channel_groups::moving_2d::*;
