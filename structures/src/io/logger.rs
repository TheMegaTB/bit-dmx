//! Very sexy logger specifically designed for BitDMX.
use log::{LogRecord, LogLevel, LogLocation, LogMetadata, set_logger, self};
use std::env;
use std::str::FromStr;
pub use ansi_term::*;

use std::error::Error;

use logic::channel::DmxAddress;
use logic::channel::DmxValue;

const DEFAULT_LOGLEVEL: LogLevel = LogLevel::Info;

/// The logger type responsible for printing that sexy output you see when launching BitDMX
pub struct Logger {
    level: LogLevel,
    show_gfx_log: bool,
    show_paths: bool
}

impl Logger {
    /// This function initializes the logger and enables it.
    ///
    /// When enabling it reads the following environment variables for configuration:
    ///
    /// `LOG`      the loglevel at which it may print (trace, debug, info, warn, error)
    ///
    /// `PATHS`    whether or not to show the origin of a message (true, false)
    ///
    /// `GFX_LOG`  if the output of the gfx crate regarding system info should be shown or not (true, false)
    pub fn init() {
        match set_logger(|max_log_level| {
            let level = match env::var("LOG") {
                Ok(level) => {
                    match LogLevel::from_str(&level) {
                        Ok(level) => level,
                        Err(_) => DEFAULT_LOGLEVEL
                    }
                },
                Err(_) => DEFAULT_LOGLEVEL
            };
            let show_paths = match env::var("PATHS") {
                Ok(val) => val == String::from("true"),
                Err(_) => false
            };
            let show_gfx_log = match env::var("GFX_LOG") {
                Ok(val) => val == String::from("true"),
                Err(_) => false
            };
            max_log_level.set(level.to_log_level_filter());
            Box::new(Logger {
                level: level,
                show_gfx_log: show_gfx_log,
                show_paths: show_paths
            })
        }) {
            Ok(_) => {},
            Err(e) => {
                println!("{} Failed to set logger: {}", Colour::Fixed(160).bold().paint("       Error"), e.description());
                ::std::process::exit(6);
            }
        }
    }
    fn is_gfx_log(&self, location: &LogLocation) -> bool {
        location.module_path().find("gfx_device_gl").is_some()
    }
}

impl log::Log for Logger {
    fn enabled(&self, metadata: &LogMetadata) -> bool {
        metadata.level() <= self.level
    }

    fn log(&self, record: &LogRecord) {
        if self.enabled(record.metadata()) && (!self.is_gfx_log(record.location()) || record.level() <= LogLevel::Warn || self.show_gfx_log) {
            let path = match self.level {
                LogLevel::Trace => {
                    let loc = record.location();
                    format!("{}:{}", loc.file(), loc.line())
                },
                LogLevel::Debug => {
                    format!("{}", record.location().module_path())
                },
                _ => {String::new()}
            };
            let level = match record.level() {
                LogLevel::Error => { Colour::Fixed(160).bold().paint("       Error") },
                LogLevel::Warn  => { Colour::Fixed(214).bold().paint("     Warning") },
                LogLevel::Info  => { Colour::Fixed( 10).bold().paint("        Info") },
                LogLevel::Debug => { Colour::Fixed(244).bold().paint("       Debug") },
                LogLevel::Trace => { Colour::Fixed(239).bold().paint("       Trace") },
            };
            if self.show_paths {
                println!("{} {}\n             {}", level, record.args(), Colour::Fixed(239).paint(path));
            } else {
                println!("{} {}", level, record.args());
            }
        }
    }
}

/// Print the (fake) interface output according to the style guidelines set by the logger
pub fn fake_if_print(channel: DmxAddress, value: DmxValue) {
    let prefix = Colour::Fixed(14).bold().paint("   Interface");
    println!("{} C{} -> V{}", prefix, channel, value);
}
