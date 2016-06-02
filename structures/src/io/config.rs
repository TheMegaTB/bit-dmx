use std::path::PathBuf;
use find_folder;

use std::error::Error;

pub enum Config {
    Server,
    Client
}

pub fn get_config_path(conf: Config, name: &String) -> PathBuf {
    // TODO: Move to .config / %AppData% and create if it doesn't exist
    let mut config_path = find_folder::Search::KidsThenParents(3, 5)
        .for_folder("config").unwrap();
    config_path = match conf {
        Config::Server => config_path.join("server").join(name),
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
