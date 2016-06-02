#[macro_use] extern crate conrod;
#[macro_use] extern crate log;
extern crate rustc_serialize;
extern crate net2;
extern crate piston_window;
extern crate find_folder;
extern crate ansi_term;
extern crate flate2;
extern crate meval;

// Version string
pub const VERSION: &'static str = env!("CARGO_PKG_VERSION");

// Globally important type definitions
pub type FadeTime = usize;
pub type DmxAddress = u16;
pub type DmxValue = u8;
pub type ChannelGroupValueTuple = (Vec<DmxValue>, (FadeCurve, FadeTime), (FadeCurve, FadeTime));
const FADE_TICKS: FadeTime = 30;

const FIXTURE_DEF: &'static str = "fixtures.dmx";

// Various helper functions
#[macro_use]
pub mod helpers;
pub use helpers::*;

// Modules
pub mod io;
pub mod ui;
pub mod networking;

// Static content w/ helpers
pub mod res;
pub use res::git_hash::*;
pub use res::compressed_data::get_assets_path;

// All the generic structs - this may need some restructuring as well
pub mod channel;
pub use channel::*;

pub mod channel_group_value;
pub use channel_group_value::*;

pub mod fixture;
pub use fixture::*;

pub mod switch;
pub use switch::*;

pub mod chaser;
pub use chaser::*;

pub mod stage;
pub use stage::*;

pub mod fade_curve;
pub use fade_curve::*;

pub mod channel_groups;
pub use channel_groups::single::*;
pub use channel_groups::rgb::*;
pub use channel_groups::rgba::*;
pub use channel_groups::moving_2d::*;
