use crate::physics::{Vec3, gravity};
use crate::simulator::World;
use crate::simulator::integrator::Integrator;

pub struct SymplecticEuler;

impl Integrator for SymplecticEuler {
    fn step(&mut self, world: &mut World, dt: f64) {
        let gravity_constant = world.gravity_constant();
        let bodies = world.mut_bodies();
        let accelerations: Vec<Vec3> = bodies
            .iter()
            .enumerate()
            .map(|(index, p1)| {
                let a = bodies
                    .iter()
                    .enumerate()
                    .fold(Vec3::zeros(), |a, (index2, p2)| {
                        if index != index2 {
                            let d = p2.distance_to(p1);
                            a + gravity::gravity_field(p2.mass(), d)
                        } else {
                            a
                        }
                    });
                gravity_constant * a
            })
            .collect();

        bodies.iter_mut().enumerate().for_each(|(index, p)| {
            p.update_position(dt);

            let acc = accelerations[index];
            p.accelerate(dt, acc);
        });
    }
}
