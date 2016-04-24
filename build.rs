extern crate gcc;

fn main() {
    gcc::compile_library("libinterface.a", &["src/interface/arduino-serial-lib.c", "src/interface/arduino-serial-dmx.c"])
    // gcc::Config::new()
    //             .file("src/interface/arduino-serial-dmx.c")
    //             .include("src/interface")
    //             .compile("interface.a");
}
