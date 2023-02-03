use std::any::Any;

use proto_actor::actor_system::ActorSystem;
use proto_actor::pid::PID;

fn main() {
    let system = ActorSystem::default();

    let pid = PID::new("127.0.0.1".to_string(), "asdfasd".to_string());
    println!("pid: {}", pid);
    
    system.process_registry.get(&pid);
}
