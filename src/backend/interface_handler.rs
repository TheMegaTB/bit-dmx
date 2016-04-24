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

fn connect(baudrate: usize, port: String) -> bool {
    unsafe {
        open_port(baudrate, CString::new(port).unwrap().as_ptr())
    }
}

fn disconnect() {
    unsafe { close_port() }
}

fn write_to_dmx(channel: DmxChannel, value: DmxValue) {
    unsafe { write_dmx(channel, value) }
}

pub struct Interface {
    baudrate: usize,
    port: String
}

pub struct InterfaceHandle {
    values: Vec<DmxValue>,
    interface: Interface
}

impl Interface {
    fn new() -> Interface {
        Interface {
            baudrate: 115200,
            port: "/dev/ttyACM0".to_string()
        }
    }

    fn baudrate(mut self, baudrate: usize) -> Interface {
        self.baudrate = baudrate;
        self
    }

    fn port(mut self, port: String) -> Interface {
        self.port = port;
        self
    }

    fn connect(self) -> Result<InterfaceHandle, &'static str> {
        match connect(self.baudrate, self.port.clone()) {
            true => Ok(InterfaceHandle {
                values: Vec::new(),
                interface: self
            }),
            false => Err("Couldn't connect.")
        }
    }
}

impl InterfaceHandle {
    pub fn write_to_dmx(&self, channel: DmxChannel, value: DmxValue) {
        write_to_dmx(channel, value);
    }

    pub fn disconnect(self) -> Interface {
        disconnect();
        self.interface
    }
}

pub fn connect_and_test() {
    let interface = Interface::new().connect();
    if interface.is_err() { panic!(interface) }
    let interface = interface.unwrap();
    (0..255).chain((0..255).rev()).map(|i| {
        interface.write_to_dmx(1, i);
        println!("{}", i);
    }).collect::<Vec<_>>();
    interface.disconnect();
}
