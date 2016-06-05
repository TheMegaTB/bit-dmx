//! Networking related structs and functions like UDPSocket and WatchDog
use std::net::{ UdpSocket, IpAddr, Ipv4Addr, SocketAddr, SocketAddrV4 };
use std::sync::{Arc, Mutex};
use std::time::Duration;
use std::thread::sleep;
use std::str::FromStr;
use std::thread;
use std::error::Error;
use std::sync::{Once, ONCE_INIT};

use VERSION;
use GIT_HASH;

/// Buffer length for incoming datagrams
pub const INPUT_BUFFER: usize = 4;
/// Interval at which the watchdog broadcasts
pub const WATCHDOG_TTL: u64 = 1;

const MULTICAST: &'static str = "228.228.228.228";
const BASE_PORT: u16 = 8000;

static PRINT_LOCAL_WARNING: Once = ONCE_INIT;

/// Builder struct for `UDPSocketHandle`
#[derive(Debug)]
pub struct UDPSocket {
    local_addr: Ipv4Addr,
    multicast_addr: Ipv4Addr,
    /// The base port on which the sockets are based on
    pub port: u16
}

/// A handle for communication via UDP multicast
#[derive(Debug)]
pub struct UDPSocketHandle {
    /// The `std::net::UdpSocket` that is used for communication
    pub socket: UdpSocket,
    multicast_addr: SocketAddr
}

/// A client that searches for ongoing broadcasts from servers
#[derive(Debug, Clone)]
pub struct WatchDogClient {
    /// A enclosure containing the current server address
    pub server_addr: Arc<Mutex<[Option<IpAddr>; 1]>>,
    /// A enclosure containing the current state (connected or not)
    pub state: Arc<Mutex<[bool; 1]>>
}

impl UDPSocket {
    /// Creates a new `UDPSocketHandle` builder
    pub fn new() -> UDPSocket {
        UDPSocket {
            local_addr: Ipv4Addr::new(0, 0, 0, 0),
            multicast_addr: Ipv4Addr::from_str(MULTICAST).expect("Failed to convert MULTICAST const to IP."),
            port: BASE_PORT
        }
    }

    /// Change the port of the resulting socket
    pub fn port(mut self, port: u16) -> UDPSocket {
        self.port = port;
        self
    }

    /// Change the local address on which the socket will bind to
    pub fn local_addr(mut self, ip: &'static str) -> UDPSocket {
        self.local_addr = FromStr::from_str(&ip).ok().expect("Failed to resolve IP.");
        self
    }

    /// Change the multicast group the socket will attempt to join
    pub fn multicast_addr(mut self, ip: &'static str) -> UDPSocket {
        self.multicast_addr = FromStr::from_str(&ip).ok().expect("Failed to resolve IP.");
        self
    }

    /// Assemble a `std::net::UdpSocket` with the previously defined parameters and a port delta. `None` results in it binding to a random free port
    pub fn assemble_socket(&mut self, delta_opt: Option<u16>) -> UdpSocket {
        let port = match delta_opt {
            Some(delta) => self.port+delta,
            None => 0
        };
        let sock = match UdpSocket::bind(SocketAddrV4::new(self.local_addr, port)) {
            Ok(s) => s, Err(e) => {exit!(8, "Error binding UDP socket: {}", e.description());}
        };
        match sock.join_multicast_v4(&self.multicast_addr, &self.local_addr) {
            Ok(_) => sock,
            Err(_) => {
                PRINT_LOCAL_WARNING.call_once(|| {
                    warn!("Multicast support not available. (NET_ERR)"); //Falling back to local mode since multicast is not available.
                });
                self.multicast_addr = Ipv4Addr::new(127, 0, 0, 1);
                sock
            }
        }
    }

    /// Create a frontend server binding to the previously defined port
    pub fn start_frontend_server(&mut self) -> UDPSocketHandle {
        UDPSocketHandle {
            socket: self.assemble_socket(Some(0)),
            multicast_addr: SocketAddr::V4(SocketAddrV4::new(self.multicast_addr, self.port))
        }
    }

    /// Create a frontend client that binds to a random port
    pub fn start_frontend_client(&mut self) -> UDPSocketHandle {
        UDPSocketHandle {
            socket: self.assemble_socket(None),
            multicast_addr: SocketAddr::V4(SocketAddrV4::new(self.multicast_addr, self.port))
        }
    }

    /// Create a backend server running on `port+1`
    pub fn start_backend_server(&mut self) -> UDPSocketHandle {
        UDPSocketHandle {
            socket: self.assemble_socket(Some(1)),
            multicast_addr: SocketAddr::V4(SocketAddrV4::new(self.multicast_addr, self.port))
        }
    }

    /// Launch a watchdog server based on the previously defined settings, broadcasting the local addr and version
    pub fn start_watchdog_server(&mut self) {
        let sock = self.assemble_socket(None);
        let target_addr = SocketAddr::V4(SocketAddrV4::new(self.multicast_addr, self.port+2));
        thread::Builder::new().name("WatchDog-Server".to_string()).spawn(move|| {
            let payload = VERSION.to_string() + &GIT_HASH.to_string();
            loop {
                sleep(Duration::from_secs(WATCHDOG_TTL));
                match sock.send_to(payload.as_bytes(), target_addr) {
                    Ok(_) => {}, Err(e) => {
                        exit!(6, "Error whilst sending beacon signal: {}", e.description());
                    }
                };
            }
        }).unwrap();
    }

    /// Convert the `UDPSocket` builder into a `WatchDogClient` instead of a `UDPSocketHandle`
    pub fn create_watchdog_client(&self) -> WatchDogClient {
        let state = Arc::new(Mutex::new([false]));
        let server_addr = Arc::new(Mutex::new([None]));

        WatchDogClient {
            server_addr: server_addr,
            state: state
        }
    }
}

impl UDPSocketHandle {
    /// Send a datagram `data` to the `target` address
    pub fn send(&self, data: &[u8], target: SocketAddr) -> usize {
        trace!("UDP SEND {:?} -> {:?}", data, target);
        self.socket.send_to(data, target).ok().expect("Failed to send transmission")
    }

    /// Broadcast a datagram `data` to the previously joined multicast group
    pub fn send_to_multicast(&self, data: &[u8]) -> usize {
        self.send(data, self.multicast_addr)
    }

    /// Receive a datagram from any sender
    pub fn receive(&self) -> ([u8; INPUT_BUFFER], SocketAddr) {
        let mut buf = [0; INPUT_BUFFER];
        let src = self.socket.recv_from(&mut buf).ok().expect("Failed to receive package.").1;
        trace!("UDP RECV {:?} <- {:?}", buf, src);
        (buf, src)
    }
}

impl WatchDogClient {
    /// Check whether or not the watchdog is connected to a server that is still alive
    pub fn is_alive(&self) -> bool {
        let state = self.state.lock().expect("Failed to lock Arc!");
        state[0]
    }

    /// Return the server address of the server the watchdog is connected to.
    ///
    /// This may or may not return a value hence the `Option`
    pub fn get_server_addr(&self) -> Option<IpAddr> {
        let server_addr = self.server_addr.lock().expect("Failed to lock Arc!");
        server_addr[0]
    }
}
