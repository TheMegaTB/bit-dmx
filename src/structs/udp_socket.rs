#![allow(dead_code)]
use std::net::{ UdpSocket, IpAddr, Ipv4Addr, SocketAddr, SocketAddrV4 };
use std::sync::{Arc, Mutex};
use std::time::Duration;
use std::thread::sleep;
use std::str::FromStr;
use std::thread;

use net2::UdpSocketExt;

const INPUT_BUFFER: usize = 4;
const WATCHDOG_TTL: u64 = 5;
const WATCHDOG_DATA: [u8; 3] = [68, 77, 88]; // "DMX" as bytes

pub struct UDPSocket {
    local_addr: Ipv4Addr,
    multicast_addr: Ipv4Addr,
    port: u16
}

pub struct UDPSocketHandle {
    socket: UdpSocket,
    multicast_addr: SocketAddr
}

pub struct WatchDogClient {
    server_addr: Arc<Mutex<[Option<IpAddr>; 1]>>,
    state: Arc<Mutex<[bool; 1]>>
}

impl UDPSocket {
    pub fn new() -> UDPSocket {
        UDPSocket {
            local_addr: Ipv4Addr::new(0, 0, 0, 0),
            multicast_addr: Ipv4Addr::new(228, 228, 228, 228),
            port: 8000
        }
    }

    pub fn port(mut self, port: u16) -> UDPSocket {
        self.port = port;
        self
    }

    pub fn local_addr(mut self, ip: &'static str) -> UDPSocket {
        self.local_addr = FromStr::from_str(&ip).ok().expect("Failed to resolve IP.");
        self
    }

    pub fn multicast_addr(mut self, ip: &'static str) -> UDPSocket {
        self.multicast_addr = FromStr::from_str(&ip).ok().expect("Failed to resolve IP.");
        self
    }

    fn assemble_socket(&self, port: u16, multicast: bool) -> UdpSocket {
        let sock = UdpSocket::bind(SocketAddrV4::new(self.local_addr, port)).unwrap();
        if multicast { sock.join_multicast_v4(&self.multicast_addr, &self.local_addr).ok().expect("Failed to join multicast."); }
        sock
    }

    pub fn start_client(&self) -> UDPSocketHandle {
        UDPSocketHandle {
            socket: self.assemble_socket(self.port, true),
            multicast_addr: SocketAddr::V4(SocketAddrV4::new(self.local_addr, self.port))
        }
    }

    pub fn start_backend_server(&self) -> UDPSocketHandle {
        UDPSocketHandle {
            socket: self.assemble_socket(self.port + 1, true),
            multicast_addr: SocketAddr::V4(SocketAddrV4::new(self.local_addr, self.port))
        }
    }

    pub fn start_watchdog_server(&self) {
        let sock = self.assemble_socket(0, true);
        let target_addr = SocketAddr::V4(SocketAddrV4::new(self.multicast_addr, self.port+2));
        thread::Builder::new().name("WatchDog-Server".to_string()).spawn(move|| {
            loop {
                sock.send_to(&WATCHDOG_DATA, target_addr).unwrap();
                sleep(Duration::from_secs(WATCHDOG_TTL));
            }
        }).unwrap();
    }

    pub fn start_watchdog_client(&self) -> WatchDogClient {
        let sock = self.assemble_socket(self.port + 2, true);
        let state = Arc::new(Mutex::new([false]));
        let server_addr = Arc::new(Mutex::new([None]));
        {
            let s = state.clone();
            let s_addr = server_addr.clone();
            thread::Builder::new().name("WatchDog-Client".to_string()).spawn(move|| {
                sock.set_read_timeout(Some(Duration::from_secs(WATCHDOG_TTL + 1))).unwrap();
                loop {
                    let mut buf = WATCHDOG_DATA;
                    match sock.recv_from(&mut buf) {
                        Ok((_, addr)) => {
                            if buf == WATCHDOG_DATA {
                                s.lock().unwrap()[0] = true;
                                s_addr.lock().unwrap()[0] = Some(addr.ip());
                            } else {
                                s.lock().unwrap()[0] = false;
                                s_addr.lock().unwrap()[0] = None;
                            }
                        },
                        Err(_) => {
                            s.lock().unwrap()[0] = false;
                            s_addr.lock().unwrap()[0] = None;
                        }
                    }
                }
            }).unwrap();
        }
        WatchDogClient {
            server_addr: server_addr,
            state: state
        }
    }
}

impl UDPSocketHandle {
    pub fn send(&self, data: &[u8], target: SocketAddr) -> usize {
        self.socket.send_to(data, target).ok().expect("Failed to send transmission")
    }

    pub fn send_to_multicast(&self, data: &[u8]) -> usize {
        self.send(data, self.multicast_addr)
    }

    pub fn receive(&self) -> ([u8; INPUT_BUFFER], SocketAddr) {
        let mut buf = [0; INPUT_BUFFER];
        let src = self.socket.recv_from(&mut buf).ok().expect("Failed to receive package.").1;
        (buf, src)
    }
}

impl WatchDogClient {
    pub fn is_alive(&self) -> bool {
        let state = self.state.lock().unwrap();
        state[0]
    }

    pub fn get_server_addr(&self) -> Option<IpAddr> {
        let server_addr = self.server_addr.lock().unwrap();
        server_addr[0]
    }
}
