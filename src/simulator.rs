pub mod body;
pub mod dimension;
pub mod runner;
pub mod simulation;
pub mod world;

use crate::physics;
use crate::physics::Vec3;
pub use crate::simulator::body::Body;
use crate::simulator::body::BodyBuilder;
use crate::simulator::dimension::{Mass, Radius};
use crate::simulator::simulation::{SimulationCommand, SimulationSnapshot, run};
pub use crate::simulator::world::World;
use std::sync::mpsc::Sender;
use std::sync::{Arc, RwLock, mpsc};
use std::thread::JoinHandle;

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

    world.add_body(
        BodyBuilder::unitary()
            .mass(Mass::kg(1e10).unwrap())
            .position(Vec3::new(60.0, 60.0, 0.0))
            .build(),
    );
    world.add_body(
        BodyBuilder::unitary()
            .mass(Mass::kg(9_000_000_000_000.0).unwrap())
            .radius(Radius::m(9.0).unwrap())
            .position(Vec3::new(50.0, 50.0, 0.0))
            .build(),
    );
    world.add_body(
        BodyBuilder::unitary()
            .mass(Mass::kg(2_000_000_000.0).unwrap())
            .radius(Radius::m(2.0).unwrap())
            .position(Vec3::new(0.0, 0.0, 0.0))
            .build(),
    );

    run(world, 1e-3, 1000000)
}

pub fn orbit() -> (JoinHandle<()>, Sender<SimulationCommand>, Snapshot) {
    let mut world = World::new(physics::G);

    let star = BodyBuilder::unitary()
        .mass(Mass::kg(1.0e30).unwrap())
        .radius(Radius::m(6.9634e8).unwrap())
        .build();

    let mut planet = BodyBuilder::unitary()
        .mass(Mass::kg(1.0e20).unwrap())
        .radius(Radius::m(6.37814e6).unwrap())
        .build();

    planet.in_circular_orbit(physics::G, &star, 1.0e9);

    world.add_body(star);
    world.add_body(planet);

    run(world, 1e-3, 2000000)
}

pub fn sol() -> (JoinHandle<()>, Sender<SimulationCommand>, Snapshot) {
    let mut world = World::new(physics::G);

    let sol = BodyBuilder::unitary()
        .mass(Mass::kg(1.989e30).unwrap())
        .radius(Radius::m(6.9634e8).unwrap())
        .build();

    let mut mercury = BodyBuilder::unitary()
        .mass(Mass::kg(3.301e23).unwrap())
        .radius(Radius::m(2.4397e6).unwrap())
        .build();

    let mut venus = BodyBuilder::unitary()
        .mass(Mass::kg(4.8675e24).unwrap())
        .radius(Radius::m(6.0518e6).unwrap())
        .build();

    let mut earth = BodyBuilder::unitary()
        .mass(Mass::kg(5.972e24).unwrap())
        .radius(Radius::m(6.37814e6).unwrap())
        .build();

    let mut mars = BodyBuilder::unitary()
        .mass(Mass::kg(6.4171e23).unwrap())
        .radius(Radius::m(3.3972e6).unwrap())
        .build();

    let mut jupiter = BodyBuilder::unitary()
        .mass(Mass::kg(1.8986e27).unwrap())
        .radius(Radius::m(7.1492e7).unwrap())
        .build();

    let mut saturn = BodyBuilder::unitary()
        .mass(Mass::kg(5.6834e26).unwrap())
        .radius(Radius::m(6.0268e7).unwrap())
        .build();

    let mut uranus = BodyBuilder::unitary()
        .mass(Mass::kg(8.6810e25).unwrap())
        .radius(Radius::m(2.5559e7).unwrap())
        .build();

    let mut neptune = BodyBuilder::unitary()
        .mass(Mass::kg(1.0243e26).unwrap())
        .radius(Radius::m(2.4746e7).unwrap())
        .build();

    let mut pluto = BodyBuilder::unitary()
        .mass(Mass::kg(1.303e22).unwrap())
        .radius(Radius::m(1.137e6).unwrap())
        .build();

    mercury.in_circular_orbit(physics::G, &sol, 5.791e10);
    venus.in_circular_orbit(physics::G, &sol, 1.082e11);
    earth.in_circular_orbit(physics::G, &sol, 1.496e11);
    mars.in_circular_orbit(physics::G, &sol, 2.279e11);
    jupiter.in_circular_orbit(physics::G, &sol, 7.783e11);
    saturn.in_circular_orbit(physics::G, &sol, 1.4336e12);
    uranus.in_circular_orbit(physics::G, &sol, 2.8710e12);
    neptune.in_circular_orbit(physics::G, &sol, 4.4954e12);
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
