pub fn get_path() -> String {
    use std::env;

    env::current_dir().unwrap().display().to_string() + "/config/fixtures.dmx.example"
}

pub fn check_for_file(path: String) -> bool {
    use std::fs::File;

    match File::open(path) {
        Ok(file) => true,
        _ => false
    }
}

pub fn get_file_content(path: String) -> String {
    use std::fs::File;
    use std::io::prelude::*;

    let mut f: File;

    match File::open(path) {
        Ok(file) => f = file,
        _ => panic!("File disapeared")
    }

    let mut content = String::new();

    f.read_to_string(&mut content);

    content.clone().to_string()
}
