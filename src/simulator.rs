pub mod body;
pub mod runner;
pub mod simulation;
pub mod world;

use crate::physics;
pub use crate::simulator::body::Body;
pub use crate::simulator::world::World;
use std::sync::mpsc::Sender;
use std::sync::{Arc, RwLock, mpsc};
use std::thread::JoinHandle;

use crate::simulator::simulation::{SimulationCommand, SimulationSnapshot, run};

pub type Snapshot = Arc<RwLock<SimulationSnapshot>>;

pub fn random(n_bodies: usize) -> (JoinHandle<()>, mpsc::Sender<SimulationCommand>, Snapshot) {
    let mut world = World::default();
    for _ in 0..n_bodies {
        world.add_body(Body::random());
    }

    run(world, 60.0 * 60.0 * 24.0 * 365.0 * 1e1, 120)
}

pub fn three_bodies_aligned() -> (JoinHandle<()>, Sender<SimulationCommand>, Snapshot) {
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

pub fn orbit() -> (JoinHandle<()>, Sender<SimulationCommand>, Snapshot) {
    let mut world = World::new(physics::G);

    let mut star = Body::new(1.0e30, 4.0e8);
    star.move_to(0.0, 0.0, 0.0);

    let mut planet = Body::new(1.0e20, 1.0e8);
    planet.in_circular_orbit(physics::G, &star, 1.0e9);

    world.add_body(star);
    world.add_body(planet);

    run(world, 1e-3, 2000000)
}

pub fn sol() -> (JoinHandle<()>, Sender<SimulationCommand>, Snapshot) {
    let mut world = World::new(physics::G);

    let mut sol = Body::new(1.989e30, 6.9634e8);
    sol.move_to(0.0, 0.0, 0.0);

    let mut mercury = Body::new(3.301e23, 2.4397e6);
    mercury.in_circular_orbit(physics::G, &sol, 5.79e10);

    let mut venus = Body::new(4.8675e24, 6.0518e6);
    venus.in_circular_orbit(physics::G, &sol, 1.082e11);

    let mut earth = Body::new(5.972e24, 6.37814e6);
    earth.in_circular_orbit(physics::G, &sol, 1.496e11);

    let mut mars = Body::new(6.4171e23, 3.3972e6);
    mars.in_circular_orbit(physics::G, &sol, 2.279e11);

    let mut jupiter = Body::new(1.8986e27, 7.1492e7);
    jupiter.in_circular_orbit(physics::G, &sol, 7.783e11);

    let mut saturn = Body::new(5.6834e26, 6.0268e7);
    saturn.in_circular_orbit(physics::G, &sol, 1.4336e12);

    let mut uranus = Body::new(8.6810e25, 2.5559e7);
    uranus.in_circular_orbit(physics::G, &sol, 2.8710e12);

    let mut neptune = Body::new(1.0243e26, 2.4746e7);
    neptune.in_circular_orbit(physics::G, &sol, 4.4954e12);

    let mut pluto = Body::new(1.303e22, 1.137e6);
    pluto.in_circular_orbit(physics::G, &sol, 5.982e12);

    world.add_body(sol);
    world.add_body(mercury);
    world.add_body(venus);
    world.add_body(earth);
    world.add_body(mars);
    world.add_body(jupiter);
    world.add_body(saturn);
    world.add_body(uranus);
    world.add_body(neptune);
    world.add_body(pluto);

    run(world, 1.0, 2000000)
}
