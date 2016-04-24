extern crate structures;
use structures::*;

fn main() {
    let socket = UDPSocket::new();
    let watchdog = socket.start_watchdog_client();
    std::thread::sleep(std::time::Duration::from_secs(10));
    println!("{}", watchdog.is_alive());
    println!("{:?}", watchdog.get_server_addr());
}
