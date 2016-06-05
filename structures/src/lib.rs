#![warn(missing_docs)]
//! Crate containing all structures to provide the BitDMX software including logic, server, and UI structs
#[macro_use] extern crate conrod;
#[macro_use] extern crate log;
extern crate rustc_serialize;
extern crate net2;
extern crate piston_window;
extern crate find_folder;
extern crate ansi_term;
extern crate flate2;
extern crate meval;

/// Constant containing version string provided by cargo
pub const VERSION: &'static str = env!("CARGO_PKG_VERSION");

/// Filename for the fixture definitions
pub const FIXTURE_DEF: &'static str = "fixtures.dmx";

#[macro_use]
pub mod helpers;
pub use helpers::*;

pub mod io;
pub mod ui;
pub mod res;
pub mod logic;
pub mod networking;

pub use res::git_hash::GIT_HASH;
pub use res::compressed_data::get_assets_path;
