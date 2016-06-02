use std::path::PathBuf;
use find_folder;

pub enum Config {
    Server,
    Client
}

pub fn get_config_path(conf: Config, name: &String) -> PathBuf {
    let mut config_path = find_folder::Search::KidsThenParents(3, 5)
        .for_folder("config").unwrap();
    config_path = match conf {
        Config::Server => config_path.join("server").join(name),
        Config::Client => config_path.join("client")
    };
    config_path
}
