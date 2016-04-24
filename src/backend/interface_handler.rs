use std::time::Duration;
use std::thread::sleep;

use std::ffi::CString;
use std::os::raw::c_char;

use structures::DmxChannel;
use structures::DmxValue;

extern {
    fn open_port(baudrate: usize, port: *const c_char) -> bool;
    fn close_port();
    fn write_dmx(channel: u16, value: u8);
    fn is_connected() -> bool;
}

pub fn connect(baudrate: usize, port: String) -> bool {
    unsafe { open_port(baudrate, CString::new(port).unwrap().as_ptr()) }
}

pub fn disconnect() {
    unsafe { close_port() }
}

pub fn write_to_dmx(channel: DmxChannel, value: DmxValue) {
    unsafe { write_dmx(channel, value) }
}

pub fn connect_and_test() {
    if connect(115200, "/dev/ttyACM0".to_string()) {
        (0..255).chain((0..255).rev()).map(|i| {
            write_to_dmx(1, i);
            write_to_dmx(2, i);
            write_to_dmx(3, i);
            println!("{}", i);
        }).collect::<Vec<_>>();
        disconnect();
    }
    // unsafe {
    //     if open_port(115200, port.as_ptr()) {
    //         println!("Connection established!");
    //         (0..255).chain((0..255).rev()).map(|i| {
    //             write_dmx(1, i);
    //             write_dmx(2, i);
    //             write_dmx(3, i);
    //             // println!("{}", i);
    //             // sleep(Duration::from_millis(50));
    //         }).collect::<Vec<_>>();
    //         println!("Data transmitted.");
    //         close_port();
    //     } else {
    //         println!("Failed to open serial port!");
    //     }
    // }
}
