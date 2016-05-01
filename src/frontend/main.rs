#[macro_use] extern crate conrod;
extern crate piston_window;
extern crate find_folder;
extern crate rand;
extern crate structures;
extern crate rustc_serialize;
use structures::*;
use std::io::Read;
use std::time::Duration;
use std::thread::{self, sleep};

use std::net::{TcpStream, SocketAddr};
use std::sync::{Arc, Mutex, mpsc};
use conrod::{
    color,
    Canvas,
    Colorable,
    Frameable,
    Positionable,
    Text,
    Theme,
    Widget,
    Button,
    // Circle,
    // Color,
    // DropDownList,
    // EnvelopeEditor,
    Labelable,
    // NumberDialer,
    // Point,
    Sizeable,
    // Slider,
    // TextBox,
    // Toggle,
    // WidgetMatrix,
    // XYPad,
};
use piston_window::{ EventLoop, Glyphs, PistonWindow, UpdateEvent, WindowSettings, PressEvent, ReleaseEvent };

use rustc_serialize::json;

type Backend = (<piston_window::G2d<'static> as conrod::Graphics>::Texture, Glyphs);
type Ui = conrod::Ui<Backend>;
type UiCell<'a> = conrod::UiCell<'a, Backend>;


widget_ids! {
    CANVAS,
    TITLE,
    CONNECTED_BUTTON,
    BUTTON with 4000
}

struct UI {
    pub watchdog: WatchDogClient,
    tx: mpsc::Sender<Vec<u8>>,
    frontend_data: FrontendData,
    shift_state: bool
}

impl UI {
    fn new() -> UI {
        let socket = UDPSocket::new();
        let watchdog = socket.start_watchdog_client();
        let client = socket.start_client();
        let (tx, rx) = mpsc::channel::<Vec<u8>>();
        {
            let watchdog = watchdog.clone();
            thread::spawn(move || {
                let mut data;
                loop {
                    data = rx.recv().unwrap();
                    if watchdog.is_alive() {
                        client.send(data.as_slice(), SocketAddr::new(watchdog.get_server_addr().unwrap(), 8001));
                    } else {
                        println!("Couldn't send data. No server available");
                    }
                }
            });
        }

        UI {
            watchdog: watchdog,
            tx: tx,
            frontend_data: FrontendData::new(),
            shift_state: false
        }
    }

    fn fetch_data(&mut self) {
        if self.watchdog.get_server_addr().is_some() {
            match TcpStream::connect((&*self.watchdog.get_server_addr().unwrap().to_string(), 8000)) {
                Ok(mut stream) => {
                    let mut buffer = String::new();
                    let _ = stream.read_to_string(&mut buffer);

                    self.frontend_data = json::decode(&buffer).unwrap();
                    //println!("{:?}", self.frontend_data);
                }
                Err(_) => {
                    println!("Error while connecting");
                }
            }
        }
        else {
            println!("No server ip");
        }
    }
}

fn create_output_window() {
    let mut ui = UI::new();
    sleep(Duration::from_millis(6000));
    ui.fetch_data();

    let mut window: PistonWindow = WindowSettings::new("Sushi Reloaded!", [1100, 560])
                                    .exit_on_esc(false).vsync(true).build().unwrap();

    let mut conrod_ui = {
        let assets = find_folder::Search::KidsThenParents(3, 5)
            .for_folder("assets").unwrap();
        let font_path = assets.join("fonts/NotoSans/NotoSans-Regular.ttf");
        let theme = Theme::default();
        let glyph_cache = Glyphs::new(&font_path, window.factory.clone());
        Ui::new(glyph_cache.unwrap(), theme)
    };

    window.set_ups(30);

    // Poll events from the window.
    while let Some(event) = window.next() {
        if let Some(button) = event.press_args() {
            println!("button {:?} pressed", button);
            if button == piston_window::Button::Keyboard(piston_window::Key::LShift){    //Button::Mouse(piston_window::MouseButton::Left) {
                ui.shift_state = true;
            }
        };
        if let Some(button) = event.release_args() {
            println!("button {:?} pressed", button);
            if button == piston_window::Button::Keyboard(piston_window::Key::LShift){    //Button::Mouse(piston_window::MouseButton::Left) {
                ui.shift_state = false;
            }
        };
        conrod_ui.handle_event(&event);
        event.update(|_| conrod_ui.set_widgets(|mut conrod_ui| set_widgets(&mut conrod_ui, &mut ui)));
        window.draw_2d(&event, |c, g| conrod_ui.draw_if_changed(c, g));
    }
}

fn set_widgets(mut conrod_ui: &mut UiCell, ui: &mut UI) {
    let bg_color = color::rgb(0.236, 0.239, 0.241);

    Canvas::new()
        .frame(1.0)
        .pad(30.0)
        .color(bg_color)
        .scroll_kids()
        .set(CANVAS, &mut conrod_ui);

    Text::new("Moonshadow 2016!")
        .top_left_with_margin_on(CANVAS, 0.0)
        .font_size(32)
        .color(bg_color.plain_contrast())
        .set(TITLE, conrod_ui);

    let connected = ui.watchdog.is_alive();
    let label = if connected { "Connected".to_string() } else { "Disconnected".to_string() };
    Button::new()
        .w_h(75.0, 25.0)
        .right_from(TITLE, 5.0)
        .frame(1.0)
        .label(&label)
        .label_font_size(11)
        .and(|b| {
            if connected {
                b.rgb(0.1, 0.9, 0.1)
            } else {
                b.rgb(0.9, 0.1, 0.1)
            }
        })
        .react(|| {})
        .set(CONNECTED_BUTTON, conrod_ui);

    let mut id = TITLE;
    let tx = ui.tx.clone();
    for (i, button) in ui.frontend_data.switches.iter().enumerate() {
        let label = button.name.clone();// i.to_string();
        Button::new()
            .w_h(200.0, 50.0)
            .and(|b| {
                if i > 0 {
                    b.right_from(id, 5.0)
                } else { b.down(25.0) }
            })
            .and(|b| {
                if button.dimmer_value != 0.0 {
                    b.rgb(0.1, 0.9, 0.1)
                } else {
                    b.rgb(0.9, 0.1, 0.1)
                }
            })
            .frame(1.0)
            .label(&label)
            .react(|| {
                let new_value = if button.dimmer_value == 0.0 {255} else {0};
                tx.send(vec![if ui.shift_state {129} else {1}, 0, i as u8, new_value]).unwrap();
                // button.1 = !button.1;
                // if button.1 {
                //     tx.send(vec![1, 0, button.0 as u8, 255]).unwrap()
                //     // client.send(&[1, 0, button.0 as u8, 255], server);
                // } else {
                //     tx.send(vec![1, 0, button.0 as u8, 0]).unwrap()
                //     // client.send(&[1, 0, button.0 as u8, 0], server);
                // }
            })
            .set(BUTTON + i, conrod_ui);
        id = BUTTON + i;
    }
    ui.fetch_data(); //TODO replace with udp
}

fn main() {
    println!("BitDMX frontend v{}-{}", VERSION, GIT_HASH);
    create_output_window();
}
