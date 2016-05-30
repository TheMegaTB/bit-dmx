use log::{LogRecord, LogLevel, LogLocation, LogMetadata, SetLoggerError, set_logger, self};
use std::env;
use std::str::FromStr;
pub use ansi_term::*;

use std::error::Error;

use DmxAddress;
use DmxValue;

struct SimpleLogger {
    level: LogLevel,
    show_gfx_log: bool,
    show_paths: bool
}

impl SimpleLogger {
    fn is_gfx_log(&self, location: &LogLocation) -> bool {
        location.module_path().find("gfx_device_gl").is_some()
    }
}

impl log::Log for SimpleLogger {
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

const DEFAULT_LOGLEVEL: LogLevel = LogLevel::Info;

pub fn init_logger() {//-> Result<(), SetLoggerError> {
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
        Box::new(SimpleLogger {
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

pub fn fake_if_print(channel: DmxAddress, value: DmxValue) {
    let prefix = Colour::Fixed(14).bold().paint("   Interface");
    println!("{} C{} -> V{}", prefix, channel, value);
}
