use rustc_serialize::json;
use std::collections::HashMap;
use std::io::prelude::*;
use flate2::read::ZlibDecoder;
use std::env;
use std::fs::{File, self};
use std::path::PathBuf;
use std::error::Error;

fn decompress_assets() -> PathBuf {
    let mut binary_data: Vec<u8> = Vec::new(); binary_data.extend_from_slice(include_bytes!("assets.bin"));
    let mut binary_data_slice = binary_data.as_slice();
    let mut d = ZlibDecoder::new(&mut binary_data_slice);
    let mut data = Vec::new();
    d.read_to_end(&mut data).unwrap();

    let assets: HashMap<String, Vec<u8>> = json::decode(&String::from_utf8_lossy(&data)).unwrap();
    let mut tmp = env::temp_dir();
    tmp.push("BitDMX/");
    match fs::create_dir(tmp.clone()) {
        Ok(_) => {}, Err(e) => debug!("Couldn't create tmp dir: {:?}", e.description())
    }
    for (path, mut data) in assets.into_iter() {
        let tmp_path = tmp.join(path);
        match fs::create_dir_all(&tmp_path.parent().unwrap()) {
            Ok(_) => {}, Err(e) => debug!("Couldn't create path for {}: {:?}", tmp_path.display(), e.description())
        }
        match File::create(tmp_path.clone()) {
            Ok(mut f) => f.write_all(&mut data).unwrap(),
            Err(e) => debug!("Couldn't create assets file {}: {:?}", tmp_path.display(), e.description())
        }
    }
    tmp.join("assets/")
}

pub fn get_assets_path() -> PathBuf {
    let mut tmp = env::temp_dir();
    tmp.push("BitDMX/assets");
    if !tmp.is_dir() { decompress_assets(); }
    tmp
}
