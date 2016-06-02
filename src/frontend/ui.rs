use std::sync::{Arc, Mutex, mpsc};
use structures::*;
use std::net::{TcpStream, SocketAddr};
use std::thread;
use std::io::prelude::*;
use std::io::{BufReader, BufWriter};
use std::fs::File;
use std::time::Duration;
use rustc_serialize::json;
use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct UI {
    pub watchdog: WatchDogClient,
    pub tx: mpsc::Sender<Vec<u8>>,
    pub frontend_data: FrontendData,
    pub shift_state: bool,
    pub control_state: bool,
    pub alt_state: bool,
    pub edit_state: bool,
    pub chasers: Vec<String>,
    pub waiting_for_keybinding: bool,
    pub current_edited_switch_id: Arc<Mutex<[Option<usize>; 1]>>,
    pub current_edited_channel_group_id: i64,
    pub current_edited_switch_name: Arc<Mutex<[String; 1]>>,
    pub current_edited_curve_strings: Arc<Mutex<[String; 2]>>,
    pub current_edited_chaser_names: Arc<Mutex<Vec<String>>>
}

impl UI {
    pub fn new() -> Arc<Mutex<UI>> {
        let mut socket = UDPSocket::new();
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
                        warn!("Could not send data since no server is available");
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
            control_state: false,
            alt_state: false,
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

    pub fn get_chaser_config_path(&self) -> PathBuf {
        get_config_path(Config::Client, &self.frontend_data.name).join(self.frontend_data.name.clone()  + ".local.dmx")
    }

    pub fn load_chaser_config(&mut self) {
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

    pub fn save_chaser_config(&self) {
        let path = self.get_chaser_config_path();
        let file = File::create(path).expect("no such file");
        let mut buf = BufWriter::new(file);
        for line in self.chasers.iter() {
            buf.write_all((line.clone() + "\n").as_bytes()).unwrap();
        }
    }

    pub fn start_udp_server(ui: Arc<Mutex<UI>>, mut socket: UDPSocket) {
        thread::spawn(move || {
            let socket = socket.start_frontend_server();
            loop {
                let buf = socket.receive().0;
                let mut ui_locked = ui.lock().expect("Failed to lock Arc!");

                // if buf == [255, 255, 255, 255] {
                //     ui_locked.fetch_data();
                // } else {
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
                // }
                trace!("{:?}", buf);
            }
        });
    }

    pub fn start_watchdog_client(ui: Arc<Mutex<UI>>, mut socket: UDPSocket) {
        let sock = socket.assemble_socket(Some(2));
        {
            let (s, s_addr) = {
                let ui_locked = ui.lock().expect("Failed to lock Arc!");
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
                                s.lock().expect("Failed to lock Arc!")[0] = true;
                                let ip_changed = {
                                    let mut s_addr_locked = s_addr.lock().expect("Failed to lock Arc!");
                                    if s_addr_locked[0] != Some(addr.ip()) {
                                        s_addr_locked[0] = Some(addr.ip());
                                        info!("Discovered new server @ {:?}", addr.ip());
                                        true
                                    } else {false}
                                };
                                if ip_changed && !ui.lock().expect("Failed to lock Arc!").fetch_data() {
                                    s_addr.lock().expect("Failed to lock Arc!")[0] = None;
                                }
                            } else {
                                trace!("received invalid watchdog data");
                                s.lock().expect("Failed to lock Arc!")[0] = false;
                                s_addr.lock().expect("Failed to lock Arc!")[0] = None;
                            }
                        },
                        Err(_) => {
                            trace!("watchdog timeout");
                            s.lock().expect("Failed to lock Arc!")[0] = false;
                            s_addr.lock().expect("Failed to lock Arc!")[0] = None;
                        }
                    }
                }
            }).unwrap();
        }
    }

    pub fn fetch_data(&mut self) -> bool {
        if self.watchdog.get_server_addr().is_some() {
            match TcpStream::connect((&*self.watchdog.get_server_addr().unwrap().to_string(), 8000)) {
                Ok(mut stream) => {
                    let mut buffer = String::new();
                    let _ = stream.read_to_string(&mut buffer);
                    self.frontend_data = json::decode(&buffer).unwrap();
                    debug!("Received TCP update from \"{}\"", self.frontend_data.name);
                    self.load_chaser_config();
                    true
                }
                Err(_) => {
                    warn!("Error while connecting to TCP socket.");
                    false
                }
            }
        } else {
            warn!("No server ip available. Even though we received a watchdog signal.");
            false
        }
    }

    pub fn send_data(&mut self) -> bool {
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
            warn!("No server available to send data to.");
            false
        }
    }
}
