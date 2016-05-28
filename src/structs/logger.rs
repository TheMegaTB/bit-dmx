use log::{LogRecord, LogLevel, LogMetadata, SetLoggerError, set_logger, self};
use std::env;
use std::str::FromStr;
pub use ansi_term::*;

use DmxAddress;
use DmxValue;

struct SimpleLogger {
    level: LogLevel,
    show_paths: bool
}

impl log::Log for SimpleLogger {
    fn enabled(&self, metadata: &LogMetadata) -> bool {
        metadata.level() <= self.level
    }

    fn log(&self, record: &LogRecord) {
        //println!("level {}", self.level);
        if self.enabled(record.metadata()) {
            // I can probably change colors here
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

pub fn init_logger() -> Result<(), SetLoggerError> {
    set_logger(|max_log_level| {
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
        max_log_level.set(level.to_log_level_filter());
        Box::new(SimpleLogger {
            level: level,
            show_paths: show_paths
        })
    })
}

pub fn fake_if_print(channel: DmxAddress, value: DmxValue) {
    let prefix = Colour::Fixed(14).bold().paint("   Interface");
    println!("{} C{} -> V{}", prefix, channel, value);
}

// #[macro_export]
// macro_rules! fake_if {
//     ($channel:expr, $value:expr) => {
//         fake_if_print($channel, $value);
//     };
// }
