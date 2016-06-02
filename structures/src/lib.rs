#[macro_use] extern crate conrod;
#[macro_use] extern crate log;
extern crate rustc_serialize;
extern crate net2;
extern crate piston_window;
extern crate find_folder;
extern crate ansi_term;
extern crate flate2;
extern crate meval;

pub const VERSION: &'static str = env!("CARGO_PKG_VERSION");

pub type FadeTime = usize;
pub type DmxAddress = u16;
pub type DmxValue = u8;
pub type ChannelGroupValueTuple = (Vec<DmxValue>, (FadeCurve, FadeTime), (FadeCurve, FadeTime));
const FADE_TICKS: FadeTime = 30;

#[macro_use]
pub mod helpers;
pub use helpers::*;

pub mod config;
pub use config::*;

pub mod compressed_data;
pub use compressed_data::*;

pub mod git_hash;
pub use git_hash::*;

pub mod logger;
pub use logger::*;

pub mod colors;
pub use colors::*;

pub mod theme;
pub use theme::*;

pub mod window;
pub use window::*;

pub mod udp_socket;
pub use udp_socket::*;

pub mod channel;
pub use channel::*;

pub mod channel_group_value;
pub use channel_group_value::*;

pub mod dmx_parser;
pub use dmx_parser::*;

pub mod fixture;
pub use fixture::*;

pub mod switch;
pub use switch::*;

pub mod chaser;
pub use chaser::*;

pub mod frontend_data;
pub use frontend_data::*;

pub mod stage;
pub use stage::*;

pub mod fade_curve;
pub use fade_curve::*;

pub mod channel_groups;
pub use channel_groups::single::*;
pub use channel_groups::rgb::*;
pub use channel_groups::rgba::*;
pub use channel_groups::moving_2d::*;
