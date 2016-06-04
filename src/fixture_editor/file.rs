use std::env;
use std::fs::File;
use std::io::prelude::*;

pub fn get_path() -> String {
    env::current_dir().unwrap().display().to_string() + "/config/server/"
}

pub fn check_for_file(path: String) -> bool {
    match File::open(path) {
        Ok(file) => true,
        _ => false
    }
}

pub fn get_file_content(path: String) -> String {
    let mut f: File;

    match File::open(path) {
        Ok(file) => f = file,
        _ => panic!("File disapeared")
    }

    let mut content = String::new();

    f.read_to_string(&mut content);

    content.clone().to_string()
}

pub fn write_file_content(path: String, content: String) {
    let mut file_to_save: File;
    match File::create(path) {
        Ok(file) => file_to_save = file,
        _ => panic!("Couldn't create file")
    }

    let to_write: &[u8] = content.as_bytes();

    file_to_save.write_all(to_write);


}
