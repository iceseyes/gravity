pub mod body;
pub mod runner;
pub mod simulation;
pub mod world;

use crate::physics;
pub use crate::simulator::body::Body;
pub use crate::simulator::world::World;
use std::sync::{Arc, RwLock};
use std::thread::JoinHandle;

use crate::simulator::simulation::run;

pub type Snapshot = Arc<RwLock<World>>;

pub fn random(n_bodies: usize) -> (JoinHandle<()>, Snapshot) {
    let mut world = World::default();
    for _ in 0..n_bodies {
        world.add_body(Body::random());
    }

    run(world, 60.0 * 60.0 * 24.0 * 365.0 * 1e1, 60)
}

pub fn three_bodies_aligned() -> (JoinHandle<()>, Snapshot) {
    let mut world = World::default();

    let mut p = Body::new(1_000_000_000.0, 1.0);
    p.move_to(60.0, 60.0, 0.0);
    world.add_body(p);

    let mut p = Body::new(9_000_000_000_000.0, 9.0);
    p.move_to(50.0, 50.0, 0.0);
    world.add_body(p);

    world.add_body(Body::new(2_000_000_000.0, 2.0));

    run(world, 1e-3, 1000000)
}

pub fn orbit() -> (JoinHandle<()>, Snapshot) {
    let mut world = World::new(physics::G);

    let mut star = Body::new(1.0e30, 9.7e8);
    star.move_to(0.0, 0.0, 0.0);
    world.add_body(star);

    let mut planet = Body::new(1.0e20, 1.0e7);
    planet.move_to(1.0e9, 0.0, 0.0);
    planet.set_velocity(0.0, 258_293.0, 0.0);
    world.add_body(planet);

    run(world, 0.001, 1000000000)
}
