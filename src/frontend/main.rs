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

type Backend = (<piston_window::G2d<'static> as conrod::Graphics>::Texture, Glyphs);
type Ui = conrod::Ui<Backend>;
type UiCell<'a> = conrod::UiCell<'a, Backend>;


widget_ids! {
    CANVAS,
    TITLE,
    BUTTON with 2000
}

fn create_output_window(tx: mpsc::Sender<Vec<u8>>) {
    let mut window: PistonWindow = WindowSettings::new("Sushi Reloaded!", [1100, 560])
                                    .exit_on_esc(true).vsync(true).build().unwrap();

    let mut ui = {
        let assets = find_folder::Search::KidsThenParents(3, 5)
            .for_folder("assets").unwrap();
        let font_path = assets.join("fonts/NotoSans/NotoSans-Regular.ttf");
        let theme = Theme::default();
        let glyph_cache = Glyphs::new(&font_path, window.factory.clone());
        Ui::new(glyph_cache.unwrap(), theme)
    };

    let mut buttons: Vec<(usize, bool, bool, String)> = Vec::new();

    buttons.push((0, false, false, "Switch 0".to_string()));
    buttons.push((1, false, false, "Switch 1".to_string()));
    buttons.push((2, false, true, "Switch 2".to_string()));

    window.set_ups(60);

    // Poll events from the window.
    while let Some(event) = window.next() {
        ui.handle_event(&event);
        event.update(|_| ui.set_widgets(|mut ui| set_widgets(&mut ui, &mut buttons, tx.clone())));
        window.draw_2d(&event, |c, g| ui.draw_if_changed(c, g));
    }
}

fn set_widgets(mut ui: &mut UiCell, buttons: &mut Vec<(usize, bool, bool, String)>, tx: mpsc::Sender<Vec<u8>>) {
    let bg_color = color::rgb(0.236, 0.239, 0.241);

    Canvas::new()
        .frame(1.0)
        .pad(30.0)
        .color(bg_color)
        .scroll_kids()
        .set(CANVAS, &mut ui);

    Text::new("Moonshadow 2016!")
        .top_left_with_margin_on(CANVAS, 0.0)
        .font_size(32)
        .color(bg_color.plain_contrast())
        .set(TITLE, ui);

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
    let socket = UDPSocket::new();
    let watchdog = socket.start_watchdog_client();
    let client = socket.start_client();
    std::thread::sleep(std::time::Duration::from_secs(6));
    println!("{}", watchdog.is_alive());
    println!("{:?}", watchdog.get_server_addr());

    if watchdog.is_alive() {
        let shift: u8 = 128;
        // client.send(&[1,0,0,255], SocketAddr::new(watchdog.get_server_addr().unwrap(), 8001));
        // client.send(&[1,0,1,255], SocketAddr::new(watchdog.get_server_addr().unwrap(), 8001));
        // std::thread::sleep(std::time::Duration::from_millis(2500));
        // client.send(&[shift + 1,0,2,255], SocketAddr::new(watchdog.get_server_addr().unwrap(), 8001));
        // client.send(&[shift + 1,0,4,13], SocketAddr::new(watchdog.get_server_addr().unwrap(), 8001));
        // println!("{:?}", client.receive());

        let (tx, rx) = mpsc::channel::<Vec<u8>>();

        thread::spawn(move || {
            loop {
                client.send(rx.recv().unwrap().as_slice(), SocketAddr::new(watchdog.get_server_addr().unwrap(), 8001));
            }
        });

        create_output_window(tx);
    }
}
