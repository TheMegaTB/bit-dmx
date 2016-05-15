#[macro_use] extern crate log;
extern crate env_logger;
#[macro_use] extern crate conrod;
extern crate piston_window;
extern crate find_folder;
extern crate rand;
extern crate structures;
extern crate rustc_serialize;
use structures::*;
//use std::io::Read;
use std::time::Duration;
use std::thread;

use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;

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
    BUTTON with 4000,
    CHASER_TITLE with 4000
}

struct UI {
    pub watchdog: WatchDogClient,
    tx: mpsc::Sender<Vec<u8>>,
    frontend_data: FrontendData,
    shift_state: bool
}

impl UI {
    fn new() -> Arc<Mutex<UI>> {
        let socket = UDPSocket::new();
        let watchdog = socket.create_watchdog_client();
        let frontend_client = socket.start_frontend_client();
        let frontend_data = FrontendData::new();

        let (tx, rx) = mpsc::channel::<Vec<u8>>();
        {
            let watchdog = watchdog.clone();
            thread::spawn(move || {
                let mut data;
                loop {
                    data = rx.recv().unwrap();
                    if watchdog.is_alive() {
                        frontend_client.send(data.as_slice(), SocketAddr::new(watchdog.get_server_addr().unwrap(), 8001));
                    } else {
                        println!("Could not send data. No server available");
                    }
                }
            });
        }

        {
            thread::spawn(move || {

            });
        }

        let ui = UI {
            watchdog: watchdog,
            tx: tx,
            frontend_data: frontend_data,
            shift_state: false
        };

        let ui = Arc::new(Mutex::new(ui));

        UI::start_udp_server(ui.clone(), UDPSocket::new());
        UI::start_watchdog_client(ui.clone(), UDPSocket::new());
        ui
    }

    fn start_udp_server(ui: Arc<Mutex<UI>>, socket: UDPSocket) {
        thread::spawn(move || {
            let socket = socket.start_frontend_server();
            loop {
                let buf = socket.receive().0;
                let mut ui_locked = ui.lock().unwrap();

                if buf == [255, 255, 255, 255] {
                    ui_locked.fetch_data();
                }
                else {
                    let address_type:u8 = buf[0] & 127;
                    let address: u16 = ((buf[1] as u16) << 8) + (buf[2] as u16);
                    let value: u8 = buf[3];



                    if address_type == 0 {
                        //TODO Channel view
                    }
                    else if address_type == 1 {
                        // Switch
                        ui_locked.frontend_data.switches[address as usize].dimmer_value = value as f64;
                    }
                    else if address_type == 2 {
                        // chaser
                        let chaser_id = ui_locked.frontend_data.switches[address as usize].clone().chaser_id;
                        let mut chaser = ui_locked.frontend_data.chasers.get_mut(&chaser_id).unwrap();

                        chaser.current_thread = value != 0;
                    }
                }
                println!("{:?}", buf);
            }
        });
    }

    fn start_watchdog_client(ui: Arc<Mutex<UI>>, socket: UDPSocket) {
        let sock = socket.assemble_socket(socket.port + 2, true);
        {
            let (s, s_addr) = {
                let ui_locked = ui.lock().unwrap();
                (ui_locked.watchdog.state.clone(), ui_locked.watchdog.server_addr.clone())
            };
            thread::Builder::new().name("WatchDog-Client".to_string()).spawn(move || {
                sock.set_read_timeout(Some(Duration::from_secs(WATCHDOG_TTL + 1))).unwrap();
                let payload = VERSION.to_string() + &GIT_HASH.to_string();
                let mut buf = (0..(payload.as_bytes().len())).map(|_| 0).collect::<Vec<_>>();
                loop {
                    match sock.recv_from(&mut buf) {
                        Ok((_, addr)) => {
                            if buf == payload.as_bytes() {
                                trace!("received valid watchdog data");
                                s.lock().unwrap()[0] = true;
                                let ip_changed = {
                                    let mut s_addr_locked = s_addr.lock().unwrap();
                                    if s_addr_locked[0] != Some(addr.ip()) {
                                        s_addr_locked[0] = Some(addr.ip());
                                        true
                                    } else {false}
                                };
                                if ip_changed && !ui.lock().unwrap().fetch_data() {
                                    s_addr.lock().unwrap()[0] = None;
                                }
                            } else {
                                println!("RECEIVED INVALID WATCHDOG DATA");
                                trace!("received invalid watchdog data");
                                s.lock().unwrap()[0] = false;
                                s_addr.lock().unwrap()[0] = None;
                            }
                        },
                        Err(_) => {
                            trace!("watchdog timeout");
                            s.lock().unwrap()[0] = false;
                            s_addr.lock().unwrap()[0] = None;
                        }
                    }
                }
            }).unwrap();
        }
    }

    fn fetch_data(&mut self) -> bool {
        if self.watchdog.get_server_addr().is_some() {
            match TcpStream::connect((&*self.watchdog.get_server_addr().unwrap().to_string(), 8000)) {
                Ok(mut stream) => {
                    let mut buffer = String::new();
                    let _ = stream.read_to_string(&mut buffer);
                    self.frontend_data = json::decode(&buffer).unwrap();
                    println!("TCP update");
                    true
                }
                Err(_) => {
                    println!("Error while connecting");
                    false
                }
            }
        }
        else {
            println!("No server ip");
            false
        }
    }
}

fn create_output_window(ui: Arc<Mutex<UI>>, chasers: Vec<String>) {
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
        let mut ui_locked = ui.lock().unwrap();
        if let Some(button) = event.press_args() {
            println!("button {:?} pressed", button);
            if button == piston_window::Button::Keyboard(piston_window::Key::LShift){    //Button::Mouse(piston_window::MouseButton::Left) {
                ui_locked.shift_state = true;
            }
        };
        if let Some(button) = event.release_args() {
            println!("button {:?} pressed", button);
            if button == piston_window::Button::Keyboard(piston_window::Key::LShift){    //Button::Mouse(piston_window::MouseButton::Left) {
                ui_locked.shift_state = false;
            }
        };
        conrod_ui.handle_event(&event);
        event.update(|_| conrod_ui.set_widgets(|mut conrod_ui| set_widgets(&mut conrod_ui, &mut ui_locked, chasers.clone())));
        window.draw_2d(&event, |c, g| conrod_ui.draw_if_changed(c, g));
    }
}

fn set_widgets(mut conrod_ui: &mut UiCell, ui: &mut UI, chasers: Vec<String>) {
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

    // let mut id = None;
    let tx = ui.tx.clone();

    let button_width = 200.0;
    let button_height = 50.0;
    let mut current_button_id = BUTTON;

    //let chasers: Vec<String> = ui.frontend_data.chasers.keys().map(|x| x.clone()).collect(); //TODO edit by user,  save & load

    for (id, (name, chaser)) in chasers.iter().map(|x| (x, ui.frontend_data.chasers.get(x).unwrap())).enumerate() {
        let x_pos = (id as f64 - 0.5) * button_width;
        let y_offset = -50.0;
        let mut last_active_switch_id = None;
        Text::new(name)
            .xy_relative_to(TITLE, [x_pos, y_offset])
            .font_size(15)
            .color(bg_color.plain_contrast())
            .set(CHASER_TITLE + id, conrod_ui);

        for (switch_id_in_chaser, (switch_id, switch)) in chaser.switches.iter().map(|&switch_id| (switch_id, &ui.frontend_data.switches[switch_id])).enumerate() {
            let y_pos = y_offset - 50.0 - switch_id_in_chaser as f64*button_height;
            Button::new()
                .w_h(button_width, button_height)
                .xy_relative_to(TITLE, [x_pos, y_pos])
                .and(|b| {
                    if switch.dimmer_value != 0.0 {
                        last_active_switch_id = Some(switch_id_in_chaser);
                        b.rgb(0.1, 0.9, 0.1)
                    } else {
                        b.rgb(0.9, 0.1, 0.1)
                    }
                })
                .frame(1.0)
                .label(&switch.name)
                .react(|| {
                    let new_value = if switch.dimmer_value == 0.0 {255} else {0};
                    tx.send(get_switch_update(ui.shift_state, switch_id as u16, new_value)).unwrap();
                })
                .set(current_button_id, conrod_ui);
                current_button_id = current_button_id + 1;
        }
        let y_pos = y_offset - 50.0 - (chaser.switches.len() as f64 - 0.25)*button_height;
        {
            let tx = tx.clone();
            let x_pos = (id as f64 - 5f64/6f64) * button_width;
            Button::new()
                .w_h(button_width/3.0, button_height/2.0)
                .xy_relative_to(TITLE, [x_pos, y_pos])
                .rgb(0.9, 0.9, 0.1)
                .frame(1.0)
                .label(&"<<".to_string())
                .react(|| {
                    println!("<<");
                    let next_switch_id = {
                        match last_active_switch_id {
                            Some(last_active_switch_id) => {
                                if last_active_switch_id == 0 {chaser.switches.len() - 1} else {last_active_switch_id - 1}
                            },
                            None => 0
                        }
                    };
                    tx.send(get_switch_update(!ui.shift_state, chaser.switches[next_switch_id] as u16, 255)).unwrap();
                })
                .set(current_button_id, conrod_ui);
                current_button_id = current_button_id + 1;
        }
        {
            let tx = tx.clone();
            let x_pos = (id as f64 - 0.5) * button_width;
            let (label, r) = {
                if chaser.current_thread {
                    ("||".to_string(), 0.1)
                }
                else {
                    (">".to_string(), 0.9)
                }
            };
            Button::new()
                .w_h(button_width/3.0, button_height/2.0)
                .xy_relative_to(TITLE, [x_pos, y_pos])
                .rgb(r, 0.9, 0.1)
                .frame(1.0)
                .label(&label)
                .react(|| {
                    println!(">");
                    let next_switch_id = {
                        match last_active_switch_id {
                            Some(last_active_switch_id) => {
                                if last_active_switch_id == 0 {chaser.switches.len() - 1} else {last_active_switch_id - 1}
                            },
                            None => 0
                        }
                    };
                    if chaser.current_thread {
                        tx.send(get_start_chaser(!ui.shift_state, chaser.switches[next_switch_id] as u16, 0)).unwrap();
                    }
                    else {
                        tx.send(get_start_chaser(!ui.shift_state, chaser.switches[next_switch_id] as u16, 255)).unwrap();
                    }
                })
                .set(current_button_id, conrod_ui);
                current_button_id = current_button_id + 1;
        }
        {
            let x_pos = (id as f64 - 1f64/6f64) * button_width;
            Button::new()
                .w_h(button_width/3.0, button_height/2.0)
                .xy_relative_to(TITLE, [x_pos, y_pos])
                .rgb(0.9, 0.9, 0.1)
                .frame(1.0)
                .label(&">>".to_string())
                .react(|| {
                    println!(">>");
                    let next_switch_id = {
                        match last_active_switch_id {
                            Some(last_active_switch_id) => {
                                if last_active_switch_id + 1 == chaser.switches.len() {0} else {last_active_switch_id + 1}
                            },
                            None => 0
                        }
                    };
                    tx.send(get_switch_update(!ui.shift_state, chaser.switches[next_switch_id] as u16, 255)).unwrap();
                })
                .set(current_button_id, conrod_ui);
                current_button_id = current_button_id + 1;
        }
    }
}

fn get_switch_update(shift_state: bool, addr: u16, value: u8) -> Vec<u8> {
    let addr_high = (addr >> 8) as u8;
    let addr_low = addr as u8;
    vec![if shift_state {129} else {1}, addr_high, addr_low, value]
}

fn get_start_chaser(shift_state: bool, addr: u16, value: u8) -> Vec<u8> {
    let addr_high = (addr >> 8) as u8;
    let addr_low = addr as u8;
    vec![if shift_state {130} else {2}, addr_high, addr_low, value]
}

fn create_splash_window(ui: Arc<Mutex<UI>>) {
    let mut window: PistonWindow = WindowSettings::new("BitDMX Splashscreen", [500, 300])
                                    .exit_on_esc(true).vsync(true).build().unwrap();

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
        conrod_ui.handle_event(&event);
        window.draw_2d(&event, |c, g| conrod_ui.draw_if_changed(c, g));

        event.update(|_| conrod_ui.set_widgets(|mut conrod_ui| {
            Canvas::new()
                .frame(1.0)
                .pad(30.0)
                .color(color::rgb(0.236, 0.239, 0.900))
                .set(CANVAS, &mut conrod_ui);
        }));

        if ui.lock().unwrap().watchdog.is_alive() { break }
    }
}

fn lines_from_file() -> Vec<String>
{
    let assets = find_folder::Search::KidsThenParents(3, 5)
        .for_folder("assets").unwrap();
    let path = assets.join("chasers.dmx");
    let file = File::open(path).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines().map(|l| l.expect("Could not parse line")).collect()
}


fn main() {
    println!("BitDMX frontend v{}-{}", VERSION, GIT_HASH);
    let ui = UI::new();
    create_splash_window(ui.clone());
    let chasers = lines_from_file();
    create_output_window(ui.clone(), chasers);
}
