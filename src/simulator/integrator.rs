pub mod symplectic_euler;

use crate::simulator::World;

pub trait Integrator {
    fn step(&mut self, world: &mut World, dt: f64);
}
