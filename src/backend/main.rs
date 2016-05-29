#[macro_use] extern crate log;
#[macro_use] extern crate structures;
extern crate net2;
extern crate rustc_serialize;

use std::thread;
use std::sync::{Arc, Mutex};

use std::io::prelude::*;
use std::env;

mod interface_handler;
use interface_handler::*;

use rustc_serialize::json;

use structures::*;

fn main() {
    init_logger().unwrap();
    info!("BitDMX backend v{}-{}", VERSION, GIT_HASH);

    // exit!(2, "Hi");

    let args: Vec<_> = env::args().collect();
    let instance_name = if args.len() > 1 {
        args[1].clone()
    } else {
        "Untitled".to_string()
    };

    let interface_port = if args.len() > 2 {
        args[2].clone()
    } else {
        "/dev/ttyACM0".to_string()
    };

    info!("Server started as \"{}\"", instance_name);

    let interface = Interface::new().port(interface_port).connect();
    if interface.is_err() { panic!(interface) }
    let (tx, _interrupt_tx) = interface.unwrap().to_thread();

    let mut stage = Parser::new(Stage::new(instance_name, tx)).parse();
    stage.load_config();

    for fixture in stage.fixtures.iter_mut() {
        match fixture.channel_groups[0] {
            ChannelGroup::Single(ref mut group) => {
                group.activate_preheat(FadeCurve::Squared, 1000);
            },
            _ => {}
        }
    }

    let mut socket = UDPSocket::new();
    socket.start_watchdog_server();
    let server = socket.start_backend_server();

    let stage = Arc::new(Mutex::new(stage));

    {
        let stage = stage.clone();
        thread::spawn(move || {
            loop {
                let (d, _) = server.receive();
                trace!("{:?}", d);

                let address_type:u8 = d[0] & 127;
                let shift: bool = d[0] & 128 != 0;
                let address: u16 = ((d[1] as u16) << 8) + (d[2] as u16);
                let value: u8 = d[3];


                if address_type == 0 {
                    // Channel
                    let stage_locked = stage.lock().unwrap();
                    let mut channel_locked = stage_locked.channels[address as usize].lock().unwrap();
                    channel_locked.stop_fade();
                    channel_locked.set(value);
                }
                else if address_type == 1 {
                    // Switch
                    let mut stage_locked = stage.lock().unwrap();
                    debug!("Set switch with address {:?} to {:?} (shifted: {:?})", address, value, shift);
                    if shift {
                        stage_locked.deactivate_group_of_switch(address as usize)
                    }
                    stage_locked.set_switch(address as usize, value as f64);
                }
                else if address_type == 2 {
                    // Chaser
                    debug!("Start chaser with switch with address {:?} to {:?} (shifted: {:?})", address, value, shift);
                    start_chaser_of_switch(stage.clone(), address as usize, value as f64);
                }
            }
        });
    }

    {
        let stage = stage.clone();
        thread::spawn(move || {
            use std::io::Write;
            use std::net::TcpListener;

            let listener = TcpListener::bind("0.0.0.0:8000").unwrap();
            debug!("TCP Server A (send) started");
            for stream in listener.incoming() {
                let stage = stage.clone();
                thread::spawn(move || {
                    let stage_locked = stage.lock().unwrap();
                    let mut stream = stream.unwrap();
                    stream.write(stage_locked.get_frontend_data().get_json_string().as_bytes()).unwrap();
                });
            }
        });
    }

    {
        let stage = stage.clone();
        thread::spawn(move || {
            use std::net::TcpListener;

            let listener = TcpListener::bind("0.0.0.0:8001").unwrap();
            debug!("TCP Server B (recv) started");
            for stream in listener.incoming() {
                let stage = stage.clone();
                thread::spawn(move || {
                    let mut stream = stream.unwrap();
                    let mut buffer = String::new();
                    let _ = stream.read_to_string(&mut buffer);
                    let frontend_data: FrontendData = json::decode(&buffer).unwrap();
                    {
                        let mut stage_locked = stage.lock().unwrap();
                        stage_locked.from_frontend_data(frontend_data);
                        stage_locked.save_config();
                    }
                    UDPSocket::new().start_frontend_client().send_to_multicast(&[255, 255, 255, 255]);
                });
            }
        }).join().unwrap();
    }
}
