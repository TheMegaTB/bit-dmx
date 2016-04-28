extern crate structures;
use structures::*;

use std::net::SocketAddr;

fn main() {
    let socket = UDPSocket::new();
    let watchdog = socket.start_watchdog_client();
    let client = socket.start_client();
    std::thread::sleep(std::time::Duration::from_secs(6));
    println!("{}", watchdog.is_alive());
    println!("{:?}", watchdog.get_server_addr());

    if watchdog.is_alive() {
        client.send(&[0,0,5,101], SocketAddr::new(watchdog.get_server_addr().unwrap(), 8001));
        client.send(&[0,0,4,101], SocketAddr::new(watchdog.get_server_addr().unwrap(), 8001));
        println!("{:?}", client.receive());
    }
}
