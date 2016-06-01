use std::path::PathBuf;
use find_folder;

pub fn get_config_path() -> PathBuf {
    find_folder::Search::KidsThenParents(3, 5)
        .for_folder("config").unwrap()
}
