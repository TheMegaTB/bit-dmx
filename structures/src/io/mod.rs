//! IO related modules like config parsing and logging

/// Module for the config.
pub mod config;
/// Module for the fixture configuration parser.
pub mod dmx_parser;
/// Module for the awesome logger.
pub mod logger;

pub use io::dmx_parser::Parser;
pub use io::logger::Logger;
