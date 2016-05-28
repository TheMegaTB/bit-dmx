#[macro_use] extern crate log;
extern crate env_logger;
#[macro_use] extern crate conrod;
extern crate piston_window;
extern crate find_folder;
extern crate structures;
extern crate rustc_serialize;
use structures::*;
use std::time::Duration;
use std::thread;

use std::io::prelude::*;
use std::io::{BufReader, BufWriter};
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
    DropDownList,
    // EnvelopeEditor,
    Labelable,
    // NumberDialer,
    // Point,
    Sizeable,
    Slider,
    TextBox,
    // Toggle,
    // WidgetMatrix,
    // XYPad,
};
use piston_window::{ EventLoop, OpenGL, Glyphs, PistonWindow, UpdateEvent, WindowSettings, PressEvent, ReleaseEvent, Window };
use rustc_serialize::json;


type Backend = (<piston_window::G2d<'static> as conrod::Graphics>::Texture, Glyphs);
type Ui = conrod::Ui<Backend>;
type UiCell<'a> = conrod::UiCell<'a, Backend>;

const OPEN_GL: OpenGL = OpenGL::V3_2;


widget_ids! {
    CANVAS,
    TITLE,
    CONNECTED_BUTTON,
    EDITOR_BUTTON,
    ADD_CHASER_BUTTON,
    EDITOR_TITLE,
    EDITOR_INFO,
    EDITOR_TIME_SLIDER,
    EDITOR_CHASER_TITLE with 4000,
    EDITOR_CONTENT with 4000,
    BUTTON with 4000,
    CHASER_TITLE with 4000,
    EDITOR_SWITCH_SLIDER with 4000,
    EDITOR_SWITCH_BUTTON with 4000,
    EDITOR_SWITCH_TEXT with 4000,
    EDITOR_SWITCH_DROP_DOWNS with 4000,
    EDITOR_CURVE_STRING1,
    EDITOR_CURVE_STRING2
}

#[derive(Debug, Clone)]
struct UI {
    pub watchdog: WatchDogClient,
    tx: mpsc::Sender<Vec<u8>>,
    frontend_data: FrontendData,
    shift_state: bool,
    edit_state: bool,
    chasers: Vec<String>,
    waiting_for_keybinding: bool,
    current_edited_switch_id: Arc<Mutex<[Option<usize>; 1]>>,
    current_edited_channel_group_id: i64,
    current_edited_switch_name: Arc<Mutex<[String; 1]>>,
    current_edited_curve_strings: Arc<Mutex<[String; 2]>>,
    current_edited_chaser_names: Arc<Mutex<Vec<String>>>
}

impl UI {
    fn new() -> Arc<Mutex<UI>> {
        let socket = UDPSocket::new();
        let watchdog = socket.create_watchdog_client();
        let frontend_client = socket.start_frontend_client();
        let frontend_data = FrontendData::new("Default".to_string());

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
                        warn!("Could not send data. No server available");
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
            shift_state: false,
            edit_state: false,
            chasers: Vec::new(),
            waiting_for_keybinding: false,
            current_edited_switch_id: Arc::new(Mutex::new([None])),
            current_edited_channel_group_id: -1,
            current_edited_switch_name: Arc::new(Mutex::new(["".to_string()])),
            current_edited_curve_strings: Arc::new(Mutex::new(["".to_string(), "".to_string()])),
            current_edited_chaser_names: Arc::new(Mutex::new(Vec::new()))
        };

        let ui = Arc::new(Mutex::new(ui));

        UI::start_udp_server(ui.clone(), UDPSocket::new());
        UI::start_watchdog_client(ui.clone(), UDPSocket::new());
        ui
    }

    fn get_chaser_config_path(&self) -> std::path::PathBuf {
        let assets = find_folder::Search::KidsThenParents(3, 5)
            .for_folder("assets").unwrap();
        assets.join(self.frontend_data.name.clone()  + ".local_dmx")
    }

    fn load_chaser_config(&mut self) {
        let path = self.get_chaser_config_path();
        match File::open(path) {
            Ok(file) => {
                let buf = BufReader::new(file);
                self.chasers = buf.lines().map(|l| l.expect("Could not parse line")).collect();
            },
            _ => {
                self.chasers = self.frontend_data.chasers.keys().map(|x| x.clone()).collect();
                self.save_chaser_config();
            }
        }
    }

    fn save_chaser_config(&self) {
        let path = self.get_chaser_config_path();
        let file = File::create(path).expect("no such file");
        let mut buf = BufWriter::new(file);
        for line in self.chasers.iter() {
            buf.write_all((line.clone() + "\n").as_bytes()).unwrap();
        }
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
                trace!("{:?}", buf);
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
                    info!("Connected to \"{}\"", self.frontend_data.name);
                    self.load_chaser_config();
                    debug!("TCP update");
                    true
                }
                Err(_) => {
                    warn!("Error while connecting");
                    false
                }
            }
        } else {
            warn!("No server ip");
            false
        }
    }

    fn send_data(&mut self) -> bool {
        if self.watchdog.get_server_addr().is_some() {
            match TcpStream::connect((&*self.watchdog.get_server_addr().unwrap().to_string(), 8001)) {
                Ok(mut stream) => {
                    stream.write(self.frontend_data.get_json_string().as_bytes()).unwrap();
                    // stream.write(json::encode(&self.frontend_data).unwrap().as_bytes()).unwrap();
                    true
                }
                Err(_) => {
                    error!("Error while connecting");
                    false
                }
            }
        }
        else {
            warn!("No server ip");
            false
        }
    }
}

fn create_output_window(ui: Arc<Mutex<UI>>) {
    let mut window: PistonWindow = WindowSettings::new("Sushi Reloaded!", [1100, 560])
                                   .opengl(OPEN_GL).exit_on_esc(false).vsync(true).build().unwrap();

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
            if ui_locked.waiting_for_keybinding {
                let switch_id = ui_locked.current_edited_switch_id.lock().unwrap()[0];
                match switch_id {
                    Some(switch_id) => {
                        match button {
                            piston_window::Button::Keyboard(key) =>  {
                                ui_locked.frontend_data.switches[switch_id].keybinding = Some(key);
                                ui_locked.waiting_for_keybinding = false;
                                ui_locked.send_data();
                            },
                            _ => {
                                ui_locked.frontend_data.switches[switch_id].keybinding = None;
                                ui_locked.waiting_for_keybinding = false;
                                ui_locked.send_data();
                            }
                        }
                    },
                    None => {}
                }
            }
            else if !ui_locked.edit_state {
                match button {
                    piston_window::Button::Keyboard(key) =>  {
                        for (switch_id, switch) in ui_locked.frontend_data.switches.iter().enumerate() {
                            if switch.keybinding == Some(key) {
                                let new_value = if switch.dimmer_value == 0.0 {255} else {0};
                                ui_locked.tx.send(get_switch_update(ui_locked.shift_state, switch_id as u16, new_value)).unwrap();
                            }
                        }
                    },
                    _ => {}
                }
            }
        };
        if let Some(button) = event.release_args() {
            println!("button {:?} released", button);
            if button == piston_window::Button::Keyboard(piston_window::Key::LShift){    //Button::Mouse(piston_window::MouseButton::Left) {
                ui_locked.shift_state = false;
            }
        };
        conrod_ui.handle_event(&event);
        event.update(|_| conrod_ui.set_widgets(|mut conrod_ui| set_widgets(&mut conrod_ui, &mut ui_locked, window.size().width)));
        window.draw_2d(&event, |c, g| conrod_ui.draw_if_changed(c, g));
    }
}

fn set_widgets(mut conrod_ui: &mut UiCell, ui: &mut UI, window_width: u32) {
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
        .w_h(105.0, 35.0)
        .down_from(TITLE, 5.0)
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

    Button::new()
        .w_h(105.0, 35.0)
        .right_from(CONNECTED_BUTTON, 5.0)
        .frame(1.0)
        .label(&"Edit Mode".to_string())
        .label_font_size(11)
        .and(|b| {
            if ui.edit_state {
                b.rgb(0.1, 0.9, 0.1)
            } else {
                b.rgb(0.9, 0.1, 0.1)
            }
        })
        .react(|| {
            ui.current_edited_switch_id.lock().unwrap()[0] = None;
            ui.current_edited_switch_name.lock().unwrap()[0] = "".to_string();
            if ui.edit_state {
                ui.send_data();
            }
            else {
                ui.current_edited_chaser_names = Arc::new(Mutex::new(ui.chasers.clone()));
            }
            ui.edit_state = !ui.edit_state;
        })
        .set(EDITOR_BUTTON, conrod_ui);

    if ui.edit_state {
        Button::new()
            .w_h(105.0, 35.0)
            .right_from(EDITOR_BUTTON, 5.0)
            .frame(1.0)
            .label(&"Add Chaser".to_string())
            .label_font_size(11)
            .and(|b| {
                if ui.edit_state {
                    b.rgb(0.1, 0.9, 0.1)
                } else {
                    b.rgb(0.9, 0.1, 0.1)
                }
            })
            .react(|| {
                let name = ui.frontend_data.add_chaser();
                ui.chasers.push(name);
                ui.current_edited_chaser_names = Arc::new(Mutex::new(ui.chasers.clone()));
                ui.save_chaser_config();
                ui.send_data();
            })
            .set(ADD_CHASER_BUTTON, conrod_ui);
    }

    // let mut id = None;
    let tx = ui.tx.clone();

    let button_width = 200.0;
    let button_height = 50.0;
    let mut current_button_id = BUTTON;

    let chasers = {
        let mut tmp_chasers = Vec::new();
        for chaser_name in ui.chasers.iter() {
            if ui.frontend_data.chasers.contains_key(chaser_name) {
                tmp_chasers.push(chaser_name.clone());
            }
        }
        tmp_chasers
    };

    let mut next_y_offset = 0f64;
    let mut x_pos = -button_width;
    let mut y_offset = -90.0;
    let mut rightmost = 0.0;

    let cloned_ui = ui.clone();


    for (id, (name, chaser)) in chasers.iter().map(|x| (x, cloned_ui.frontend_data.chasers.get(x).unwrap())).enumerate() {
        x_pos = x_pos + button_width;
        //let x_pos = (id as f64 - 0.5) * button_width;
        let usable_width = if ui.edit_state {2*window_width/3} else {window_width};
        let tmp_rightmost = x_pos + 0.5*button_width;
        if (x_pos + 1.5*button_width) as u32 >= usable_width {
            rightmost = tmp_rightmost;
            x_pos = 0.0;
            y_offset = next_y_offset;
        }
        else if tmp_rightmost > rightmost {
            rightmost = tmp_rightmost + button_width;
        }
        let mut last_active_switch_id = None;
        let current_edited_chaser_names = ui.current_edited_chaser_names.clone();
        if ui.edit_state {
            // let tmp_name = {current_edited_chaser_names.lock().unwrap()[id].clone()};
            let ref mut current_chaser_name = current_edited_chaser_names.lock().unwrap()[id];

            // let ref mut switch_name = switch_name.lock().unwrap()[0];
            TextBox::new(current_chaser_name)
                .font_size(14)
                .xy_relative_to(TITLE, [x_pos, y_offset])
                .w_h(button_width, button_height)
                .frame(2.0)
                .frame_color(bg_color.invert().plain_contrast())
                .color(bg_color.plain_contrast())
                .react(|new_name: &mut String| {
                    let old_name = ui.chasers[id].clone();
                    ui.frontend_data.rename_chaser(old_name.clone(), new_name.clone());
                    ui.chasers = ui.chasers.iter().map(|x| if *x == old_name {new_name.clone()} else {x.clone()}).collect();
                    ui.save_chaser_config();
                    ui.send_data();
                })
                .enabled(true)
                .set(EDITOR_CHASER_TITLE + id, conrod_ui);
        }
        else {
            Text::new(name)
                .xy_relative_to(TITLE, [x_pos, y_offset])
                .font_size(14)
                .color(bg_color.plain_contrast())
                .set(CHASER_TITLE + id, conrod_ui);
        }

        for (switch_id_in_chaser, (switch_id, switch)) in chaser.switches.iter().map(|&switch_id| (switch_id, &ui.frontend_data.switches[switch_id])).enumerate() {
            let y_pos = y_offset - 50.0 - switch_id_in_chaser as f64*button_height;
            let current_edited_switch = ui.current_edited_switch_id.clone();

            let label = match switch.get_keybinding_as_text() {
                Some(keybinding) => switch.name.clone() + ": " + &keybinding,
                None => switch.name.clone()
            };
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
                .label(&label)
                .react(|| {
                    if ui.edit_state {
                        current_edited_switch.lock().unwrap()[0] = Some(switch_id);
                        ui.current_edited_switch_name.lock().unwrap()[0] = switch.name.clone();
                        println!("set to {:?}", switch.name);
                    }
                    else {
                        let new_value = if switch.dimmer_value == 0.0 {255} else {0};
                        tx.send(get_switch_update(ui.shift_state, switch_id as u16, new_value)).unwrap();
                    }
                })
                .set(current_button_id, conrod_ui);
                current_button_id = current_button_id + 1;
        }
        let mut y_pos = y_offset - 50.0 - (chaser.switches.len() as f64 - 0.25)*button_height;
        if !ui.edit_state {
            {
                let tx = tx.clone();
                //let x_pos = (id as f64 - 5f64/6f64) * button_width;
                Button::new()
                    .w_h(button_width/3.0, button_height/2.0)
                    .xy_relative_to(TITLE, [x_pos - button_width/3.0, y_pos])
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
                //let x_pos = (id as f64 - 0.5) * button_width;
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
                //let x_pos = (id as f64 - 1f64/6f64) * button_width;
                Button::new()
                    .w_h(button_width/3.0, button_height/2.0)
                    .xy_relative_to(TITLE, [x_pos + button_width/3.0, y_pos])
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
        else {
            let mut test = false; //TODO Fix this fake
            Button::new()
                .w_h(button_width, button_height/2.0)
                .xy_relative_to(TITLE, [x_pos, y_pos])
                .rgb(0.9, 0.9, 0.1)
                .frame(1.0)
                .label(&"Add".to_string())
                .react(|| {
                    let switch_id = ui.frontend_data.add_switch(JsonSwitch::new("Untitled".to_string(), name.clone()));
                    ui.current_edited_switch_id.lock().unwrap()[0] = Some(switch_id);
                    ui.current_edited_switch_name.lock().unwrap()[0] = "Untitled".to_string();

                    ui.send_data();
                    test = true;
                })
                .set(current_button_id, conrod_ui);
                current_button_id = current_button_id + 1;

            Button::new()
                    .w_h(button_width, button_height/2.0)
                    .xy_relative_to(TITLE, [x_pos, y_pos - button_height/2.0])
                    .rgb(0.9, 0.1, 0.1)
                    .frame(1.0)
                    .label(&"Delete".to_string())
                    .react(|| {
                        ui.frontend_data.delete_chaser(name.clone());
                        ui.chasers.retain(|x| x != name);
                        ui.save_chaser_config();
                        ui.send_data();
                        test = true;
                    })
                    .set(current_button_id, conrod_ui);
                    current_button_id = current_button_id + 1;

            y_pos = y_pos - button_height;
            if test
            {
                return;
            }
        }
        if y_pos - button_height < next_y_offset {
            next_y_offset = y_pos - button_height;
        }
    }
    if ui.edit_state {
        let x_pos = rightmost;
        let mut y_pos = - 30.0;

        Text::new("Editor")
            .xy_relative_to(TITLE, [x_pos, y_pos])
            .font_size(22)
            .color(bg_color.plain_contrast())
            .set(EDITOR_TITLE, conrod_ui);

        y_pos = y_pos - 40.0;

        let current_edited_switch = {
            ui.current_edited_switch_id.lock().unwrap()[0].clone()
        };

        let switch_name = ui.current_edited_switch_name.clone();

        match current_edited_switch {
            Some(switch_id) => {

                Text::new(&("Switch #".to_string() + &switch_id.to_string() + ": " + &ui.frontend_data.switches[switch_id].name.clone()))
                    .xy_relative_to(TITLE, [x_pos, y_pos])
                    .font_size(14)
                    .color(bg_color.plain_contrast())
                    .set(EDITOR_INFO, conrod_ui);

                y_pos = y_pos - 60.0;

                let time = ui.frontend_data.switches[switch_id].before_chaser;
                let item_width = 320.0;
                let item_height = 40.0;
                let item_x_offset = 20.0;
                let line = "-----------------------------------------";
                let ref mut switch_name = switch_name.lock().unwrap()[0];
                //println!("name: {:?}", switch_name);


                TextBox::new(switch_name)
                    .font_size(14)
                    .xy_relative_to(TITLE, [x_pos, y_pos])
                    .w_h(item_width, item_height)
                    .frame(2.0)
                    .frame_color(bg_color.invert().plain_contrast())
                    .color(bg_color.plain_contrast())
                    .react(|new_name: &mut String| {
                        ui.frontend_data.switches[switch_id].name = new_name.clone();
                        ui.send_data();
                    })
                    .enabled(true)
                    .set(EDITOR_CONTENT, conrod_ui);

                y_pos = y_pos - 60.0;

                let time_string = time.to_string();
                let label = {
                    let mut text = "Chaser time: ".to_string();
                    text.push_str(&time_string);
                    text
                };

                Slider::new(time as f32, 0.0, 10000.0)
                    .w_h(item_width, item_height)
                    .xy_relative_to(TITLE, [x_pos, y_pos])
                    .rgb(0.5, 0.3, 0.6)
                    .frame(2.0)
                    .label(&label)
                    .label_color(color::WHITE)
                    .react(|new_time: f32| {
                        ui.frontend_data.switches[switch_id].before_chaser = new_time as FadeTime;
                        ui.send_data();
                    })
                    .set(EDITOR_TIME_SLIDER, conrod_ui);

                let label = if ui.waiting_for_keybinding {
                    "Keybinding: ?".to_string()
                }
                else {
                    match ui.frontend_data.switches[switch_id].get_keybinding_as_text() {
                        Some(keybinding) => "Keybinding: ".to_string() + &keybinding,
                        None => "No keybinding".to_string()
                    }
                };



                y_pos = y_pos - 60.0;
                let mut editor_switch_slider_count = 0;
                let mut editor_switch_button_count = 0;
                let mut editor_switch_text_count = 0;
                let mut editor_switch_drop_downs_count = 0;


                Button::new()
                .w_h(item_width, item_height)
                .xy_relative_to(TITLE, [x_pos, y_pos])
                .rgb(0.9, 0.9, 0.1)
                .frame(1.0)
                .label(&label)
                .react(|| {
                    ui.waiting_for_keybinding = true;
                })
                .set(EDITOR_SWITCH_BUTTON + editor_switch_button_count, conrod_ui);
                editor_switch_button_count += 1;
                y_pos = y_pos - 60.0;

                Text::new(line)
                    .xy_relative_to(TITLE, [x_pos, y_pos])
                    .font_size(14)
                    .color(bg_color.plain_contrast())
                    .set(EDITOR_SWITCH_TEXT + editor_switch_text_count, conrod_ui);
                editor_switch_text_count += 1;
                y_pos = y_pos - 60.0;

                let cloned_ui = ui.clone();

                let mut data: Vec<String> = cloned_ui.frontend_data.switches[switch_id].channel_groups.keys().map(|x| x.clone()).collect();
                data.sort();

                //println!("{:?}", cloned_ui.frontend_data.switches[switch_id].channel_groups);

                let mut dropdown_list = Vec::new();
                let mut dropdown_background_list_fixture = Vec::new();
                let mut dropdown_background_list_channel_groups = Vec::new();
                for (fixture_index, fixture) in cloned_ui.frontend_data.fixtures.iter().enumerate() {
                    if fixture.channel_groups.len() == 1 {
                        dropdown_list.push(fixture.name.clone());
                        dropdown_background_list_fixture.push(fixture_index);
                        dropdown_background_list_channel_groups.push(0);
                    }
                    else {
                        for (channel_group_index, _) in fixture.channel_groups.iter().enumerate() {
                            dropdown_list.push(fixture.name.clone() + ":" + &channel_group_index.to_string());
                            dropdown_background_list_fixture.push(fixture_index);
                            dropdown_background_list_channel_groups.push(channel_group_index);
                        }
                    }
                }

                for (id_string, data) in data.iter().map(|x| (x, cloned_ui.frontend_data.switches[switch_id].channel_groups.get(x).unwrap())) {
                    let mut id_vector: Vec<String> = id_string.split(",").map(|x| x.to_string()).collect();
                    id_vector[0].remove(0);
                    id_vector[1].pop();
                    let fixture_id = id_vector[0].parse::<usize>().unwrap();
                    let channel_group_id = id_vector[1].parse::<usize>().unwrap();



                    let mut dropdown_index = 0;
                    for (index, (&fixture_index, &channel_group_index)) in dropdown_background_list_fixture.iter().zip(dropdown_background_list_channel_groups.iter()).enumerate() {
                        if fixture_index == fixture_id && channel_group_index == channel_group_id {
                            dropdown_index = index;
                        }
                    }

                    DropDownList::new(&mut dropdown_list, &mut Some(dropdown_index))
                        .w_h(item_width-item_height, item_height)
                        .xy_relative_to(TITLE, [x_pos-item_height/2.0, y_pos])
                        .rgb(0.5, 0.3, 0.6)
                        .frame(2.0)
                        .label(&cloned_ui.frontend_data.fixtures[fixture_id].name.clone())
                        .label_color(color::WHITE)
                        .react(|_: &mut Option<usize>, new_idx, _: &str| {
                            if ui.frontend_data.change_channel_group(switch_id, id_string.clone(), dropdown_background_list_fixture[new_idx], dropdown_background_list_channel_groups[new_idx]) {
                                ui.current_edited_channel_group_id = new_idx as i64;
                                ui.send_data();
                            }
                            //println!("{:?}", ui.frontend_data.switches[switch_id].channel_groups);
                        })
                        .set(EDITOR_SWITCH_DROP_DOWNS + editor_switch_drop_downs_count, conrod_ui);
                        editor_switch_drop_downs_count += 1;

                    let label = if dropdown_index as i64 == ui.current_edited_channel_group_id {
                        "v".to_string()
                    }
                    else {
                        ">".to_string()
                    };

                    Button::new()
                        .w_h(item_height, item_height)
                        .xy_relative_to(TITLE, [x_pos+(item_width-item_height)/2.0, y_pos])
                        .rgb(0.9, 0.9, 0.1)
                        .frame(1.0)
                        .label(&label)
                        .react(|| {
                            if dropdown_index as i64 == ui.current_edited_channel_group_id {
                                ui.current_edited_channel_group_id = -1;
                            }
                            else {
                                ui.current_edited_channel_group_id = dropdown_index as i64;
                                let mut current_edited_curve_strings_locked = ui.current_edited_curve_strings.lock().unwrap();
                                current_edited_curve_strings_locked[0] = data.curve_in.get_string();
                                current_edited_curve_strings_locked[1] = data.curve_out.get_string();
                            };
                        })
                        .set(EDITOR_SWITCH_BUTTON + editor_switch_button_count, conrod_ui);
                    editor_switch_button_count += 1;
                    y_pos = y_pos - 60.0;


                    if dropdown_index as i64 == ui.current_edited_channel_group_id {

                        for (index, &value) in data.values.iter().enumerate() {
                            let label = {
                                let mut text = "Value: ".to_string();
                                text.push_str(&value.to_string());
                                text
                            };

                            Slider::new(value as f32, 0.0, 255.0)
                                .w_h(item_width - item_x_offset, item_height)
                                .xy_relative_to(TITLE, [x_pos + item_x_offset/2.0, y_pos])
                                .rgb(0.5, 0.3, 0.6)
                                .frame(2.0)
                                .label(&label)
                                .label_color(color::WHITE)
                                .react(|new_value: f32| {
                                    ui.frontend_data.switches[switch_id].channel_groups.get_mut(id_string).unwrap().values[index] = new_value as u8;
                                    ui.send_data();
                                })
                                .set(EDITOR_SWITCH_SLIDER + editor_switch_slider_count, conrod_ui);
                            editor_switch_slider_count += 1;
                            y_pos = y_pos - 60.0;
                        }

                        let mut fade_curve_list = vec!("Linear".to_string(), "Squared".to_string(), "Square root".to_string(), "Custom".to_string());

                        {
                            let data = ui.frontend_data.switches[switch_id].channel_groups.get(id_string).unwrap().clone();

                            let fade_curve_id = data.curve_in.get_id();
                            DropDownList::new(&mut fade_curve_list, &mut Some(fade_curve_id))
                                .w_h(item_width - item_x_offset, item_height)
                                .xy_relative_to(TITLE, [x_pos + item_x_offset/2.0, y_pos])
                                .rgb(0.5, 0.3, 0.6)
                                .frame(2.0)
                                .label(&cloned_ui.frontend_data.fixtures[fixture_id].name.clone())
                                .label_color(color::WHITE)
                                .react(|_: &mut Option<usize>, new_idx, _: &str| {
                                    ui.frontend_data.switches[switch_id].channel_groups.get_mut(id_string).unwrap().curve_in = FadeCurve::get_by_id(new_idx, "x".to_string());
                                    ui.send_data();
                                })
                                .set(EDITOR_SWITCH_DROP_DOWNS + editor_switch_drop_downs_count, conrod_ui);
                            editor_switch_drop_downs_count += 1;
                            y_pos = y_pos - 60.0;

                            if fade_curve_id == 3 {
                                let ref mut curve_string = {ui.current_edited_curve_strings.lock().unwrap()[0].clone()};

                                TextBox::new(curve_string)
                                    .w_h(item_width - item_x_offset, item_height)
                                    .xy_relative_to(TITLE, [x_pos + item_x_offset/2.0, y_pos])
                                    .font_size(14)
                                    .frame(2.0)
                                    .frame_color(bg_color.invert().plain_contrast())
                                    .color(bg_color.plain_contrast())
                                    .react(|new_name: &mut String| {
                                        ui.frontend_data.switches[switch_id].channel_groups.get_mut(id_string).unwrap().curve_in = FadeCurve::Custom(new_name.clone());
                                        ui.send_data();
                                    })
                                    .enabled(true)
                                    .set(EDITOR_CURVE_STRING1, conrod_ui);
                                y_pos = y_pos - 60.0;
                            }

                            let label = {
                                let mut text = "Time in: ".to_string();
                                text.push_str(&data.time_in.to_string());
                                text
                            };

                            Slider::new(data.time_in as f32, 0.0, 10000.0)
                                .w_h(item_width - item_x_offset, item_height)
                                .xy_relative_to(TITLE, [x_pos + item_x_offset/2.0, y_pos])
                                .rgb(0.5, 0.3, 0.6)
                                .frame(2.0)
                                .label(&label)
                                .label_color(color::WHITE)
                                .react(|new_value: f32| {
                                    ui.frontend_data.switches[switch_id].channel_groups.get_mut(id_string).unwrap().time_in = new_value as FadeTime;
                                    ui.send_data();
                                })
                                .set(EDITOR_SWITCH_SLIDER + editor_switch_slider_count, conrod_ui);
                            editor_switch_slider_count += 1;
                            y_pos = y_pos - 60.0;

                            let fade_curve_id = data.curve_out.get_id();
                            DropDownList::new(&mut fade_curve_list, &mut Some(fade_curve_id))
                                .w_h(item_width - item_x_offset, item_height)
                                .xy_relative_to(TITLE, [x_pos + item_x_offset/2.0, y_pos])
                                .rgb(0.5, 0.3, 0.6)
                                .frame(2.0)
                                .label(&cloned_ui.frontend_data.fixtures[fixture_id].name.clone())
                                .label_color(color::WHITE)
                                .react(|_: &mut Option<usize>, new_idx, _: &str| {
                                    ui.frontend_data.switches[switch_id].channel_groups.get_mut(id_string).unwrap().curve_out = FadeCurve::get_by_id(new_idx, "x".to_string());
                                    ui.send_data();
                                })
                                .set(EDITOR_SWITCH_DROP_DOWNS + editor_switch_drop_downs_count, conrod_ui);
                            editor_switch_drop_downs_count += 1;
                            y_pos = y_pos - 60.0;

                            if fade_curve_id == 3 {
                                let ref mut curve_string = {ui.current_edited_curve_strings.lock().unwrap()[1].clone()};

                                TextBox::new(curve_string)
                                    .w_h(item_width - item_x_offset, item_height)
                                    .xy_relative_to(TITLE, [x_pos + item_x_offset/2.0, y_pos])
                                    .frame(2.0)
                                    .frame_color(bg_color.invert().plain_contrast())
                                    .color(bg_color.plain_contrast())
                                    .react(|new_name: &mut String| {
                                        ui.frontend_data.switches[switch_id].channel_groups.get_mut(id_string).unwrap().curve_out = FadeCurve::Custom(new_name.clone());
                                        ui.send_data();
                                    })
                                    .enabled(true)
                                    .set(EDITOR_CURVE_STRING2, conrod_ui);
                                y_pos = y_pos - 60.0;
                            }

                            let label = {
                                let mut text = "Time out: ".to_string();
                                text.push_str(&data.time_out.to_string());
                                text
                            };

                            Slider::new(data.time_out as f32, 0.0, 10000.0)
                                .w_h(item_width - item_x_offset, item_height)
                                .xy_relative_to(TITLE, [x_pos + item_x_offset/2.0, y_pos])
                                .rgb(0.5, 0.3, 0.6)
                                .frame(2.0)
                                .label(&label)
                                .label_color(color::WHITE)
                                .react(|new_value: f32| {
                                    ui.frontend_data.switches[switch_id].channel_groups.get_mut(id_string).unwrap().time_out = new_value as FadeTime;
                                    ui.send_data();
                                })
                                .set(EDITOR_SWITCH_SLIDER + editor_switch_slider_count, conrod_ui);
                            editor_switch_slider_count += 1;
                            y_pos = y_pos - 60.0;
                        }

                        Button::new()
                            .w_h(item_width - item_x_offset, item_height)
                            .xy_relative_to(TITLE, [x_pos + item_x_offset/2.0, y_pos])
                            .rgb(0.9, 0.1, 0.1)
                            .frame(1.0)
                            .label("Delete")
                            .react(|| {
                                ui.frontend_data.remove_channel_group(switch_id, id_string.clone());
                                ui.send_data();
                            })
                            .set(EDITOR_SWITCH_BUTTON + editor_switch_button_count, conrod_ui);
                            editor_switch_button_count += 1;
                        y_pos = y_pos - 60.0;
                    }
                }
                Button::new()
                    .w_h(item_width, item_height)
                    .xy_relative_to(TITLE, [x_pos, y_pos])
                    .rgb(0.1, 0.9, 0.1)
                    .frame(1.0)
                    .label("Add")
                    .react(|| {
                        ui.frontend_data.add_channel_group(switch_id);
                        ui.send_data();
                    })
                    .set(EDITOR_SWITCH_BUTTON + editor_switch_button_count, conrod_ui);
                editor_switch_button_count += 1;
                y_pos = y_pos - 60.0;

                Button::new()
                    .w_h(item_width, item_height)
                    .xy_relative_to(TITLE, [x_pos, y_pos])
                    .rgb(0.9, 0.1, 0.1)
                    .frame(1.0)
                    .label("Delete")
                    .react(|| {
                        ui.frontend_data.remove_switch_with_id(switch_id);
                        ui.current_edited_switch_id.lock().unwrap()[0] = None;
                        ui.send_data();
                    })
                    .set(EDITOR_SWITCH_BUTTON + editor_switch_button_count, conrod_ui);
                // editor_switch_button_count += 1;
                // y_pos = y_pos - 60.0;
            }
            None => {
                Text::new("No switch selected")
                    .xy_relative_to(TITLE, [x_pos, y_pos])
                    .font_size(14)
                    .color(bg_color.plain_contrast())
                    .set(EDITOR_INFO, conrod_ui);
            }
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
                                    .opengl(OPEN_GL).exit_on_esc(true).vsync(true).build().unwrap();

    let mut conrod_ui = {
        let assets = find_folder::Search::KidsThenParents(3, 5)
            .for_folder("assets").unwrap();
        let font_path = assets.join("fonts/NotoSans/NotoSans-Regular.ttf");
        let theme = Theme::default();
        let glyph_cache = Glyphs::new(&font_path, window.factory.clone());
        Ui::new(glyph_cache.unwrap(), theme)
    };

    window.set_ups(1);

    while let Some(event) = window.next() {
        conrod_ui.handle_event(&event);
        window.draw_2d(&event, |c, g| conrod_ui.draw(c, g));

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


fn main() {
    println!("BitDMX frontend v{}-{}", VERSION, GIT_HASH);
    env_logger::init().unwrap();
    let ui = UI::new();
    create_splash_window(ui.clone());
    if {ui.lock().unwrap().watchdog.is_alive()} { create_output_window(ui.clone()); }
}
