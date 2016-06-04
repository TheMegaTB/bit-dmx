use rustc_serialize::json;
use std::io::{BufReader, BufWriter};
use std::fs::File;
use std::path::PathBuf;
use std::io::prelude::*;

#[derive(Debug, Clone, RustcDecodable, RustcEncodable)]
pub struct FrontendConfig {
    pub theme_id: usize,
    pub chasers: Vec<String>
}

impl FrontendConfig {
    pub fn empty() -> FrontendConfig {
        FrontendConfig {
            theme_id: 0,
            chasers: Vec::new()
        }
    }
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
    pub fn save(&self, path: PathBuf) {
        let file = File::create(path).expect("no such file");
        let mut buf = BufWriter::new(file);
        buf.write_all(json::encode(self).unwrap().as_bytes()).unwrap();
    }
}
