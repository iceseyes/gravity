use crate::simulator::World;
use crate::simulator::integrator::Integrator;

pub struct SymplecticEuler;

impl Integrator for SymplecticEuler {
    fn step(&mut self, world: &mut World, dt: f64) {
        let accelerations = world.accelerations();
        world
            .mut_bodies()
            .iter_mut()
            .enumerate()
            .for_each(|(index, p)| {
                p.update_position(dt);

                let acc = accelerations[index];
                p.accelerate(dt, acc);
            });
    }
}
