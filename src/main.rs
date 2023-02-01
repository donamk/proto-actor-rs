use proto_actor::actor_system::ActorSystem;

fn main() {
    let system = ActorSystem::default();
    println!("{:?}", system);
}
