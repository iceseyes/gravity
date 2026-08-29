use crate::physics::Vec3;
use crate::simulator::integrator::Integrator;
use crate::simulator::{Body, World};

#[derive(Debug, Default)]
pub struct VelocityVerlet;

impl Integrator for VelocityVerlet {
    fn step(&mut self, world: &mut World, dt: f64) {
        let old_acceleration = world
            .bodies()
            .iter()
            .map(|p| p.acceleration())
            .collect::<Vec<_>>();
        update_positions(world.mut_bodies(), dt);

        let accelerations = world.accelerations();
        update_velocities(world.mut_bodies(), old_acceleration, accelerations, dt);
    }
}

fn update_positions(bodies: &mut [Body], dt: f64) {
    bodies.iter_mut().for_each(|p| {
        p.move_to(p.position() + p.velocity() * dt + 0.5 * p.acceleration() * dt * dt);
    });
}

fn update_velocities(
    bodies: &mut [Body],
    old_accelerations: Vec<Vec3>,
    accelerations: Vec<Vec3>,
    dt: f64,
) {
    bodies
        .iter_mut()
        .zip(old_accelerations)
        .zip(accelerations)
        .for_each(|((body, old_acc), acc)| {
            let acc = 0.5 * (old_acc + acc);
            body.accelerate(dt, acc);
        });
}

#[cfg(test)]
mod tests {
    use crate::physics::{G, Vec3};
    use crate::simulator::World;
    use crate::simulator::body::BodyBuilder;
    use crate::simulator::dimension::Radius;
    use crate::simulator::integrator::Integrator;
    use crate::simulator::integrator::velocity_verlet::VelocityVerlet;
    use approx::assert_abs_diff_eq;

    #[test]
    fn test_velocity_verlet_two_steps() {
        let b1 = BodyBuilder::unitary()
            .radius(Radius::m(0.01).unwrap())
            .position(Vec3::new(-1.0, 0.0, 0.0))
            .build();
        let b2 = BodyBuilder::unitary()
            .radius(Radius::m(0.01).unwrap())
            .position(Vec3::new(1.0, 0.0, 0.0))
            .build();

        let mut world = World::default();
        world.add_body(b1);
        world.add_body(b2);

        // È importante inizializzare l'accelerazione a0.
        let accelerations = world.accelerations();

        for (body, acceleration) in world.mut_bodies().iter_mut().zip(accelerations) {
            body.set_acceleration(acceleration);
        }

        let dt = 0.1;
        let mut integrator = VelocityVerlet;

        // Stato iniziale:
        //
        // b1: x = -1, v = 0
        // b2: x =  1, v = 0
        //
        // a = G / 4
        let a = G / 4.0;

        // -----------------
        // STEP 1
        // -----------------

        integrator.step(&mut world, dt);

        let b1 = &world.bodies()[0];
        let b2 = &world.bodies()[1];

        // x1 = x0 + v0*dt + 1/2*a*dt²
        let expected_x1 = -1.0 + 0.5 * a * dt * dt;
        let expected_x2 = 1.0 - 0.5 * a * dt * dt;

        // v1 = v0 + 1/2*(a0 + a1)*dt
        //
        // a1 è leggermente maggiore perché i corpi si sono avvicinati.
        let a1 = G / (expected_x2 - expected_x1).powi(2);
        let expected_v1 = 0.5 * (a + a1) * dt;

        assert_abs_diff_eq!(b1.position().x, expected_x1);
        assert_abs_diff_eq!(b2.position().x, expected_x2);
        assert_abs_diff_eq!(b1.velocity().x, expected_v1);
        assert_abs_diff_eq!(b2.velocity().x, -expected_v1);

        // -----------------
        // STEP 2
        // -----------------

        integrator.step(&mut world, dt);

        let b1 = &world.bodies()[0];
        let b2 = &world.bodies()[1];

        // Calcoliamo manualmente l'accelerazione nello stato dopo il primo step.
        let r1 = expected_x2 - expected_x1;
        let a1 = G / r1.powi(2);

        // Posizione dopo il secondo step:
        let expected_x1_step2 = expected_x1 + expected_v1 * dt + 0.5 * a1 * dt * dt;
        let expected_x2_step2 = expected_x2 - expected_v1 * dt - 0.5 * a1 * dt * dt;

        // Nuova accelerazione.
        let r2 = expected_x2_step2 - expected_x1_step2;
        let a2 = G / r2.powi(2);

        let expected_v2 = expected_v1 + 0.5 * (a1 + a2) * dt;

        assert_abs_diff_eq!(b1.position().x, expected_x1_step2);
        assert_abs_diff_eq!(b2.position().x, expected_x2_step2);
        assert_abs_diff_eq!(b1.velocity().x, expected_v2);
        assert_abs_diff_eq!(b2.velocity().x, -expected_v2);
    }
}
