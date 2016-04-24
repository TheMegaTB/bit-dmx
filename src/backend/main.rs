extern crate structures;

mod interface_handler;

use interface_handler::*;
use structures::*;
use std::sync::mpsc;

fn main() {

}

// pub fn connect_and_test() {
//     let interface = Interface::new().connect();
//     if interface.is_err() { panic!(interface) }
//     let (tx, interrupt_tx) = interface.unwrap().to_thread();
//     (0..255).chain((0..255).rev()).map(|i| {
//         tx.send((1, i)).unwrap();
//         tx.send((2, i)).unwrap();
//         //tx.send((3, i)).unwrap();
//         sleep(Duration::from_millis(16));
//         // interface.write_to_dmx(1, i);
//         // println!("{}", i);
//     }).collect::<Vec<_>>();
//     sleep(Duration::from_millis(2000));
//     println!("Disconnecting...");
//     interrupt_tx.send(true).unwrap();
// }


#[test]
//#[should_panic]
fn test_fade_curve() {
    use std::time::Duration;
    use std::thread::sleep;

    let interface = Interface::new().connect();
    if interface.is_err() { panic!(interface) }
    let (tx, interrupt_tx) = interface.unwrap().to_thread();
    tx.send((1, 0)).unwrap();

    let curve = FadeCurve::Custom("sin(10*x)".to_string());
    //let curve = FadeCurve::Squared;
    let mut stage = Stage::new();
    let mut test_group = ChannelGroup::Single(Single::new(1, tx.clone()));
    // let test_fixture = Fixture::new(vec![test_group]);
    // stage.add_fixture(test_fixture);

    match test_group {
        ChannelGroup::Single(mut group) => {
            group.fade(curve.clone(), 10000, 255);
        },
        _ => {}
    }


    sleep(Duration::from_millis(6000));
    println!("Disconnecting...");
    interrupt_tx.send(true).unwrap();
}
