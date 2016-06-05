use std::thread;
use std::sync::{Arc, Mutex};
use std::error::Error;

use std::io::prelude::*;
use std::env;

use logic::server::interface_handler::*;

use io::logger::Logger;
use GIT_HASH;
use VERSION;
use io::dmx_parser::Parser;
use logic::Stage;
use logic::fade::FadeCurve;
use logic::ChannelGroup;
use networking::UDPSocket;
use logic::chaser::start_chaser_of_switch;
use ui::frontend_data::FrontendData;

pub fn start(instance_name: String, interface_port: String) {
    info!("Server started as \"{}\"", instance_name);

    let (tx, _interrupt_tx) = match Interface::new().port(interface_port).connect() {
        Ok(interface) => interface.to_thread(),
        Err(interface) => {
            warn!("No hardware interface detected."); //Enabled fake interface since
            interface.to_thread()
        }
    };

    let mut stage = Parser::new(Stage::new(instance_name, tx)).parse();
    stage.load_config();

    // for fixture in stage.fixtures.iter_mut() {
    //     match fixture.channel_groups[0] {
    //         ChannelGroup::Single(ref mut group) => {
    //             group.activate_preheat(FadeCurve::Squared, 1000);
    //         },
    //         _ => {}
    //     }
    // }

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
                    let stage_locked = stage.lock().expect("Failed to lock Arc!");
                    let mut channel_locked = stage_locked.channels[address as usize].lock().expect("Failed to lock Arc!");
                    channel_locked.stop_fade();
                    channel_locked.set(value);
                }
                else if address_type == 1 {
                    // Switch
                    let mut stage_locked = stage.lock().expect("Failed to lock Arc!");
                    debug!("Set switch with address {:?} to {:?} (shifted: {:?})", address, value, shift);
                    if shift {
                        stage_locked.deactivate_group_of_switch(address as usize, true)
                    }
                    stage_locked.set_switch(address as usize, value as f64, true);
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
                    let stage_locked = stage.lock().expect("Failed to lock Arc!");
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
                    stream.read_to_string(&mut buffer).unwrap();
                    match FrontendData::from_json(buffer) {
                        Ok(data) => {
                            let mut stage_locked = stage.lock().expect("Failed to lock Arc!");
                            stage_locked.from_frontend_data(data);
                            stage_locked.save_config();
                            UDPSocket::new().start_frontend_client().send_to_multicast(&[255, 255, 255, 255]);
                        },
                        Err(e) => {error!("Failed to decode JSON received from client: {}", e.description());}
                    }
                });
            }
        }).join().unwrap();
    }
}