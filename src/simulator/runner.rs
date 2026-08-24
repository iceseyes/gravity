use crate::simulator::World;
use crate::simulator::integrator::Integrator;
use crate::simulator::integrator::symplectic_euler::SymplecticEuler;

#[derive(Debug)]
pub struct Runner<I: Integrator> {
    world: World,
    time: f64,
    dt: f64,
    integrator: I,
}

impl<I: Integrator> Runner<I> {
    pub fn new(world: World, dt: f64, integrator: I) -> Self {
        Self {
            world,
            dt,
            time: 0.0,
            integrator,
        }
    }

    pub fn time(&self) -> f64 {
        self.time
    }

    pub fn world(&self) -> &World {
        &self.world
    }

    pub fn run_once(&mut self) {
        self.step();
    }

    pub fn update_world_size(&mut self) {
        self.world.compute_world_size();
    }

    /// Advances the simulation by one fixed timestep using
    /// semi-implicit Euler integration.
    pub fn step(&mut self) {
        self.integrator.step(&mut self.world, self.dt);
        self.time += self.dt;
    }
}

impl Default for Runner<SymplecticEuler> {
    fn default() -> Self {
        Self::new(World::default(), 1e-3, SymplecticEuler)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::physics::{G, Vec3};
    use crate::simulator::Body;
    use crate::simulator::body::{BodyBuilder, distance};
    use crate::simulator::dimension::{Mass, Radius};
    use approx::assert_abs_diff_eq;

    #[test]
    fn test_single_particle() {
        let mut w = World::default();
        w.add_body(BodyBuilder::unitary().build());
        let runner = Runner::new(w, 1.0, SymplecticEuler);
        let p = &runner.world.bodies()[0];

        assert_abs_diff_eq!(p.speed(), 0.0);
    }

    #[test]
    fn test_two_particles_attract_each_other() {
        let mut w = World::default();
        w.add_body(BodyBuilder::unitary().build());
        w.add_body(
            BodyBuilder::unitary()
                .position(Vec3::new(10.0, 0.0, 0.0))
                .build(),
        );

        let mut runner = Runner::new(w, 1e-3, SymplecticEuler);

        runner.step();
        let bodies = runner.world.bodies();

        assert!(bodies[0].speed() > 0.0);
        assert!(bodies[1].speed() > 0.0);

        assert_abs_diff_eq!(bodies[0].velocity_direction(), Vec3::new(1.0, 0.0, 0.0));
        assert_abs_diff_eq!(bodies[1].velocity_direction(), Vec3::new(-1.0, 0.0, 0.0));
    }

    #[test]
    fn test_singularity() {
        // due corpi nello stesso punto: assumiamo che l'accelerazione sia zero.
        let mut w = World::default();
        w.add_body(BodyBuilder::unitary().build());
        w.add_body(
            BodyBuilder::unitary()
                .mass(Mass::kg(1e3).unwrap())
                .radius(Radius::m(10.0).unwrap())
                .build(),
        );

        let mut runner = Runner::new(w, 1e-3, SymplecticEuler);

        let (p1, p2) = (&runner.world.bodies()[0], &runner.world.bodies()[1]);
        assert_abs_diff_eq!(distance(p1, p2), 0.0);

        runner.step();

        let (p1, p2) = (&runner.world.bodies()[0], &runner.world.bodies()[1]);
        assert_abs_diff_eq!(distance(p1, p2), 0.0);
        assert_abs_diff_eq!(p1.speed(), 0.0);
        assert_abs_diff_eq!(p2.speed(), 0.0);
    }

    macro_rules! two_body_test_system {
        ($property: ident) => {
            two_body_test_system!($property, setup (_p1, _p2) {})
        };
        ($property: ident, setup ($p1:ident, $p2:ident) $setup_func: block) => {{
            let mut p1 = BodyBuilder::new(Mass::kg(1e10).unwrap(), Radius::m(1.0).unwrap()).position(Vec3::new(-10.0, 0.0, 0.0)).build();
            let mut p2 = BodyBuilder::new(Mass::kg(1e10).unwrap(), Radius::m(1.0).unwrap()).position(Vec3::new(10.0, 0.0, 0.0)).build();

            (|$p1: &mut Body, $p2: &mut Body| $setup_func)(&mut p1, &mut p2);

            let mut world = World::default();
            world.add_body(p1);
            world.add_body(p2);

            let mut runner = Runner::new(world, 1e-3, SymplecticEuler);
            let initial = runner.world.$property();

            for _ in 0..1000 {
                runner.step();
            }

            (initial, runner.world.$property())
        }};
    }

    #[test]
    fn test_momentum_is_conserved() {
        let (initial, final_) = two_body_test_system!(
            total_momentum,
            setup (p1, p2) {
                p1.set_velocity(Vec3::new(0.0, 100.0, 0.0));
                p2.set_velocity(Vec3::new(0.0, -100.0, 0.0));
            }
        );

        assert_abs_diff_eq!(initial, final_);
    }

    #[test]
    fn test_angular_momentum_is_conserved() {
        let (initial, final_) = two_body_test_system!(
            total_angular_momentum,
            setup (p1, p2) {
                p1.set_velocity(Vec3::new(0.0, 100.0, 0.0));
                p2.set_velocity(Vec3::new(0.0, -100.0, 0.0));
            }
        );

        let error = (final_ - initial).norm() / initial.norm();
        assert!(error < 2e-6, "angular momentum relative error: {error:e}");
    }

    #[test]
    fn test_center_of_mass_does_not_move() {
        let (initial_com, final_com) = two_body_test_system!(center_of_mass);

        assert_abs_diff_eq!(initial_com, final_com);
    }

    #[test]
    fn test_energy_conservation() {
        let (initial_energy, final_energy) = two_body_test_system!(energy);
        let relative_error = ((final_energy - initial_energy) / initial_energy).abs();
        assert!(relative_error < 1e-4, "relative error: {}", relative_error);
    }

    #[test]
    fn test_circular_orbit() {
        let star = BodyBuilder::new(Mass::kg(1e30).unwrap(), Radius::m(1.0e8).unwrap()).build();
        let mut planet =
            BodyBuilder::new(Mass::kg(1e20).unwrap(), Radius::m(1.0e3).unwrap()).build();
        let orbital_radius = 1.0e11_f64;

        planet.in_circular_orbit(G, &star, orbital_radius);

        let mut world = World::default();
        world.add_body(star);
        world.add_body(planet);

        let mut runner = Runner::new(world, 1.0, SymplecticEuler);
        let initial_energy = runner.world.energy();
        let mut min_distance = f64::MAX;
        let mut max_distance = 0.0f64;

        for _ in 0..24_320_750 {
            runner.step();

            let r = distance(&runner.world.bodies()[0], &runner.world.bodies()[1]);

            min_distance = min_distance.min(r);
            max_distance = max_distance.max(r);
        }

        let final_energy = runner.world.energy();
        let radial_error = (max_distance - min_distance) / orbital_radius;
        let relative_energy_error = ((final_energy - initial_energy) / initial_energy).abs();

        assert!(
            relative_energy_error < 0.05,
            "relative energy error: {}",
            relative_energy_error
        );
        assert!(radial_error < 0.05);
    }

    #[test]
    fn test_angular_momentum_is_conserved_in_orbit() {
        let star = BodyBuilder::new(Mass::kg(1e30).unwrap(), Radius::m(1.0e8).unwrap()).build();
        let mut planet =
            BodyBuilder::new(Mass::kg(1e20).unwrap(), Radius::m(1.0e3).unwrap()).build();
        planet.in_circular_orbit(G, &star, 1.0e11);

        let mut world = World::default();
        world.add_body(star);
        world.add_body(planet);

        let mut runner = Runner::new(world, 1.0, SymplecticEuler);

        let initial = runner.world().total_angular_momentum();

        for _ in 0..100_000 {
            runner.step();
        }

        let final_ = runner.world().total_angular_momentum();
        let error = (final_ - initial).norm() / initial.norm();

        assert!(error < 2e-6, "angular momentum relative error: {error}");
    }
}
