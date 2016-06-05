use rustc_serialize::json;
use std::io::{BufReader, BufWriter};
use std::fs::File;
use std::path::PathBuf;
use std::io::prelude::*;

#[derive(Debug, Clone, RustcDecodable, RustcEncodable)]
/// The struct to save the client project configuration.
pub struct FrontendConfig {
    /// The id of the theme that is used.
    pub theme_id: usize,
    /// A list of chasers that are shown in this client.
    pub chasers: Vec<String>
}

impl FrontendConfig {
    /// Generates a default configuration.
    pub fn empty() -> FrontendConfig {
        FrontendConfig {
            theme_id: 0,
            chasers: Vec::new()
        }
    }
    /// Loads a client project configuration from a given path.
    pub fn load(path: PathBuf) -> Option<FrontendConfig> {
        match File::open(path) {
            Ok(file) => {
                let mut buf = BufReader::new(file);
                let mut json_data: String = String::new();
                let _ = buf.read_to_string(&mut json_data);
                Some(json::decode(&json_data).unwrap())
            },
            _ => {
                None
            }
        }
    }
    /// Saves the client project configuration to a given path.
    pub fn save(&self, path: PathBuf) {
        let file = File::create(path).expect("no such file");
        let mut buf = BufWriter::new(file);
        buf.write_all(json::encode(self).unwrap().as_bytes()).unwrap();
    }
}
