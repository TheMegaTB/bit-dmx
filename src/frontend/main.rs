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
    Slider,
    TextBox,
    // Toggle,
    // WidgetMatrix,
    // XYPad,
};
use piston_window::{ EventLoop, Glyphs, PistonWindow, UpdateEvent, WindowSettings, PressEvent, ReleaseEvent, Window };
use rustc_serialize::json;


type Backend = (<piston_window::G2d<'static> as conrod::Graphics>::Texture, Glyphs);
type Ui = conrod::Ui<Backend>;
type UiCell<'a> = conrod::UiCell<'a, Backend>;


widget_ids! {
    CANVAS,
    TITLE,
    CONNECTED_BUTTON,
    EDITOR_BUTTON,
    EDITOR_TITLE,
    EDITOR_INFO,
    EDITOR_TIME_SLIDER,
    EDITOR_CONTENT with 4000,
    BUTTON with 4000,
    CHASER_TITLE with 4000,
    EDITOR_SWITCH_ELEMENT with 4000
}

struct UI {
    pub watchdog: WatchDogClient,
    tx: mpsc::Sender<Vec<u8>>,
    frontend_data: FrontendData,
    shift_state: bool,
    edit_state: bool,
    current_edited_switch_id: Arc<Mutex<[Option<usize>; 1]>>,
    current_edited_switch_name: Arc<Mutex<[String; 1]>>
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
            shift_state: false,
            edit_state: false,
            current_edited_switch_id: Arc::new(Mutex::new([None])),
            current_edited_switch_name: Arc::new(Mutex::new(["".to_string()]))
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
        event.update(|_| conrod_ui.set_widgets(|mut conrod_ui| set_widgets(&mut conrod_ui, &mut ui_locked, chasers.clone(), window.size().width)));
        window.draw_2d(&event, |c, g| conrod_ui.draw_if_changed(c, g));
    }
}

fn set_widgets(mut conrod_ui: &mut UiCell, ui: &mut UI, chasers: Vec<String>, window_width: u32) {
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
            // let mut current_edited_switch_locked = ui.current_edited_switch.lock().unwrap();
            // current_edited_switch_locked[0] = None;

            ui.current_edited_switch_id.lock().unwrap()[0] = None;

            ui.current_edited_switch_name.lock().unwrap()[0] = "".to_string();
            println!("reset the name");
            ui.edit_state = !ui.edit_state;
        })
        .set(EDITOR_BUTTON, conrod_ui);

    // let mut id = None;
    let tx = ui.tx.clone();

    let button_width = 200.0;
    let button_height = 50.0;
    let mut current_button_id = BUTTON;

    let chasers = {
        let mut tmp_chasers = Vec::new();
        for chaser_name in chasers.iter() {
            if ui.frontend_data.chasers.get(chaser_name).is_some() {
                tmp_chasers.push(chaser_name.clone());
            }
        }
        tmp_chasers
    };

    let mut next_y_offset = 0f64;
    let mut x_pos = -button_width;
    let mut y_offset = -90.0;
    let mut rightmost = 0.0;


    for (id, (name, chaser)) in chasers.iter().map(|x| (x, ui.frontend_data.chasers.get(x).unwrap())).enumerate() {
        x_pos = x_pos + button_width;
        //let x_pos = (id as f64 - 0.5) * button_width;
        let usable_width = if ui.edit_state {2*window_width/3} else {window_width};
        if (x_pos + 1.5*button_width) as u32 >= usable_width {
            rightmost = x_pos + 0.5*button_width;
            x_pos = 0.0;
            y_offset = next_y_offset;
        }
        let mut last_active_switch_id = None;
        Text::new(name)
            .xy_relative_to(TITLE, [x_pos, y_offset])
            .font_size(15)
            .color(bg_color.plain_contrast())
            .set(CHASER_TITLE + id, conrod_ui);

        for (switch_id_in_chaser, (switch_id, switch)) in chaser.switches.iter().map(|&switch_id| (switch_id, &ui.frontend_data.switches[switch_id])).enumerate() {
            let y_pos = y_offset - 50.0 - switch_id_in_chaser as f64*button_height;
            let current_edited_switch = ui.current_edited_switch_id.clone();
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
        let y_pos = y_offset - 50.0 - (chaser.switches.len() as f64 - 0.25)*button_height;
        if y_pos - button_height < next_y_offset {
            next_y_offset = y_pos - button_height;
        }
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
                let line = "-----------------------------------------";
                let ref mut switch_name = switch_name.lock().unwrap()[0];
                //println!("name: {:?}", switch_name);

                TextBox::new(switch_name)
                    .font_size(20)
                    .xy_relative_to(TITLE, [x_pos, y_pos])
                    .w_h(item_width, item_height)
                    .frame(2.0)
                    .frame_color(bg_color.invert().plain_contrast())
                    .color(bg_color.plain_contrast())
                    .react(|new_name: &mut String| {
                        println!("changed name: {:?}", new_name);
                        ui.frontend_data.switches[switch_id].name = new_name.clone();
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
                        println!("Changed time: {:?}", new_time);})
                    .set(EDITOR_TIME_SLIDER, conrod_ui);


                y_pos = y_pos - 60.0;
                let mut editor_switch_element_count = 0;


                Text::new(line)
                    .xy_relative_to(TITLE, [x_pos, y_pos])
                    .font_size(14)
                    .color(bg_color.plain_contrast())
                    .set(EDITOR_SWITCH_ELEMENT + editor_switch_element_count, conrod_ui);
                editor_switch_element_count += 1;
                y_pos = y_pos - 60.0;

                // for (id_string, data) in ui.frontend_data.switches[switch_id].channel_groups.iter() {
                //     let mut id_vector: Vec<String> = id_string.split(",").map(|x| x.to_string()).collect();
                //     id_vector[0].remove(0);
                //     id_vector[1].pop();
                //     let fixture_id = id_vector[0].parse::<usize>();
                //     let channel_group_id = id_vector[1].parse::<usize>();
                //     println!("{:?}", id_vector);
                //
                //     for (index, &value) in data.0.iter().enumerate() {
                //         let label = {
                //             let mut text = "Value: ".to_string();
                //             text.push_str(&value.to_string());
                //             text
                //         };
                //
                //         Slider::new(value as f32, 0.0, 255.0)
                //             .w_h(item_width, item_height)
                //             .xy_relative_to(TITLE, [x_pos, y_pos])
                //             .rgb(0.5, 0.3, 0.6)
                //             .frame(2.0)
                //             .label(&label)
                //             .label_color(color::WHITE)
                //             .react(|new_value: f32| {})
                //             .set(EDITOR_TIME_SLIDER, conrod_ui);
                //         editor_switch_element_count += 1;
                //         y_pos = y_pos - 60.0;
                //     }
                //
                //     println!("{:?}", data);
                // }


                Button::new()
                    .w_h(item_width, item_height)
                    .xy_relative_to(TITLE, [x_pos, y_pos])
                    .rgb(0.9, 0.9, 0.1)
                    .frame(1.0)
                    .label("Add")
                    .react(|| {})
                    .set(EDITOR_SWITCH_ELEMENT + editor_switch_element_count, conrod_ui);
                editor_switch_element_count += 1;
                y_pos = y_pos - 60.0;


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
    let chasers = lines_from_file();
    let ui = UI::new();
    create_splash_window(ui.clone());
    create_output_window(ui.clone(), chasers);
}
