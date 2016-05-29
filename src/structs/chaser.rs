use std::sync::mpsc;

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
