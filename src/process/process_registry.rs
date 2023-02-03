use std::{collections::HashMap, sync::atomic::Ordering};

use crate::pid::PID;

use super::process::{Process, DeadLetterProcess, ActorProcess};

pub struct ProcessRegistry {
    dead_letter: DeadLetterProcess,
    local_processes: HashMap<String, Box<dyn Process>> // TODO: Fix to concurrent map
}

impl ProcessRegistry {
    pub fn get(&self, pid: &PID) -> &dyn Process {
        match self.local_processes.get(&pid.id) {
            Some(p) => 
                if let Some(ap) = p.as_any().downcast_ref::<ActorProcess>() {
                    if ap.dead_flag.load(Ordering::SeqCst) != true {
                        ap
                    } else {
                        &self.dead_letter
                    }
                } else {
                    p.as_ref()
                }
            None => {
                println!("found dead");
                &self.dead_letter
            }
        }
    }
}

impl Default for ProcessRegistry {
    fn default() -> Self {
        ProcessRegistry {
            dead_letter: DeadLetterProcess {},
            local_processes: HashMap::new()
        }
    }
}