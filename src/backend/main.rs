#[macro_use] extern crate log;
extern crate env_logger;
extern crate structures;
extern crate net2;
extern crate rustc_serialize;

use std::time::Duration;
use std::thread::{self, sleep};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use std::io::prelude::*;
use std::env;

mod interface_handler;
use interface_handler::*;

use rustc_serialize::json;

use structures::*;

fn main() {
    println!("BitDMX backend v{}-{}", VERSION, GIT_HASH);
    if cfg!(feature = "local") { println!("HEUREKA") };

    env_logger::init().unwrap();

    let args: Vec<_> = env::args().collect();
    let instance_name = if args.len() > 1 {
        args[1].clone()
    }
    else {
        "Untitled".to_string()
    };

    println!("Server started as \"{}\"", instance_name);

    //let interface = Interface::new().port("/dev/tty.usbmodem40131".to_string()).connect();
    let interface = Interface::new().port("/dev/ttyACM0".to_string()).connect();
    if interface.is_err() { panic!(interface) }
    let (tx, _interrupt_tx) = interface.unwrap().to_thread();

    let mut stage = Parser::new(Stage::new(instance_name, tx)).parse();


    let mut v1 = HashMap::new();
    v1.insert((0, 0), ChannelGroupValue::from_tuple((vec![255], (FadeCurve::Squared, 1000), (FadeCurve::Linear, 1000))));
    v1.insert((1, 0), ChannelGroupValue::from_tuple((vec![255], (FadeCurve::Squared, 1000), (FadeCurve::Linear, 1000))));
    v1.insert((2, 0), ChannelGroupValue::from_tuple((vec![255], (FadeCurve::Squared, 1000), (FadeCurve::Linear, 1000))));
    stage.add_switch(Switch::new("RGB full on".to_string(), v1, "Full".to_string(), 3000));

    let mut v1 = HashMap::new();
    v1.insert((0, 0), ChannelGroupValue::from_tuple((vec![0], (FadeCurve::Squared, 1000), (FadeCurve::Linear, 1000))));
    v1.insert((1, 0), ChannelGroupValue::from_tuple((vec![0], (FadeCurve::Squared, 1000), (FadeCurve::Linear, 1000))));
    v1.insert((2, 0), ChannelGroupValue::from_tuple((vec![0], (FadeCurve::Squared, 1000), (FadeCurve::Linear, 1000))));
    stage.add_switch(Switch::new("Blackout".to_string(), v1, "Full".to_string(), 3000));

    let mut v2 = HashMap::new();
    v2.insert((0, 0), ChannelGroupValue::from_tuple((vec![20], (FadeCurve::Squared, 0), (FadeCurve::Linear, 0))));
    stage.add_switch(Switch::new("RED".to_string(), v2, "Single Colors".to_string(), 1000));

    let mut test_v = HashMap::new();
    test_v.insert((1, 0), ChannelGroupValue::from_tuple((vec![20], (FadeCurve::Squared, 0), (FadeCurve::Linear, 5000))));
    stage.add_switch(Switch::new("GREEN".to_string(), test_v, "Single Colors".to_string(), 1000));

    let mut test_v2 = HashMap::new();
    test_v2.insert((2, 0), ChannelGroupValue::from_tuple((vec![20], (FadeCurve::Squared, 500), (FadeCurve::Linear, 0))));
    stage.add_switch(Switch::new("BLUE".to_string(), test_v2, "Single Colors".to_string(), 800));


    for fixture in stage.fixtures.iter_mut() {
        match fixture.channel_groups[0] {
            ChannelGroup::Single(ref mut group) => {
                group.activate_preheat(FadeCurve::Squared, 1000);
            },
            _ => {}
        }
    }

   let socket = UDPSocket::new();
    socket.start_watchdog_server();
    let server = socket.start_backend_server(); //receiving updates (DMX values etc. from frontend)`

    let stage = Arc::new(Mutex::new(stage));

    {
        let stage = stage.clone();
        thread::spawn(move || {
            loop {
                let (d, _) = server.receive();
                debug!("{:?}", d);

                let address_type:u8 = d[0] & 127;
                let shift: bool = d[0] & 128 != 0;
                let address: u16 = ((d[1] as u16) << 8) + (d[2] as u16);
                let value: u8 = d[3];


                if address_type == 0 {
                    let stage_locked = stage.lock().unwrap();
                    let mut channel_locked = stage_locked.channels[address as usize].lock().unwrap();
                    channel_locked.stop_fade();
                    channel_locked.set(value);
                }
                else if address_type == 1 {
                    // Switch
                    let mut stage_locked = stage.lock().unwrap();
                    println!("Set switch with address {:?} to {:?} (shifted: {:?})", address, value, shift);
                    if shift {
                        stage_locked.deactivate_group_of_switch(address as usize)
                    }
                    stage_locked.set_switch(address as usize, value as f64);
                }
                else if address_type == 2 {
                    // Chaser
                    println!("Start chaser with switch with address {:?} to {:?} (shifted: {:?})", address, value, shift);
                    start_chaser_of_switch(stage.clone(), address as usize, value as f64);
                }
            }
        });
    }

    {
        let stage = stage.clone();
        thread::spawn(move || {
            sleep(Duration::from_millis(10000));
            let mut stage_locked = stage.lock().unwrap();
            let mut test_v2 = HashMap::new();
            test_v2.insert((1, 0), ChannelGroupValue::from_tuple((vec![20], (FadeCurve::Squared, 1000), (FadeCurve::Linear, 1000))));
            test_v2.insert((2, 0), ChannelGroupValue::from_tuple((vec![20], (FadeCurve::Squared, 1000), (FadeCurve::Linear, 1000))));
            stage_locked.add_switch(Switch::new("CYAN".to_string(), test_v2, "Multi Color".to_string(), 3000));
            UDPSocket::new().start_frontend_client().send_to_multicast(&[255, 255, 255, 255]);
        });
    }

    {
        let stage = stage.clone();
        thread::spawn(move || {
            use std::io::Write;
            use std::net::TcpListener;

            let listener = TcpListener::bind("0.0.0.0:8000").unwrap();
            info!("listening (send) started, ready to accept");
            for stream in listener.incoming() {
                let stage = stage.clone();
                thread::spawn(move || {
                    let stage_locked = stage.lock().unwrap();
                    let mut stream = stream.unwrap();

                    stream.write(json::encode(&stage_locked.get_frontend_data()).unwrap().as_bytes()).unwrap();
                });
            }
        });
    }

    {
        let stage = stage.clone();
        thread::spawn(move || {
            use std::net::TcpListener;

            let listener = TcpListener::bind("0.0.0.0:8001").unwrap();
            info!("listening (receive) started, ready to accept");
            for stream in listener.incoming() {
                let stage = stage.clone();
                thread::spawn(move || {
                    let mut stream = stream.unwrap();
                    let mut buffer = String::new();
                    let _ = stream.read_to_string(&mut buffer);
                    let frontend_data: FrontendData = json::decode(&buffer).unwrap();
                    {stage.lock().unwrap().from_frontend_data(frontend_data);}
                    //TODO: receive data and update stage
                    UDPSocket::new().start_frontend_client().send_to_multicast(&[255, 255, 255, 255]);
                    //stream.write(json::encode(&stage_locked.get_frontend_data()).unwrap().as_bytes()).unwrap();
                });
            }
        }).join().unwrap();
    }
}


#[test]
fn test_fade_curve() {
    use std::time::Duration;
    use std::thread::sleep;
    use interface_handler::*;
    use structures::*;

    let interface = Interface::new().connect();
    if interface.is_err() { panic!(interface) }
    let (tx, interrupt_tx) = interface.unwrap().to_thread();

    //let curve = FadeCurve::Custom("-cos(1.5*6.28318530718*x)*0.5+0.5".to_string());
    let curve = FadeCurve::Squared;
    let curve = FadeCurve::Sin(0);
    let mut stage = Stage::new(tx);


    let mut test_group = ChannelGroup::Single(Single::new(stage.get_channel_object(1)));
    // let mut test_group = ChannelGroup::Moving2D(Moving2D::new(stage.get_channel_object(1), stage.get_channel_object(2)));
    // let mut test_group = ChannelGroup::RGB(RGB::new(stage.get_channel_object(1), stage.get_channel_object(2), stage.get_channel_object(3)));
    //let test_group = ChannelGroup::RGBA(RGBA::new(stage.get_channel_object(1), stage.get_channel_object(2), stage.get_channel_object(3), stage.get_channel_object(4)));

    match test_group {
        ChannelGroup::Single(mut group) => {
            group.fade_simple(curve.clone(), 5000, 255);
        },
        ChannelGroup::RGB(mut group) => {
            group.fade_rgb(curve.clone(), 1000, 255, 0, 0);
            sleep(Duration::from_millis(1000));
            group.fade_rgb(curve.clone(), 1000, 0, 255, 0);
            sleep(Duration::from_millis(1000));
            group.fade_rgb(curve.clone(), 1000, 0, 0, 255);
            sleep(Duration::from_millis(1000));
        },
        ChannelGroup::RGBA(mut group) => {
            group.fade_rgb(curve.clone(), 1000, 255, 0, 0, 1);
            sleep(Duration::from_millis(1000));
            group.fade_rgb(curve.clone(), 1000, 0, 255, 0, 1);
            sleep(Duration::from_millis(1000));
            group.fade_rgb(curve.clone(), 1000, 0, 0, 255, 1);
            sleep(Duration::from_millis(1000));
        },
        ChannelGroup::Moving2D(mut group) => {
            group.fade_simple(curve.clone(), 1000, 255, 255);
            sleep(Duration::from_millis(1000));
            group.fade_simple(curve.clone(), 1000, 0, 0);
        }//,
        //_ => {}
    }


    sleep(Duration::from_millis(500));
    println!("Disconnecting...");
    interrupt_tx.send(true).unwrap();
}
