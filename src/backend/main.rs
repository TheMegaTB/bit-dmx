#[macro_use] extern crate structures;
#[macro_use] extern crate log;

use std::env;

use structures::io::logger::Logger;
use structures::GIT_HASH;
use structures::VERSION;

use structures::logic::server::server_handler;

fn main() {
    Logger::init();
    info!("BitDMX backend v{}-{}", VERSION, GIT_HASH);

    let args: Vec<_> = env::args().collect();
    let instance_name = if args.len() > 1 {
        args[1].clone()
    } else {
        "Untitled".to_string()
    };

    let interface_port = if args.len() > 2 {
        Some(args[2].clone())
    } else {
        None
    };
    server_handler::start(instance_name, interface_port);
}
