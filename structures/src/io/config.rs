//! Functions and enums to provide a dynamic config path
use std::path::PathBuf;
use find_folder;

use std::error::Error;

/// The `Config` type. It defines what config a program uses.
pub enum Config {
    /// Configuration directory for server named after the value
    Server(String),
    /// Configuration directory for client containing all configs of all servers
    Client
}

/// This function converts the provided `conf` type into a fully qualified path pointing to the specific directory
///
/// Even though this function returns a path whilst the binary resists in the target directory it does not create a config directory yet.
///
pub fn get_config_path(conf: Config) -> PathBuf {
    // TODO: Move to .config / %AppData% and create if it doesn't exist
    let mut config_path = find_folder::Search::KidsThenParents(3, 5)
        .for_folder("config").unwrap();
    config_path = match conf {
        Config::Server(name) => config_path.join("server").join(name),
        Config::Client => config_path.join("client")
    };
    if match config_path.metadata() {
        Ok(m) => {
            if m.is_dir() { false } else { true }
        },
        Err(_) => {
            warn!("No config directory found.");
            true
        }
    } {
        match ::std::fs::create_dir_all(&config_path) {
            Ok(_) => {
                info!("Created config directory @ {:?}", config_path);
            }, Err(e) => {
                exit!(8, "Unable to create config files: {}", e.description());
            }
        }
    }
    config_path
}
