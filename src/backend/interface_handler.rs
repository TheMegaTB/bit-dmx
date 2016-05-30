use std::sync::mpsc;
use std::thread;

use std::ffi::CString;
use std::os::raw::c_char;

use structures::DmxAddress;
use structures::DmxValue;
use structures::fake_if_print;

use std::error::Error;

#[allow(dead_code)]
extern {
    fn open_port(baudrate: usize, port: *const c_char) -> bool;
    fn close_port();
    fn write_dmx(address: u16, value: u8);
    fn is_connected() -> bool;
    fn set_fake_interface_mode(enabled: bool);
}

fn connect(baudrate: usize, port: String) -> bool {
    unsafe {
        match CString::new(port) {
            Ok(port) => open_port(baudrate, port.as_ptr()),
            Err(e) => {exit!(4, "Invalid port string. (Can't convert to CString: {})", e.description());}
        }
    }
}

fn disconnect() {
    unsafe { close_port() }
}

fn write_to_dmx(address: DmxAddress, value: DmxValue) {
    unsafe { write_dmx(address, value) }
}

pub struct Interface {
    baudrate: usize,
    port: String
}

pub struct InterfaceHandle {
    interface: Interface,
    fake: bool
}

impl Interface {
    pub fn new() -> Interface {
        Interface {
            baudrate: 115200,
            port: "/dev/ttyACM0".to_string()
        }
    }

    #[allow(dead_code)]
    pub fn baudrate(mut self, baudrate: usize) -> Interface {
        self.baudrate = baudrate;
        self
    }

    #[allow(dead_code)]
    pub fn port(mut self, port: String) -> Interface {
        self.port = port;
        self
    }

    pub fn connect(self) -> Result<InterfaceHandle, InterfaceHandle> {
        match connect(self.baudrate, self.port.clone()) {
            true => Ok(InterfaceHandle {interface: self, fake: false}),
            false => {
                unsafe { set_fake_interface_mode(true); }
                //warn!("Enabled fake interface since no hardware interface is detected.");
                Err(InterfaceHandle {interface: self, fake: true})
            }
        }
    }
}

type DmxTouple = (DmxAddress, DmxValue);
fn insert_to_vector(cache: &mut Vec<DmxTouple>, elem: DmxTouple) {
    match cache.iter().position(|&x| x.0 == elem.0 ) {
        Some(index) => cache[index] = elem,
        None => cache.push(elem)
    }
}

impl InterfaceHandle {
    pub fn to_thread(self) -> (mpsc::Sender<(DmxAddress, DmxValue)>, mpsc::Sender<bool>) {
        let (tx, rx) = mpsc::channel();
        let (interrupt_tx, interrupt_rx) = mpsc::channel();
        match thread::Builder::new().name("DMX-IF".to_string()).spawn(move|| {

            let mut cache: Vec<DmxTouple> = Vec::with_capacity(16);

            loop {
                if interrupt_rx.try_recv().is_ok() {
                    self.disconnect();
                    return;
                }

                let mut rx_available = true;
                while rx_available {
                    match rx.try_recv() {
                        Ok(elem) => {
                            insert_to_vector(&mut cache, elem);
                        },
                        Err(_) => {
                            if cache.len() == 0 {
                                match rx.recv() {
                                    Ok(elem) => {
                                        insert_to_vector(&mut cache, elem);
                                    },
                                    Err(e) => {error!("Receive error: {:?}", e)}
                                }
                            } else {
                                rx_available = false;
                            }
                        }
                    }
                }

                match cache.drain(0..1).last() {
                    Some(elem) => {
                        trace!("Setting channel {:?} to value {:?}", elem.0, elem.1);
                        self.write_to_dmx(elem.0, elem.1);
                    },
                    None => { unreachable!() }
                }
            }
        }) {
            Ok(_) => {}, Err(e) => {exit!(5, "Error whilst spawning DMX Interface thread: {}", e.description());}
        };
        (tx, interrupt_tx)
    }

    pub fn write_to_dmx(&self, address: DmxAddress, value: DmxValue) {
        if self.fake { fake_if_print(address, value); }
        write_to_dmx(address, value);
    }

    pub fn disconnect(self) -> Interface {
        disconnect();
        self.interface
    }
}
