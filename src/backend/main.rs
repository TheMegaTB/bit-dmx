extern crate structures;
extern crate net2;

mod interface_handler;

use structures::*;

fn main() {
    let socket = UDPSocket::new();
    socket.start_watchdog_server();
    let server = socket.start_backend_server(); //receiving updates (DMX values etc. from frontend)

    let (d, _) = server.receive();
    //do something with the data
    server.send_to_multicast(&d);

    // server.send_to_multicast(&[1, 2, 3, 4, 5, 6, 7, 8]);
    // println!("{:?}", server.receive());
    // std::thread::sleep(std::time::Duration::from_secs(3000));
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
    tx.send((1, 0)).unwrap();

    //let curve = FadeCurve::Custom("-cos(1.5*6.28318530718*x)*0.5+0.5".to_string());
    let curve = FadeCurve::Squared;
    // let curve = FadeCurve::SquareRoot;
    let mut stage = Stage::new();
    let mut test_group = ChannelGroup::Single(Single::new(1, tx.clone()));
    // let test_fixture = Fixture::new(vec![test_group]);
    // stage.add_fixture(test_fixture);

    match test_group {
        ChannelGroup::Single(mut group) => {
            group.fade(curve.clone(), 500, 255);
            sleep(Duration::from_millis(1000));
            group.fade(curve.clone(), 500, 0);
        },
        _ => {}
    }


    sleep(Duration::from_millis(500));
    println!("Disconnecting...");
    interrupt_tx.send(true).unwrap();
}
