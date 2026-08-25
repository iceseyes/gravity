use crate::physics::Vec3;
use crate::simulator::integrator::Integrator;
use crate::simulator::{Body, World};

pub struct VelocityVerlet;

impl Integrator for VelocityVerlet {
    fn step(&mut self, world: &mut World, dt: f64) {
        update_positions(world.mut_bodies(), dt);

        let accelerations = world.accelerations();
        update_velocities(world.mut_bodies(), accelerations, dt);
    }
}

fn update_positions(bodies: &mut [Body], dt: f64) {
    bodies.iter_mut().for_each(|p| {
        p.move_to(p.position() + p.velocity() * dt + 0.5 * p.last_acceleration() * dt * dt);
    });
}

fn update_velocities(bodies: &mut [Body], accelerations: Vec<Vec3>, dt: f64) {
    bodies.iter_mut().enumerate().for_each(|(index, p)| {
        let acc = accelerations[index];
        p.accelerate(dt, acc);
    });
}
