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
        let shift: u8 = 128;
        client.send(&[1,0,5,101], SocketAddr::new(watchdog.get_server_addr().unwrap(), 8001));
        client.send(&[shift + 1,0,4,13], SocketAddr::new(watchdog.get_server_addr().unwrap(), 8001));
        println!("{:?}", client.receive());
    }
}
