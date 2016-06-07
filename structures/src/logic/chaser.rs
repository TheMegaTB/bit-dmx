use std::sync::{Arc, Mutex};
use std::sync::mpsc;
use std::time::Duration;
use std::thread::{self, sleep};

use logic::Stage;
use networking::UDPSocket;

#[derive(Debug, Clone)]
/// A struct that represents a chaser in the backend
pub struct Chaser {
    /// A list of ids of switches, which are part of this chaser
    pub switches: Vec<usize>,
    /// The sender to interrupt the current chaser thread
    pub current_thread: Option<mpsc::Sender<()>>
}

#[derive(Debug, Clone, RustcDecodable, RustcEncodable)]
/// The frontend representation of a chaser
pub struct FrontendChaser {
    /// A list of ids of switches, which are part of this chaser
    pub switches: Vec<usize>,
    /// The bool to save if the chaser thread is running
    pub current_thread: bool
}

impl FrontendChaser {
    /// Create an empty Chaser
    pub fn new() -> FrontendChaser {
        FrontendChaser {
            switches: Vec::new(),
            current_thread: true
        }
    }
    /// Remove a switch with a given switch id from the chaser. This function also changes the ids of all switches
    /// with a higher id.
    pub fn remove_switch_with_id(&mut self, switch_id: usize) {
        debug!("Removed switch {:?}", switch_id);
        self.switches.retain(|&id| id != switch_id);
        self.switches = self.switches.iter().map(|x| if *x < switch_id {*x} else {x - 1}).collect();
    }
}

impl Chaser {
    /// Create an empty Chaser
    pub fn new() -> Chaser {
        Chaser {
            switches: Vec::new(),
            current_thread: None
        }
    }
    /// Converts a frontend chaser to a backend chaser
    pub fn from_frontend_data(frontend_chaser: FrontendChaser) -> Chaser {
        Chaser {
            switches: frontend_chaser.switches,
            current_thread: None
        }
    }
    /// Stops the chaser thread if it is running
    pub fn stop_chaser(&mut self) {
        match self.current_thread {
            Some(ref tx) => {
                if tx.send(()).is_ok() {debug!("Killed chaser")}
            },
            None => {}
        }
        self.current_thread = None;
    }

    /// Converts a backend chaser to a frontend chaser
    pub fn get_frontend_data(&self) -> FrontendChaser {
        FrontendChaser {
            switches: self.switches.clone(),
            current_thread: self.current_thread.is_some()
        }
    }
}

/// This function starts a chaser thread for a chaser of a given switch
pub fn start_chaser_of_switch(stage: Arc<Mutex<Stage>>, switch_id: usize, dimmer_value: f64) {
    let addr_high = (switch_id >> 8) as u8;
    let addr_low = switch_id as u8;
    UDPSocket::new().start_frontend_client().send_to_multicast(&[2, addr_high, addr_low, dimmer_value as u8]);
    let (chaser, rx) = {
        let mut stage_locked = stage.lock().expect("Failed to lock Arc!");
        let chaser_name = stage_locked.switches[switch_id].clone().chaser_name;
        let mut chaser = stage_locked.chasers.get_mut(&chaser_name).unwrap();
            chaser.stop_chaser();
        if dimmer_value == 0.0 {
            return
        }
        let (tx, rx) = mpsc::channel();
        chaser.current_thread = Some(tx);
        (chaser.clone(), rx)
    };
    thread::spawn(move || {
        let mut current_switch_id_in_chaser: usize = 0; //TODO use switch_id
        loop {
            {stage.lock().expect("Failed to lock Arc!").deactivate_group_of_switch(chaser.switches[current_switch_id_in_chaser], false);}
            {stage.lock().expect("Failed to lock Arc!").set_switch(chaser.switches[current_switch_id_in_chaser], dimmer_value, true);}
            let sleep_time = {
                let stage_locked = stage.lock().expect("Failed to lock Arc!");
                stage_locked.switches[chaser.switches[current_switch_id_in_chaser]].before_chaser as u64
            };
            sleep(Duration::from_millis(sleep_time));
            if rx.try_recv().is_ok() { return };
            current_switch_id_in_chaser = (current_switch_id_in_chaser + 1) % chaser.switches.len();
        }
    });
}
