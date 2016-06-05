use std::sync::{Arc, Mutex};
use std::sync::mpsc;
use std::time::Duration;
use std::thread::{self, sleep};

use logic::Stage;
use networking::UDPSocket;

#[derive(Debug, Clone)]
pub struct Chaser {
    pub switches: Vec<usize>,
    pub current_thread: Option<mpsc::Sender<()>>
}

#[derive(Debug, Clone, RustcDecodable, RustcEncodable)]
pub struct FrontendChaser {
    pub switches: Vec<usize>,
    pub current_thread: bool
}

impl FrontendChaser {
    pub fn new() -> FrontendChaser {
        FrontendChaser {
            switches: Vec::new(),
            current_thread: true
        }
    }
    pub fn remove_switch_with_id(&mut self, switch_id: usize) {
        debug!("Removed switch {:?}", switch_id);
        trace!("{:?}", self.switches);
        self.switches.retain(|&id| id != switch_id);
        self.switches = self.switches.iter().map(|x| if *x < switch_id {*x} else {x - 1}).collect();
        trace!("{:?}", self.switches);
    }
}

impl Chaser {
    pub fn new() -> Chaser {
        Chaser {
            switches: Vec::new(),
            current_thread: None
        }
    }
    pub fn from_frontend_data(frontend_chaser: FrontendChaser) -> Chaser {
        Chaser {
            switches: frontend_chaser.switches,
            current_thread: None
        }
    }
    pub fn stop_chaser(&mut self) {
        match self.current_thread {
            Some(ref tx) => {
                if tx.send(()).is_ok() {debug!("Killed chaser")}
            },
            None => {}
        }
        self.current_thread = None;
    }
    pub fn get_frontend_data(&self) -> FrontendChaser {
        FrontendChaser {
            switches: self.switches.clone(),
            current_thread: self.current_thread.is_some()
        }
    }
}

pub fn start_chaser_of_switch(stage: Arc<Mutex<Stage>>, switch_id: usize, dimmer_value: f64) {

    let addr_high = (switch_id >> 8) as u8;
    let addr_low = switch_id as u8;
    UDPSocket::new().start_frontend_client().send_to_multicast(&[2, addr_high, addr_low, dimmer_value as u8]);
    let (chaser, rx) = {
        let mut stage_locked = stage.lock().expect("Failed to lock Arc!");
        let chaser_id = stage_locked.switches[switch_id].clone().chaser_id;
        let mut chaser = stage_locked.chasers.get_mut(&chaser_id).unwrap();
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
