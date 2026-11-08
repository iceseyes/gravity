pub mod body;
pub mod dimension;
pub mod integrator;
pub mod runner;
pub mod simulation;
pub mod world;

use crate::physics;
use crate::physics::Vec3;
pub use crate::simulator::body::Body;
use crate::simulator::body::BodyBuilder;
use crate::simulator::dimension::{Mass, Radius};
use crate::simulator::integrator::Integrator;
use crate::simulator::simulation::{SimulationCommand, SimulationSnapshot, run};
pub use crate::simulator::world::World;
use std::sync::mpsc::Sender;
use std::sync::{Arc, RwLock, mpsc};
use std::thread::JoinHandle;

pub type Snapshot = Arc<RwLock<SimulationSnapshot>>;

pub fn random<I: Integrator + Send + 'static>(
    integrator: I,
    n_bodies: usize,
) -> (JoinHandle<()>, mpsc::Sender<SimulationCommand>, Snapshot) {
    let mut world = World::default();
    for _ in 0..n_bodies {
        world.add_body(Body::random());
    }

    run(world, 60.0 * 60.0 * 24.0 * 365.0 * 1e1, 120, integrator)
}

pub fn three_bodies_aligned<I: Integrator + Send + 'static>(
    integrator: I,
) -> (JoinHandle<()>, Sender<SimulationCommand>, Snapshot) {
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

    run(world, 1e-3, 1000000, integrator)
}

pub fn orbit<I: Integrator + Send + 'static>(
    integrator: I,
) -> (JoinHandle<()>, Sender<SimulationCommand>, Snapshot) {
    let mut world = World::default();

    let star = BodyBuilder::unitary()
        .mass(Mass::kg(1.0e30).unwrap())
        .radius(Radius::m(6.9634e8).unwrap())
        .build();

    let mut planet = BodyBuilder::unitary()
        .mass(Mass::kg(1.0e20).unwrap())
        .radius(Radius::m(6.37814e6).unwrap())
        .build();

    planet.in_circular_orbit(physics::G, &star, 1.0e9, Vec3::new(0.0, 0.0, 1.0));

    world.add_body(star);
    world.add_body(planet);

    run(world, 1e-3, 2000000, integrator)
}

/// Calcola il vettore normale del piano orbitale a partire dall'inclinazione (in gradi)
/// e dalla longitudine del nodo ascendente (in gradi) rispetto al piano di riferimento (X-Y).
fn orbital_normal(inclination_deg: f64, node_deg: f64) -> Vec3 {
    let i = inclination_deg.to_radians();
    let omega = node_deg.to_radians();
    Vec3::new(i.sin() * omega.sin(), -i.sin() * omega.cos(), i.cos())
}

pub fn sol<I: Integrator + Send + 'static>(
    integrator: I,
) -> (JoinHandle<()>, Sender<SimulationCommand>, Snapshot) {
    let mut world = World::default();

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

    // Inclinazioni reali e longitudini del nodo ascendente dei pianeti del sistema solare
    mercury.in_circular_orbit(physics::G, &sol, 5.791e10, orbital_normal(7.00, 48.33));
    venus.in_circular_orbit(physics::G, &sol, 1.082e11, orbital_normal(3.39, 76.68));
    earth.in_circular_orbit(physics::G, &sol, 1.496e11, orbital_normal(0.00, 0.00));
    mars.in_circular_orbit(physics::G, &sol, 2.279e11, orbital_normal(1.85, 49.56));
    jupiter.in_circular_orbit(physics::G, &sol, 7.783e11, orbital_normal(1.30, 100.46));
    saturn.in_circular_orbit(physics::G, &sol, 1.4336e12, orbital_normal(2.49, 113.67));
    uranus.in_circular_orbit(physics::G, &sol, 2.8710e12, orbital_normal(0.77, 74.01));
    neptune.in_circular_orbit(physics::G, &sol, 4.4954e12, orbital_normal(1.77, 131.78));
    pluto.in_circular_orbit(physics::G, &sol, 5.982e12, orbital_normal(17.16, 110.30));

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

    run(world, 1.0, 2000000, integrator)
}
