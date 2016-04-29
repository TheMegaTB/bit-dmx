#[macro_use] extern crate log;
extern crate env_logger;
extern crate structures;
extern crate net2;

use std::time::Duration;
use std::thread::{self, sleep};
use std::collections::HashMap;

mod interface_handler;
use interface_handler::*;

use structures::*;

fn main() {
    env_logger::init().unwrap();

    let interface = Interface::new().port("/dev/tty.usbmodem40131".to_string()).connect();
    if interface.is_err() { panic!(interface) }
    let (tx, _interrupt_tx) = interface.unwrap().to_thread();
    let mut stage = Parser::new(Stage::new(tx)).parse();

    for fixture in stage.fixtures.iter_mut() {
        match fixture.channel_groups[0] {
            ChannelGroup::Single(ref mut group) => {
                group.activate_preheat(FadeCurve::Squared, 1000);
                sleep(Duration::from_millis(1500));
                group.fade_simple(FadeCurve::Linear, 5000, 255);
                sleep(Duration::from_millis(2500));
                group.fade_simple(FadeCurve::Linear, 2500, 0);
            },
            ChannelGroup::RGB(ref mut group) => {
                group.fade_simple(FadeCurve::Linear, 2500, 255, 255, 0);
                sleep(Duration::from_millis(2500));
                group.fade_simple(FadeCurve::Linear, 2500, 0, 0, 255);
            }
            _ => {}
        }
    }

    let socket = UDPSocket::new();
    socket.start_watchdog_server();
    let server = socket.start_backend_server(); //receiving updates (DMX values etc. from frontend)


    thread::spawn(move || {
        loop {
            let (d, _) = server.receive();
            debug!("{:?}", d); //TODO: do something with the data that isn't completely useless

            let address_type:u8 = d[0] & (2u8.pow(7)-1);
            let shift: bool = d[0] & (2u8.pow(7)) != 0;
            let address: u16 = ((d[1] as u16) << 8) + (d[2] as u16);
            let value: u8 = d[3];

            if address_type == 0 {
                let mut channel_locked = stage.channels[address as usize].lock().unwrap();
                channel_locked.stop_fade();
                channel_locked.set(value);
            }
            else if address_type == 1 {
                // Switch
                println!("Set switch with address {:?} to {:?} (shifted: {:?})", address, value, shift);
                if value == 0 {
                    stage.deactivate_switch(address as usize);
                }
                else {
                    stage.activate_switch(address as usize, value as f64);
                }
            }
            println!("{:?}, {:?}", address, value);



            //stage.fixtures.push();
            server.send_to_multicast(&d);
        }
    });

    thread::spawn(|| {
        use std::io::Write;
        use std::net::TcpListener;

        let listener = TcpListener::bind("0.0.0.0:8000").unwrap();
        info!("listening started, ready to accept");
        for stream in listener.incoming() {
            thread::spawn(|| {
                let mut stream = stream.unwrap();
                stream.write(b"Hello World\r\n").unwrap();
            });
        }
    }).join().unwrap();
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
