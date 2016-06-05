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

pub mod logic;
pub use logic::server;
