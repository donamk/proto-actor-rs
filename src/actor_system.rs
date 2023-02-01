#[derive(Debug)]
pub struct ActorSystemConfig {}

#[derive(Debug)]
pub struct ActorSystem {
    config: ActorSystemConfig
}

impl ActorSystem {
    pub fn new(sys_config: ActorSystemConfig) -> Self {
        ActorSystem { config: sys_config }
    }
}

impl Default for ActorSystem {
    fn default() -> Self {
        ActorSystem { config: ActorSystemConfig {} }
    }
}

impl Drop for ActorSystem {
    fn drop(&mut self) {
        print!("dropping actor system");
    }
}