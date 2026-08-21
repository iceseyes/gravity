use crate::physics::gravity;
use crate::simulator::World;

#[derive(Debug)]
pub struct Runner {
    world: World,
    time: f64,
    dt: f32,
}

impl Runner {
    pub fn new(world: World, dt: f32) -> Self {
        Self {
            world,
            dt,
            time: 0.0,
        }
    }

    pub fn time(&self) -> f64 {
        self.time
    }

    pub fn world(&self) -> World {
        self.world.clone()
    }

    pub fn run_once(&mut self) {
        self.step();
        self.world.reset_viewport();
    }

    pub fn step(&mut self) {
        let gravity_constant: f32 = self.world.gravity_constant();
        let bodies = self.world.mut_bodies();
        let accelerations: Vec<(f32, f32, f32)> = bodies
            .iter()
            .enumerate()
            .map(|(index, p1)| {
                let (mut ax, mut ay, mut az) = (0.0, 0.0, 0.0);

                bodies.iter().enumerate().for_each(|(index2, p2)| {
                    if index != index2 {
                        let (dx, dy, dz) = p2.distance_to(p1);
                        let (ax_, ay_, az_) = gravity::gravity_field(p2.mass(), dx, dy, dz);
                        ax += ax_;
                        ay += ay_;
                        az += az_;
                    }
                });

                (
                    gravity_constant * ax,
                    gravity_constant * ay,
                    gravity_constant * az,
                )
            })
            .collect();

        bodies.iter_mut().enumerate().for_each(|(index, p)| {
            p.update_position(self.dt);

            let (ax, ay, az) = accelerations[index];
            p.accelerate(self.dt, ax, ay, az);
        });

        self.time += self.dt as f64;
    }
}

impl Default for Runner {
    fn default() -> Self {
        Self::new(World::default(), 1e-3)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::physics::{assert_approx_eq, assert_approx_eq_f64};
    use crate::simulator::Body;
    use crate::simulator::body::distance;

    #[test]
    fn test_single_particle() {
        let mut w = World::default();
        w.add_body(Body::new(1.0, 1.0));
        let runner = Runner::new(w, 1.0);
        let p = &runner.world.bodies()[0];

        assert_approx_eq(p.speed(), 0.0);
    }

    #[test]
    fn test_two_particles_attract_each_other() {
        let mut p1 = Body::new(1.0, 1.0);
        let mut p2 = Body::new(1.0, 1.0);

        p1.move_to(0.0, 0.0, 0.0);
        p2.move_to(10.0, 0.0, 0.0);

        let mut w = World::default();
        w.add_body(p1);
        w.add_body(p2);

        let mut runner = Runner::new(w, 1e-3);

        runner.step();
        let bodies = runner.world.bodies();

        assert!(bodies[0].speed() > 0.0);
        assert!(bodies[1].speed() > 0.0);

        assert_approx_eq(bodies[0].velocity_direction().0, 1.0);
        assert_approx_eq(bodies[0].velocity_direction().1, 0.0);
        assert_approx_eq(bodies[0].velocity_direction().2, 0.0);

        assert_approx_eq(bodies[1].velocity_direction().0, -1.0);
        assert_approx_eq(bodies[1].velocity_direction().1, 0.0);
        assert_approx_eq(bodies[1].velocity_direction().2, 0.0);
    }

    #[test]
    fn test_singularity() {
        // due corpi nello stesso punto: assumiamo che l'accelerazione sia zero.
        let mut w = World::default();
        w.add_body(Body::new(1.0, 1.0));
        w.add_body(Body::new(1000.0, 10.0));

        let mut runner = Runner::new(w, 1e-3);

        let (p1, p2) = (&runner.world.bodies()[0], &runner.world.bodies()[1]);
        assert_approx_eq(distance(p1, p2), 0.0);

        runner.step();

        let (p1, p2) = (&runner.world.bodies()[0], &runner.world.bodies()[1]);
        assert_approx_eq(distance(p1, p2), 0.0);
        assert_approx_eq(p1.speed(), 0.0);
        assert_approx_eq(p2.speed(), 0.0);
    }

    macro_rules! two_bodies_in_equilibrium {
        ($property: ident $(, setup ($p1:ident, $p2:ident) $setup_func: block)?) => {{
            let mut p1 = Body::new(1.0e10, 1.0);
            let mut p2 = Body::new(1.0e10, 1.0);

            p1.move_to(-10.0, 0.0, 0.0);
            p2.move_to(10.0, 0.0, 0.0);

            $((|$p1: &mut Body, $p2: &mut Body| $setup_func)(&mut p1, &mut p2);)?

            let mut world = World::default();
            world.add_body(p1);
            world.add_body(p2);

            let mut runner = Runner::new(world, 1e-3);
            let initial = runner.world.$property();

            for _ in 0..1000 {
                runner.step();
            }

            (initial, runner.world.$property())
        }};
    }

    #[test]
    fn test_momentum_is_conserved() {
        let (initial, final_) = two_bodies_in_equilibrium!(
            total_momentum,
            setup (p1, p2) {
                p1.set_velocity(0.0, 100.0, 0.0);
                p2.set_velocity(0.0, -100.0, 0.0);
            }
        );

        assert_approx_eq_f64(initial.0, final_.0);
        assert_approx_eq_f64(initial.1, final_.1);
        assert_approx_eq_f64(initial.2, final_.2);
    }

    #[test]
    fn test_center_of_mass_does_not_move() {
        let (initial_com, final_com) = two_bodies_in_equilibrium!(center_of_mass);

        assert_approx_eq_f64(initial_com.0, final_com.0);
        assert_approx_eq_f64(initial_com.1, final_com.1);
        assert_approx_eq_f64(initial_com.2, final_com.2);
    }

    #[test]
    fn test_energy_conservation() {
        let (initial_energy, final_energy) = two_bodies_in_equilibrium!(energy);
        let relative_error = ((final_energy - initial_energy) / initial_energy).abs();
        assert!(relative_error < 1e-4, "relative error: {}", relative_error);
    }

    #[test]
    fn test_circular_orbit() {
        let star = Body::new(1.0e30, 1.0e8);
        let mut planet = Body::new(1.0e20, 1.0e3);
        let orbital_radius = 1.0e11_f64;

        planet.in_circular_orbit(&star, orbital_radius as f32);

        let mut world = World::default();
        world.add_body(star);
        world.add_body(planet);

        let mut runner = Runner::new(world, 1.0);
        let initial_energy = runner.world.energy();
        let mut min_distance = f64::MAX;
        let mut max_distance = 0.0f64;

        for _ in 0..24_320_750 {
            runner.step();

            let r = distance(&runner.world.bodies()[0], &runner.world.bodies()[1]) as f64;

            min_distance = min_distance.min(r);
            max_distance = max_distance.max(r);
        }

        let final_energy = runner.world.energy();
        let radial_error = (max_distance - min_distance) / orbital_radius;
        let energy_loss = 100.0 * (final_energy - initial_energy) / initial_energy;

        // the energy loss is less than 2% of the initial energy, so we can tolerate a 5% error
        assert!(energy_loss > -2.0);
        assert!(radial_error < 0.05);
    }
}
