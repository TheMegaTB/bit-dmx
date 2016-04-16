#![allow(dead_code)]
#![allow(unused_imports)]
#[macro_use] extern crate conrod;
extern crate piston_window;
extern crate find_folder;
extern crate serial;
extern crate rand;

use conrod::{
    color,
    Button,
    Canvas,
    Circle,
    Color,
    Colorable,
    DropDownList,
    EnvelopeEditor,
    Frameable,
    Labelable,
    NumberDialer,
    Point,
    Positionable,
    Slider,
    Sizeable,
    Text,
    TextBox,
    Theme,
    Toggle,
    Widget,
    WidgetMatrix,
    XYPad,
};
use piston_window::{ EventLoop, Glyphs, PistonWindow, UpdateEvent, WindowSettings };
use std::sync::{Arc, Mutex, mpsc};
use std::thread::{sleep, self};
use std::time::Duration;
use std::net::{ UdpSocket, Ipv4Addr, SocketAddr, SocketAddrV4 };

#[cfg(windows)]
const PORT: &'static str = "COM1";
#[cfg(unix)]
const PORT: &'static str = "/dev/ttyACM0";

#[cfg(feature = "DMX256")]
const DMX_CHANNELS: usize = 256;
#[cfg(feature = "DMX512")]
const DMX_CHANNELS: usize = 512;

type Backend = (<piston_window::G2d<'static> as conrod::Graphics>::Texture, Glyphs);
type Ui = conrod::Ui<Backend>;
type UiCell<'a> = conrod::UiCell<'a, Backend>;

widget_ids! {
    CANVAS,
    TITLE,
    CHANNEL_VALUE with DMX_CHANNELS,
    CHANNEL_FADER with DMX_CHANNELS,
    MODE_SELECT with DMX_CHANNELS
}

#[derive(Clone, Debug)]
enum Mode {
    LTP,
    HTP,
    ADD,
    SUB,
    DEFAULT
}

impl Mode {
	fn string(&self) -> String {
		match *self {
		    Mode::LTP       => "LTP".to_string(),
    		Mode::HTP       => "HTP".to_string(),
            Mode::ADD       => "ADD".to_string(),
            Mode::SUB       => "SUB".to_string(),
            Mode::DEFAULT   => "DEFAULT".to_string()
		}
	}
}

#[derive(Debug)]
struct Value {
    value: u8,
    mode: Mode
}

impl Value {
    fn new(mode: Mode, value: u8) -> Value {
        Value {
            value: value,
            mode: mode
        }
    }
    fn on(mode: Mode) -> Value {
        Value {
            value: 255,
            mode: mode
        }
    }
    fn off(mode: Mode) -> Value {
        Value {
            value: 0,
            mode: mode
        }
    }
}

enum Type {
    Single,
    RGB,
}

struct Channel {
    out_patch: usize,
    value: u8,
    name: String,
    if_tx: mpsc::Sender<(usize, u8)>,
    default_mode: Mode,
    mode_selected_idx: Option<usize>,
    fader_value: u8,
    fader_enabled: bool
    //TODO: Store active 'scenes'/pending values with ID's here
}

impl Channel {
    fn new(patch: usize, name: String, if_tx: mpsc::Sender<(usize, u8)>) -> Channel {
        Channel {
            out_patch: patch,
            value: 0,
            name: name,
            if_tx: if_tx,
            default_mode: Mode::LTP,
            mode_selected_idx: None,//Some(0),
            fader_value: 0,
            fader_enabled: false
        }
    }

    fn set(&mut self, v: Value) {
        let value = v.value;
        let mode = if let Mode::DEFAULT = v.mode { self.default_mode.clone() } else { v.mode };
        print!("Setting DMX channel {} to {} using mode {:?}", self.out_patch, value, mode);
        match mode {
            Mode::LTP       => self.value = value,
            Mode::HTP       => {
                if self.value < value { self.value = value }
            },
            Mode::ADD       => self.value = self.value.saturating_add(value),
            Mode::SUB       => self.value = self.value.saturating_sub(value),
            Mode::DEFAULT   => self.value = value
        }
        println!(" => {}", self.value);
        //TODO: Send stuff to DMX Interface.
        //send_to_interface(self.out_patch, self.value);
        self.if_tx.send((self.out_patch, self.value)).unwrap();
    }
}

struct Fixture {
    channels: Vec<Channel>,
    name: String
}

impl Fixture {
    fn new_with_type(t: Type, name: String, start_channel: usize, if_tx: mpsc::Sender<(usize, u8)>) -> Fixture {
        match t {
            Type::Single => {
                Fixture {
                    channels: vec![ Channel::new(start_channel, "".to_string(), if_tx) ],
                    name: name
                }
            },
            Type::RGB => {
                Fixture {
                    channels: (0..3).map(|i| {
                        match i {
                            0 => { Channel::new(start_channel + i, "R".to_string(), if_tx.clone()) }
                            1 => { Channel::new(start_channel + i, "G".to_string(), if_tx.clone()) }
                            _ => { Channel::new(start_channel + i, "B".to_string(), if_tx.clone()) }
                        }
                    }).collect(),
                    name: name
                }
            }
        }
    }

    fn new(start_channel: usize, end_channel: usize, name: String, if_tx: mpsc::Sender<(usize, u8)>) -> Fixture {
        let delta: usize = ( (end_channel as i16) - (start_channel as i16) ).abs() as usize;
        Fixture {
            channels: (0..delta+1).map(|channel| {
                Channel::new(start_channel + channel, channel.to_string(), if_tx.clone())
            }).collect::<Vec<_>>(),
            name: name
        }
    }
}

struct DMXApp {
    bg_color: Color,
    frame_width: f64,
    output_height: f64,
    fixtures: Vec<Fixture>,
    interface: mpsc::Sender<(usize, u8)>
}

impl DMXApp {
    fn new(if_tx: mpsc::Sender<(usize, u8)>) -> DMXApp {
        DMXApp {
            bg_color: color::rgb(0.236, 0.239, 0.241),
            frame_width: 1.0,
            output_height: 230.0,
            fixtures: Vec::new(),
            interface: if_tx
        }
    }
}

// fn fade(app: &mut DMXApp, channel: usize, target: u8) {
//     let current = app.channels[channel];
//     let delta = (target as i16) - (current as i16);
//     println!("cur: {}, target: {}, delta: {}", current, target, delta);
//     for _ in 0..delta.abs() {
//         sleep(Duration::new(0, 10000));
//         if delta.is_negative() && current != 0 {
//             app.channels[channel] = app.channels[channel].saturating_sub(1);
//         } else {
//             app.channels[channel] = app.channels[channel].saturating_add(1);
//         };
//     }
// }

fn set_widgets(ui: &mut UiCell, app: &mut DMXApp) {
    Canvas::new()
        .frame(app.frame_width)
        .pad(30.0)
        .color(app.bg_color)
        .scroll_kids()
        .set(CANVAS, ui);
    Text::new("DMX Out")
        .top_left_with_margin_on(CANVAS, 0.0)
        .font_size(32)
        .color(app.bg_color.plain_contrast())
        .set(TITLE, ui);

    let mut i = 0;
    let mut j = false;
    let mut id = CHANNEL_FADER;
    for fixture in app.fixtures.iter_mut() {
        for channel in fixture.channels.iter_mut() {
            let mut fader_enabled = channel.fader_enabled;

            if !fader_enabled { channel.fader_value = channel.value };
            let value = channel.fader_value as f32;
            let label = if value > 0.0 { format!("{:.*}", 0, value) } else { format!("") };

            Slider::new(value, 0.0, 255.0)
                .and(|slider| {
                    if i > 0 && j {
                        slider.right_from(id, 5.0)
                    } else if i > 0 && !j {
                        slider.right_from(id, 0.0)
                    } else { slider.down(25.0) }

                })
                .w_h(if fader_enabled { 30.0 } else { 40.0 }, app.output_height)
                .color(color::rgb(0.55, 0.1, 0.1))
                .frame(app.frame_width)
                .label(&label)
                .label_color(color::WHITE)
                .enabled(fader_enabled)
                .react(|v| {
                    let v = v as u8;
                    if v != channel.fader_value {
                        channel.fader_value = v;
                        channel.set(Value::new(Mode::DEFAULT, v));
                    }
                })
                .set(CHANNEL_FADER + i, ui);
            id = CHANNEL_FADER + i;

            if fader_enabled {
                id = CHANNEL_VALUE + i;
                Slider::new(channel.value as f32, 0.0, 255.0)
                    .right_from(CHANNEL_FADER + i, 0.0)
                    .w_h(10.0, app.output_height)
                    .color(color::rgb(0.1, 0.55, 0.1))
                    .enabled(false)
                    .frame(app.frame_width)
                    .react(|v| {let _: f32 = v;})
                    .set(CHANNEL_VALUE + i, ui);
            }

            let label = if !fader_enabled { "A".to_string() } else { channel.default_mode.string() };
            let mut dmode = channel.default_mode.clone();
            let mut modes = vec!["A".to_string(), "LTP".to_string(), "HTP".to_string(), "ADD".to_string(), "SUB".to_string()];
            DropDownList::new(&mut modes, &mut channel.mode_selected_idx)
                .w_h(40.0, 30.0)
                .down_from(CHANNEL_FADER + i, 5.0)
                .max_visible_items(5)
                .color(color::BLUE)
                .frame(app.frame_width)
                .frame_color(app.bg_color.plain_contrast())
                .label(&label)
                .label_color(app.bg_color.plain_contrast())
                .react(|selected_idx: &mut Option<usize>, new_idx, string: &str| {
                    *selected_idx = Some(new_idx);
                    fader_enabled = true;
                    dmode = match string {
                        "HTP" => Mode::HTP,
                        "ADD" => Mode::ADD,
                        "SUB" => Mode::SUB,
                        "LTP" => Mode::LTP,
                        _     => {
                            fader_enabled = false;
                            Mode::LTP //TODO: setting disabled = true doesn't work for some reason...
                        }
                    };
                })
                .set(MODE_SELECT + i, ui);
            if fader_enabled { channel.default_mode = dmode; }
            channel.fader_enabled = fader_enabled;


            // let mut disabled = false; //TODO: If it's enabled then display a second bar representing the true output value and one for the output value
            // let label = if disabled { "".to_string() } else { channel.default_mode.string() };
            // let mut dmode = channel.default_mode.clone();
            // let mut modes = vec!["".to_string(), "LTP".to_string(), "HTP".to_string(), "ADD".to_string(), "SUB".to_string()];
            // DropDownList::new(&mut modes, &mut channel.mode_selected_idx)
            //     .w_h(40.0, 30.0)
            //     .down_from(CHANNEL_FADER + i, 5.0)
            //     .max_visible_items(4)
            //     .color(color::BLUE)
            //     .frame(app.frame_width)
            //     .frame_color(app.bg_color.plain_contrast())
            //     .label(&label)
            //     .label_color(app.bg_color.plain_contrast())
            //     .enabled(!disabled)
            //     .react(|selected_idx: &mut Option<usize>, new_idx, string: &str| {
            //         *selected_idx = Some(new_idx);
            //         dmode = match string {
            //             "HTP" => Mode::HTP,
            //             "ADD" => Mode::ADD,
            //             "SUB" => Mode::SUB,
            //             "LTP" => Mode::LTP,
            //             _     => {disabled = true; Mode::LTP} //TODO: setting disabled = true doesn't work for some reason...
            //         };
            //     })
            //     .set(MODE_SELECT + i, ui);
            // //println!("Disabled: {}", disabled);
            // if !disabled { channel.default_mode = dmode; }


            i+=1;
            j = false;
        }
        j = true;
    }
}

fn create_output_window(app_lock: Arc<Mutex<DMXApp>>) {
    let window: PistonWindow = WindowSettings::new("Sushi Reloaded!", [1100, 560])
                                    .exit_on_esc(true).vsync(true).build().unwrap();

    let mut ui = {
        let assets = find_folder::Search::KidsThenParents(3, 5)
            .for_folder("assets").unwrap();
        let font_path = assets.join("fonts/NotoSans/NotoSans-Regular.ttf");
        let theme = Theme::default();
        let glyph_cache = Glyphs::new(&font_path, window.factory.borrow().clone());
        Ui::new(glyph_cache.unwrap(), theme)
    };
    {
        let mut app = app_lock.lock().unwrap();
        let mut fix0 = Fixture::new(1, 10, "RandomThingy".to_string(), app.interface.clone());
        fix0.channels[0].set(Value::new(Mode::LTP, 0));
        app.fixtures.push(fix0);
        let mut fix1 = Fixture::new_with_type(Type::RGB, "LED".to_string(), 11, app.interface.clone());
        fix1.channels[0].set(Value::new(Mode::LTP, 92));
        app.fixtures.push(fix1);
        let mut fix2 = Fixture::new_with_type(Type::Single, "PAR".to_string(), 14, app.interface.clone());
        fix2.channels[0].set(Value::new(Mode::LTP, 120));
        app.fixtures.push(fix2);
    }

    for event in window.ups(60) {
        ui.handle_event(&event);
        {
            let mut app = app_lock.lock().unwrap();
            event.update(|_| ui.set_widgets(|mut ui| set_widgets(&mut ui, &mut app)));
        }
        // event.draw_2d(|c, g| ui.draw_if_changed(c, g));
        event.draw_2d(|c, g| ui.draw(c, g));
    }
}

fn send_to_interface(sock: &UdpSocket, channel: usize, value: u8) {
    // println!("sending {}c{}w", channel, value);
    let buf = format!("{}c{}w", channel, value).into_bytes();
    sock.send_to(&buf, SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), 7777)).unwrap();
}

fn main() {
    println!("Hello world!");

    let (tx, rx) = mpsc::channel::<(usize, u8)>();
    thread::spawn(move || {
        let sock_addr = SocketAddrV4::new(Ipv4Addr::new(0, 0, 0, 0), 9999);
        let sock = UdpSocket::bind(sock_addr).unwrap();
        for m in rx.iter() {
            send_to_interface(&sock, m.0, m.1)
        }
    });

    let app = Arc::new(Mutex::new(DMXApp::new(tx.clone())));

    let app_lock = app.clone();
    let output_thread = thread::spawn(move || {
        println!("Hello world from output window thread!");
        create_output_window(app_lock);
    });

    let app_lock = app.clone();
    let fade_thread = thread::spawn(move || {
        sleep(Duration::new(1, 0));
        println!("Hello world from fade thread!");
        for i in 0..255 {
            {
                let mut app = app_lock.lock().unwrap();
                app.fixtures[0].channels[0].set(Value::new(Mode::LTP, i));
            }
            sleep(Duration::new(0, 50000000));
            // sleep(Duration::new(0, 1960784));
        }
    });

    fade_thread.join().unwrap();
    println!("Fade thread exited!");
    output_thread.join().unwrap();
}
