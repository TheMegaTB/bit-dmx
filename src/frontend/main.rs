extern crate structures;
use structures::*;

fn main() {
    let socket = UDPSocket::new();
    let watchdog_server = socket.start_watchdog_client();
    std::thread::sleep(std::time::Duration::from_secs(10));
    println!("{}", watchdog_server.is_alive());
    println!("{:?}", watchdog_server.get_server_addr());
}
