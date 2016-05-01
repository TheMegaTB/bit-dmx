#[macro_use] extern crate log;
extern crate env_logger;
extern crate structures;
extern crate net2;
extern crate rustc_serialize;

use std::time::Duration;
use std::thread::{self, sleep};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

mod interface_handler;
use interface_handler::*;

use rustc_serialize::json;

use structures::*;

fn main() {
    println!("BitDMX backend v{}-{}", VERSION, GIT_HASH);

    env_logger::init().unwrap();

    let interface = Interface::new().port("/dev/tty.usbmodem40131".to_string()).connect();
    if interface.is_err() { panic!(interface) }
    let (tx, _interrupt_tx) = interface.unwrap().to_thread();
    let mut stage = Parser::new(Stage::new(tx)).parse();

    let mut v1 = HashMap::new();
    v1.insert((0, 0), (vec![255], (FadeCurve::Squared, 1000), (FadeCurve::Linear, 1000)));
    v1.insert((1, 0), (vec![255], (FadeCurve::Squared, 1000), (FadeCurve::Linear, 1000)));
    v1.insert((2, 0), (vec![255], (FadeCurve::Squared, 1000), (FadeCurve::Linear, 1000)));
    stage.add_switch(Switch::new("RGB full on".to_string(), v1, "Full".to_string(), 3000));

    let mut v1 = HashMap::new();
    v1.insert((0, 0), (vec![0], (FadeCurve::Squared, 1000), (FadeCurve::Linear, 1000)));
    v1.insert((1, 0), (vec![0], (FadeCurve::Squared, 1000), (FadeCurve::Linear, 1000)));
    v1.insert((2, 0), (vec![0], (FadeCurve::Squared, 1000), (FadeCurve::Linear, 1000)));
    stage.add_switch(Switch::new("Blackout".to_string(), v1, "Full".to_string(), 3000));

    let mut v2 = HashMap::new();
    v2.insert((0, 0), (vec![20], (FadeCurve::Squared, 1000), (FadeCurve::Linear, 1000)));
    stage.add_switch(Switch::new("RED".to_string(), v2, "Single Colors".to_string(), 3000));

    let mut test_v = HashMap::new();
    test_v.insert((1, 0), (vec![20], (FadeCurve::Squared, 1000), (FadeCurve::Linear, 1000)));
    stage.add_switch(Switch::new("GREEN".to_string(), test_v, "Single Colors".to_string(), 3000));

    let mut test_v2 = HashMap::new();
    test_v2.insert((2, 0), (vec![20], (FadeCurve::Squared, 1000), (FadeCurve::Linear, 1000)));
    stage.add_switch(Switch::new("BLUE".to_string(), test_v2, "Single Colors".to_string(), 3000));


    for fixture in stage.fixtures.iter_mut() {
        match fixture.channel_groups[0] {
            ChannelGroup::Single(ref mut group) => {
                group.activate_preheat(FadeCurve::Squared, 1000);
            },
            _ => {}
        }
    }

    // let data = stage.get_frontend_data();
    //
    // println!("{:?}", (json::encode(&stage.get_frontend_data()).unwrap()));

    // stage.set_switch(s2, 100.0);
    // sleep(Duration::from_millis(2500));
    // stage.set_switch(s2, 255.0);
    // sleep(Duration::from_millis(2500));
    // stage.set_switch(s2, 0.0);


    // stage.set_switch(s1, 255.0);
    // stage.set_switch(s2, 255.0);
    // sleep(Duration::from_millis(2500));
    // stage.deactivate_group_of_switch(s3);
    // stage.set_switch(s3, 255.0);

    let stage = Arc::new(Mutex::new(stage));

    let socket = UDPSocket::new();
    socket.start_watchdog_server();
    let server = socket.start_backend_server(); //receiving updates (DMX values etc. from frontend)

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

                let mut stage_locked = stage.lock().unwrap();
                if address_type == 0 {
                    let mut channel_locked = stage_locked.channels[address as usize].lock().unwrap();
                    channel_locked.stop_fade();
                    channel_locked.set(value);
                }
                else if address_type == 1 {
                    // Switch
                    println!("Set switch with address {:?} to {:?} (shifted: {:?})", address, value, shift);
                    if shift {
                        for switch_id in stage_locked.deactivate_group_of_switch(address as usize) {
                            println!("{:?}", switch_id);
                            let addr_high = (switch_id >> 8) as u8;
                            let addr_low = switch_id as u8;
                            server.send_to_multicast(&[1, addr_high, addr_low, 0]);
                        }
                    }
                    stage_locked.set_switch(address as usize, value as f64);
                }
                println!("{:?}, {:?}", address, value);

                server.send_to_multicast(&d); //TODO: remove this and place ist in set switch etc. so that set switch can be called from a different thread (e.g. from chaser thread)
            }
        });
    }

    {
        let stage = stage.clone();
        thread::spawn(move || {
            sleep(Duration::from_millis(10000));
            let mut stage_locked = stage.lock().unwrap();
            let mut test_v2 = HashMap::new();
            test_v2.insert((1, 0), (vec![20], (FadeCurve::Squared, 1000), (FadeCurve::Linear, 1000)));
            test_v2.insert((2, 0), (vec![20], (FadeCurve::Squared, 1000), (FadeCurve::Linear, 1000)));
            stage_locked.add_switch(Switch::new("CYAN".to_string(), test_v2, "Multi Color".to_string(), 3000));
            UDPSocket::new().start_frontend_client().send_to_multicast(&[255, 255, 255, 255]); //TODO make this server public (see todo above)
        });
    }

    {
        let stage = stage.clone();
        thread::spawn(move || {
            use std::io::Write;
            use std::net::TcpListener;

            let listener = TcpListener::bind("0.0.0.0:8000").unwrap();
            info!("listening started, ready to accept");
            for stream in listener.incoming() {
                let stage = stage.clone();
                thread::spawn(move || {
                    let stage_locked = stage.lock().unwrap();
                    let mut stream = stream.unwrap();
                    //TODO: receive data and update stage

                    stream.write(json::encode(&stage_locked.get_frontend_data()).unwrap().as_bytes()).unwrap();
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
            //group.fade_simple(curve.clone(), 500, 255);
            //sleep(Duration::from_millis(2000));
            //group.activate_preheat(curve.clone(), 500);
            //println!("pre");
            group.fade_simple(curve.clone(), 5000, 255);
            //group.deactivate_preheat(curve.clone(), 500);
            //sleep(Duration::from_millis(2000));
            //group.fade_simple(curve.clone(), 1000, 0);
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
