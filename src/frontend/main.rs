#[macro_use] extern crate conrod;
extern crate piston_window;
extern crate find_folder;
extern crate rand;
extern crate structures;
use structures::*;

use std::net::SocketAddr;
use std::thread;
use std::sync::mpsc;
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
use piston_window::{ EventLoop, Glyphs, PistonWindow, UpdateEvent, WindowSettings, PressEvent };

type Backend = (<piston_window::G2d<'static> as conrod::Graphics>::Texture, Glyphs);
type Ui = conrod::Ui<Backend>;
type UiCell<'a> = conrod::UiCell<'a, Backend>;


widget_ids! {
    CANVAS,
    TITLE,
    CONNECTED_BUTTON
}

struct UI {
    pub watchdog: WatchDogClient,
    tx: mpsc::Sender<Vec<u8>>
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
            tx: tx
        }
    }
}

fn create_output_window() {
    let mut ui = UI::new();

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

    window.set_ups(60);

    // Poll events from the window.
    while let Some(event) = window.next() {
        if let Some(button) = event.press_args() {
            println!("button {:?} pressed", button);
            if button == piston_window::Button::Mouse(piston_window::MouseButton::Left) {
                println!("HI");
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

    let mut i = 0;
    let mut id = TITLE;
    for button in buttons.iter_mut() {
        Button::new()
            .w_h(200.0, 50.0)
            .and(|b| {
                if i > 0 {
                    b.right_from(id, 5.0)
                } else { b.down(25.0) }
            })
            .and(|b| {
                if button.1 {
                    b.rgb(0.1, 0.9, 0.1)
                } else {
                    b.rgb(0.9, 0.1, 0.1)
                }
            })
            .frame(1.0)
            .label(&(button.3.clone()))
            .react(|| {
                button.1 = !button.1;
                let address_type = if button.2 {129} else {1};

                if button.1 {
                    tx.send(vec![address_type, 0, button.0 as u8, 255]).unwrap()
                    // client.send(&[1, 0, button.0 as u8, 255], server);
                } else {
                    tx.send(vec![address_type, 0, button.0 as u8, 0]).unwrap()
                    // client.send(&[1, 0, button.0 as u8, 0], server);
                }
            })
            .set(BUTTON + i, ui);
        id = BUTTON + i;
        i += 1;
    }
}

fn main() {
    // let socket = UDPSocket::new();
    // let watchdog = socket.start_watchdog_client();
    // let client = socket.start_client();
    // std::thread::sleep(std::time::Duration::from_secs(6));
    // println!("{}", watchdog.is_alive());
    // println!("{:?}", watchdog.get_server_addr());

    // if watchdog.is_alive() {
        // let shift: u8 = 128;
        // client.send(&[0,0,11,255], SocketAddr::new(watchdog.get_server_addr().unwrap(), 8001));
        // client.send(&[0,0,15,255], SocketAddr::new(watchdog.get_server_addr().unwrap(), 8001));
        // client.send(&[shift + 1,0,4,13], SocketAddr::new(watchdog.get_server_addr().unwrap(), 8001));
        // println!("{:?}", client.receive());
        create_output_window();
    // }
}
