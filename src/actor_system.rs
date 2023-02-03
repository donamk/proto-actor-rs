use std::collections::HashMap;

use crate::process::{process_registry::ProcessRegistry, process::DeadLetterProcess};

#[derive(Debug)]
pub struct ActorSystemConfig {}

/// Manages all the local actors, their communication and supervision
pub struct ActorSystem {
    config: ActorSystemConfig,
    pub process_registry: ProcessRegistry
}

impl ActorSystem {
    pub fn new(sys_config: ActorSystemConfig) -> Self {
        ActorSystem { 
            config: sys_config, 
            process_registry: ProcessRegistry::default()
        }
    }
}

impl Default for ActorSystem {
    fn default() -> Self {
        ActorSystem { config: ActorSystemConfig {}, process_registry: ProcessRegistry::default() }
    }
}

impl Drop for ActorSystem {
    fn drop(&mut self) {
        print!("dropping actor system");
    }
}