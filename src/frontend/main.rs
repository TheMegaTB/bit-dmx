extern crate structures;
use structures::*;

use std::net::SocketAddr;

fn main() {
    let socket = UDPSocket::new();
    let watchdog = socket.start_watchdog_client();
    let client = socket.start_client();
    std::thread::sleep(std::time::Duration::from_secs(10));
    println!("{}", watchdog.is_alive());
    println!("{:?}", watchdog.get_server_addr());

    if watchdog.is_alive() {
        client.send(&[1,2,3,4], SocketAddr::new(watchdog.get_server_addr().unwrap(), 8001));
        println!("{:?}", client.receive());
    }
}
