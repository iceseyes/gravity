pub mod body;
pub mod runner;
pub mod world;

use crate::physics;
pub use crate::simulator::body::Body;
use crate::simulator::runner::Runner;
pub use crate::simulator::world::World;

pub fn random(n_bodies: usize) -> Runner {
    let mut world = World::default();
    for _ in 0..n_bodies {
        world.add_body(Body::random());
    }

    Runner::new(world, 60.0 * 60.0 * 24.0 * 365.0 * 1e1)
}

pub fn three_bodies_aligned() -> Runner {
    let mut world = World::default();

    let mut p = Body::new(1_000_000_000.0, 1.0);
    p.move_to(60.0, 60.0, 0.0);
    world.add_body(p);

    let mut p = Body::new(9_000_000_000_000.0, 9.0);
    p.move_to(50.0, 50.0, 0.0);
    world.add_body(p);

    world.add_body(Body::new(2_000_000_000.0, 2.0));

    Runner::new(world, 1e-3)
}

pub fn orbit() -> Runner {
    let mut world = World::new(physics::G);

    let mut star = Body::new(1.0e30, 9.7e8);
    star.move_to(0.0, 0.0, 0.0);
    world.add_body(star);

    let mut planet = Body::new(1.0e20, 1.0e7);
    planet.move_to(1.0e9, 0.0, 0.0);
    planet.set_velocity(0.0, 258_293.0, 0.0);
    world.add_body(planet);

    Runner::new(world, 60.0)
}
