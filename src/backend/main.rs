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

    let interface = Interface::new().connect();
    if interface.is_err() { panic!(interface) }
    let (tx, _interrupt_tx) = interface.unwrap().to_thread();
    let mut stage = Parser::new(Stage::new(tx)).parse();

    //UMBAULIGHT
    let mut umbaulight_values = HashMap::new();
    umbaulight_values.insert((16, 0), (vec![0, 0, 1], FadeCurve::Squared, 200));
    umbaulight_values.insert((17, 0), (vec![0, 0, 1], FadeCurve::Squared, 200));
    umbaulight_values.insert((18, 0), (vec![0, 0, 1], FadeCurve::Squared, 200));
    umbaulight_values.insert((19, 0), (vec![0, 0, 1], FadeCurve::Squared, 200));
    stage.add_switch(ValueCollection::new(umbaulight_values));


    //SCENE 1
    let mut scene1_1_values = HashMap::new();
    scene1_1_values.insert((10, 0), (vec![255], FadeCurve::Squared, 3000));
    scene1_1_values.insert((2, 0), (vec![255], FadeCurve::Squared, 3000));
    stage.add_switch(ValueCollection::new(scene1_1_values));

    let mut scene1_2_values = HashMap::new();
    scene1_2_values.insert((1, 0), (vec![127], FadeCurve::Squared, 3000));
    scene1_2_values.insert((3, 0), (vec![255], FadeCurve::Squared, 3000));
    scene1_2_values.insert((6, 0), (vec![255], FadeCurve::Squared, 3000));
    scene1_2_values.insert((15, 0), (vec![50], FadeCurve::Squared, 3000));
    stage.add_switch(ValueCollection::new(scene1_2_values));


    //SCENE 2
    let mut scene2_1_values = HashMap::new();
    scene2_1_values.insert((1, 0), (vec![127], FadeCurve::Squared, 3000));
    scene2_1_values.insert((5, 0), (vec![255], FadeCurve::Squared, 3000));
    scene2_1_values.insert((7, 0), (vec![255], FadeCurve::Squared, 3000));
    stage.add_switch(ValueCollection::new(scene2_1_values));

    let mut scene2_2_values = HashMap::new();
    scene2_2_values.insert((1, 0), (vec![255], FadeCurve::Squared, 3000));
    scene2_2_values.insert((2, 0), (vec![255], FadeCurve::Squared, 3000));
    stage.add_switch(ValueCollection::new(scene2_2_values));

    let mut scene2_3_values = HashMap::new();
    scene2_3_values.insert((3, 0), (vec![255], FadeCurve::Squared, 3000));
    stage.add_switch(ValueCollection::new(scene2_3_values));


    //SCENE 3
    let mut scene3_1_values = HashMap::new();
    scene3_1_values.insert((3, 0), (vec![255], FadeCurve::Squared, 3000));
    scene3_1_values.insert((4, 0), (vec![140], FadeCurve::Squared, 3000));
    stage.add_switch(ValueCollection::new(scene3_1_values));

    let mut scene3_2_values = HashMap::new();
    scene3_2_values.insert((5, 0), (vec![255], FadeCurve::Squared, 3000));
    // scene3_2_values.insert((3, 0), (vec![0], FadeCurve::Squared, 3000));
    // scene3_2_values.insert((4, 0), (vec![0], FadeCurve::Squared, 3000));
    stage.add_switch(ValueCollection::new(scene3_2_values));

    let mut scene3_3_values = HashMap::new();
    scene3_3_values.insert((7, 0), (vec![255], FadeCurve::Squared, 3000));
    // scene3_3_values.insert((5, 0), (vec![0], FadeCurve::Squared, 3000));
    stage.add_switch(ValueCollection::new(scene3_3_values));


    //SCENE 4
    let mut scene4_1_values = HashMap::new();
    scene4_1_values.insert((10, 0), (vec![127], FadeCurve::Squared, 3000));
    scene4_1_values.insert((11, 0), (vec![127], FadeCurve::Squared, 3000));
    scene4_1_values.insert((12, 0), (vec![64], FadeCurve::Squared, 3000));
    scene4_1_values.insert((14, 0), (vec![255], FadeCurve::Squared, 3000));
    scene4_1_values.insert((15, 0), (vec![74], FadeCurve::Squared, 3000));
    scene4_1_values.insert((1, 0), (vec![192], FadeCurve::Squared, 3000));
    scene4_1_values.insert((2, 0), (vec![165], FadeCurve::Squared, 3000));
    scene4_1_values.insert((4, 0), (vec![140], FadeCurve::Squared, 3000));
    scene4_1_values.insert((6, 0), (vec![127], FadeCurve::Squared, 3000));
    stage.add_switch(ValueCollection::new(scene4_1_values));


    //SCENE 5
    let mut scene5_1_values = HashMap::new();
    scene5_1_values.insert((10, 0), (vec![255], FadeCurve::Squared, 3000));
    scene5_1_values.insert((11, 0), (vec![255], FadeCurve::Squared, 3000));
    scene5_1_values.insert((14, 0), (vec![160], FadeCurve::Squared, 3000));
    scene5_1_values.insert((15, 0), (vec![100], FadeCurve::Squared, 3000));
    stage.add_switch(ValueCollection::new(scene5_1_values));

    let mut scene5_2_values = HashMap::new();
    scene5_2_values.insert((10, 0), (vec![100], FadeCurve::Squared, 3000));
    scene5_2_values.insert((11, 0), (vec![100], FadeCurve::Squared, 3000));
    scene5_2_values.insert((16, 0), (vec![0, 255, 63], FadeCurve::Squared, 20000));
    scene5_2_values.insert((17, 0), (vec![0, 255, 63], FadeCurve::Squared, 20000));
    scene5_2_values.insert((18, 0), (vec![0, 255, 63], FadeCurve::Squared, 20000));
    scene5_2_values.insert((19, 0), (vec![0, 255, 63], FadeCurve::Squared, 20000));
    stage.add_switch(ValueCollection::new(scene5_2_values));

    let mut scene5_3_values = HashMap::new();
    scene5_3_values.insert((10, 0), (vec![0], FadeCurve::Squared, 3000));
    scene5_3_values.insert((11, 0), (vec![0], FadeCurve::Squared, 3000));
    scene5_3_values.insert((14, 0), (vec![50], FadeCurve::Squared, 3000));
    scene5_3_values.insert((15, 0), (vec![50], FadeCurve::Squared, 3000));
    scene5_3_values.insert((1, 0), (vec![50], FadeCurve::Squared, 3000));
    scene5_3_values.insert((2, 0), (vec![50], FadeCurve::Squared, 3000));
    scene5_3_values.insert((4, 0), (vec![127], FadeCurve::Squared, 3000));
    stage.add_switch(ValueCollection::new(scene5_3_values));


    //SCENE 6
    let mut scene6_values = HashMap::new();
    scene6_values.insert((1, 0), (vec![133], FadeCurve::Squared, 3000));
    scene6_values.insert((2, 0), (vec![150], FadeCurve::Squared, 3000));
    scene6_values.insert((3, 0), (vec![160], FadeCurve::Squared, 3000));
    scene6_values.insert((4, 0), (vec![185], FadeCurve::Squared, 3000));
    scene6_values.insert((16, 0), (vec![0, 50, 50], FadeCurve::Squared, 500));
    scene6_values.insert((17, 0), (vec![0, 50, 50], FadeCurve::Squared, 500));
    scene6_values.insert((18, 0), (vec![0, 50, 50], FadeCurve::Squared, 500));
    scene6_values.insert((19, 0), (vec![0, 50, 50], FadeCurve::Squared, 500));
    stage.add_switch(ValueCollection::new(scene6_values));


    //SCENE 7
    let mut scene7_values = HashMap::new();
    scene7_values.insert((9, 0), (vec![255], FadeCurve::Squared, 3000));
    scene7_values.insert((10, 0), (vec![255], FadeCurve::Squared, 3000));
    stage.add_switch(ValueCollection::new(scene7_values));

    //SCENE 8
    let mut scene8_1_values = HashMap::new();
    scene8_1_values.insert((11, 0), (vec![255], FadeCurve::Squared, 3000));
    scene8_1_values.insert((15, 0), (vec![255], FadeCurve::Squared, 3000));
    scene8_1_values.insert((8, 0), (vec![255], FadeCurve::Squared, 3000));
    scene8_1_values.insert((9, 0), (vec![255], FadeCurve::Squared, 3000));
    scene8_1_values.insert((12, 0), (vec![255], FadeCurve::Squared, 3000));
    scene8_1_values.insert((13, 0), (vec![255], FadeCurve::Squared, 3000));
    scene8_1_values.insert((0, 0), (vec![255], FadeCurve::Squared, 3000));
    scene8_1_values.insert((3, 0), (vec![191], FadeCurve::Squared, 3000));
    scene8_1_values.insert((5, 0), (vec![127], FadeCurve::Squared, 3000));
    scene8_1_values.insert((16, 0), (vec![0, 191, 64], FadeCurve::Squared, 3000));
    scene8_1_values.insert((17, 0), (vec![0, 191, 64], FadeCurve::Squared, 3000));
    scene8_1_values.insert((18, 0), (vec![0, 64, 191], FadeCurve::Squared, 3000));
    scene8_1_values.insert((19, 0), (vec![0, 64, 191], FadeCurve::Squared, 3000));
    stage.add_switch(ValueCollection::new(scene8_1_values));

    let mut scene8_2_values = HashMap::new();
    scene8_2_values.insert((0, 0), (vec![0], FadeCurve::Squared, 3000));
    scene8_2_values.insert((5, 0), (vec![0], FadeCurve::Squared, 3000));
    stage.add_switch(ValueCollection::new(scene8_2_values));


    //SCENE 15 - Besuch bei Hora
    let mut scene15_values = HashMap::new();
    scene15_values.insert((8, 0), (vec![127], FadeCurve::Squared, 3000));
    scene15_values.insert((9, 0), (vec![127], FadeCurve::Squared, 3000));
    scene15_values.insert((10, 0), (vec![64], FadeCurve::Squared, 3000));
    scene15_values.insert((12, 0), (vec![127], FadeCurve::Squared, 3000));
    scene15_values.insert((13, 0), (vec![127], FadeCurve::Squared, 3000));
    scene15_values.insert((14, 0), (vec![64], FadeCurve::Squared, 3000));
    scene15_values.insert((1, 0), (vec![192], FadeCurve::Squared, 3000));
    scene15_values.insert((2, 0), (vec![165], FadeCurve::Squared, 3000));
    scene15_values.insert((4, 0), (vec![140], FadeCurve::Squared, 3000));
    scene15_values.insert((6, 0), (vec![127], FadeCurve::Squared, 3000));
    scene15_values.insert((16, 0), (vec![90, 10, 90], FadeCurve::Squared, 10000));
    scene15_values.insert((18, 0), (vec![90, 10, 90], FadeCurve::Squared, 10000));
    stage.add_switch(ValueCollection::new(scene15_values));

    //SCENE 15.1 - Stundenblumen
    let mut scene15_1_values = HashMap::new();
    // scene15_1_values.insert((20, 0), (vec![255], FadeCurve::Squared, 500));
    // scene15_1_values.insert((20, 1), (vec![255], FadeCurve::Squared, 500));
    scene15_1_values.insert((16, 0), (vec![150, 150, 0], FadeCurve::Squared, 10000));
    scene15_1_values.insert((18, 0), (vec![150, 150, 0], FadeCurve::Squared, 10000));
    stage.add_switch(ValueCollection::new(scene15_1_values));

    //let mut test_values2 = HashMap::new();
    //test_values1.insert((2, 0), (vec![255, 255, 255, 100], FadeCurve::Squared, 1000));
    //test_values2.insert((0, 1), (vec![255], FadeCurve::Squared, 1000));
    //let test_switch2 = ValueCollection::new(test_values2);
    //let id2 = stage.add_switch(test_switch2);

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
                stage.channels[address as usize].lock().unwrap().set(value);
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
