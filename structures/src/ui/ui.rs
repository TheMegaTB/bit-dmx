use std::sync::{Arc, Mutex, mpsc};
use std::net::{TcpStream, SocketAddr};
use std::thread;
use std::io::prelude::*;
use std::time::Duration;
use rustc_serialize::json;
use std::path::PathBuf;


use io::config::{get_config_path, Config};
use networking::UDPSocket;
use ui::FrontendData;
use networking::WatchDogClient;
use networking::WATCHDOG_TTL;
use super::frontend_config::FrontendConfig;
use VERSION;
use GIT_HASH;

#[derive(Debug, Clone)]
/// The struct to save all important information about the frontend ui.
pub struct UI {
    /// The watchdog client to request the server ip.
    pub watchdog: WatchDogClient,
    /// The sender to interrupt the watchdog client.
    pub tx: mpsc::Sender<Vec<u8>>,
    /// The frontend data struct to save the current project state.
    pub frontend_data: FrontendData,
    /// The configuration of the frontend.
    pub config: FrontendConfig,
    /// The shift key state.
    pub shift_state: bool,
    /// The control key state.
    pub control_state: bool,
    /// The alt key state.
    pub alt_state: bool,
    /// The editor state.
    pub editor_state: bool,
    /// The editor state.
    pub show_ip: bool,
    /// Saves whether the ui waites for a new keybinding.
    pub waiting_for_keybinding: bool,
    /// This is set to true after a DropDownList element was selected, so that buttons below the opend DropDownList aren't triggert
    pub dropdown_clicked: bool,
    /// The id of the switch that is opened in the editor.
    pub current_editor_switch_id: Arc<Mutex<[Option<usize>; 1]>>,
    /// The id of the channel group that is opened in the editor.
    pub current_editor_channel_group_id: i64,
    /// The name of the opened switch.
    pub current_editor_switch_name: Arc<Mutex<[String; 1]>>,
    /// The custom curve strings that are opened in the editor.
    pub current_editor_curve_strings: Arc<Mutex<[String; 2]>>,
    /// The list of all chaser names for the editor.
    pub current_editor_chaser_names: Arc<Mutex<Vec<String>>>
}

impl UI {
    /// Createsan ui struct with default values and initialized services.
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

        let ui = UI {
            watchdog: watchdog,
            tx: tx,
            frontend_data: frontend_data,
            config: FrontendConfig::empty(),
            shift_state: false,
            control_state: false,
            alt_state: false,
            editor_state: false,
            show_ip: false,
            waiting_for_keybinding: false,
            dropdown_clicked: false,
            current_editor_switch_id: Arc::new(Mutex::new([None])),
            current_editor_channel_group_id: -1,
            current_editor_switch_name: Arc::new(Mutex::new(["".to_string()])),
            current_editor_curve_strings: Arc::new(Mutex::new(["".to_string(), "".to_string()])),
            current_editor_chaser_names: Arc::new(Mutex::new(Vec::new()))
        };

        let ui = Arc::new(Mutex::new(ui));

        UI::start_udp_server(ui.clone(), UDPSocket::new());
        UI::start_watchdog_client(ui.clone(), UDPSocket::new());
        ui
    }

    /// Enters the edit Mode
    pub fn toggle_edit_mode(&mut self) {
        lock!(self.current_editor_switch_id)[0] = None;
        lock!(self.current_editor_switch_name)[0] = "".to_string();
        if self.editor_state {
            self.send_data();
        }
        else {
            self.current_editor_chaser_names = Arc::new(Mutex::new(self.config.chasers.clone()));
        }
        self.editor_state = !self.editor_state;
    }

    /// Return the path to the client configration of the current project.
    pub fn get_chaser_config_path(&self) -> PathBuf {
        get_config_path(Config::Client).join(self.frontend_data.name.clone()  + ".local.dmx")
    }

    /// Load the client configration.
    pub fn load_chaser_config(&mut self) {
        match FrontendConfig::load(self.get_chaser_config_path()) {
            Some(config) => self.config = config,
            None => {
                self.config.chasers = self.frontend_data.chasers.keys().map(|x| x.clone()).collect();
            }
        }
    }

    /// Save the client configuration.
    pub fn save_chaser_config(&self) {
        self.config.save(self.get_chaser_config_path());
    }

    /// Start th UDP server th receive data (refresh, channel, switch and chaser)
    pub fn start_udp_server(ui: Arc<Mutex<UI>>, mut socket: UDPSocket) {
        thread::spawn(move || {
            let socket = socket.start_frontend_server();
            loop {
                let buf = socket.receive().0;
                let mut ui_locked = lock!(ui);

                if buf == [255, 255, 255, 255] {
                    ui_locked.fetch_data();
                } else {
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
                        let chaser_name = ui_locked.frontend_data.switches[address as usize].clone().chaser_name;
                        let mut chaser = ui_locked.frontend_data.chasers.get_mut(&chaser_name).unwrap();

                        chaser.current_thread = value != 0;
                    }
                }
                trace!("{:?}", buf);
            }
        });
    }

    /// Start the watchdog server to receive the ip of the server
    pub fn start_watchdog_client(ui: Arc<Mutex<UI>>, mut socket: UDPSocket) {
        let sock = socket.assemble_socket(Some(2));
        {
            let (s, s_addr) = {
                let ui_locked = lock!(ui);
                (ui_locked.watchdog.state.clone(), ui_locked.watchdog.server_addr.clone())
            };
            thread::Builder::new().name("WatchDog-Client".to_string()).spawn(move || {
                sock.set_read_timeout(Some(Duration::from_secs(WATCHDOG_TTL + 1))).unwrap();
                let payload = VERSION.to_string() + &GIT_HASH.to_string();
                let mut buf = (0..(payload.as_bytes().len())).map(|_| 0).collect::<Vec<_>>();
                loop {
                    trace!("watchdog message");
                    match sock.recv_from(&mut buf) {
                        Ok((_, addr)) => {
                            if buf == payload.as_bytes() {
                                trace!("received valid watchdog data");
                                lock!(s)[0] = true;
                                let ip_changed = {
                                    let mut s_addr_locked = lock!(s_addr);
                                    if s_addr_locked[0] != Some(addr.ip()) {
                                        s_addr_locked[0] = Some(addr.ip());
                                        info!("Discovered new server @ {:?}", addr.ip());
                                        true
                                    } else {false}
                                };
                                if ip_changed && !lock!(ui).fetch_data() {
                                    lock!(s_addr)[0] = None;
                                }
                            } else {
                                trace!("received invalid watchdog data");
                                lock!(s)[0] = false;
                                lock!(s_addr)[0] = None;
                            }
                        },
                        Err(_) => {
                            trace!("watchdog timeout");
                            lock!(s)[0] = false;
                            lock!(s_addr)[0] = None;
                        }
                    }
                }
            }).unwrap();
        }
    }

    /// Fetch the data from the server. This function return whether the request was successful.
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

    /// Send the local configuration to the server. This function return whether the request was successful.
    pub fn send_data(&mut self) -> bool {
        if self.watchdog.get_server_addr().is_some() {
            match TcpStream::connect((&*self.watchdog.get_server_addr().unwrap().to_string(), 8001)) {
                Ok(mut stream) => {
                    stream.write(self.frontend_data.get_json_string().as_bytes()).unwrap();
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
