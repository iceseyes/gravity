use crate::physics::gravity;
use crate::simulator::World;

#[derive(Debug)]
pub struct Runner {
    world: World,
    time: f64,
    dt: f64,
}

impl Runner {
    pub fn new(world: World, dt: f64) -> Self {
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
    }

    pub fn update_world_size(&mut self) {
        self.world.compute_world_size();
    }

    pub fn step(&mut self) {
        let gravity_constant = self.world.gravity_constant() as f32;
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
            p.update_position(self.dt as f32);

            let (ax, ay, az) = accelerations[index];
            p.accelerate(self.dt as f32, ax, ay, az);
        });

        self.time += self.dt;
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
    use crate::physics::{G, Vec3, assert_approx_eq, assert_approx_eq_f64};
    use crate::simulator::Body;
    use crate::simulator::body::{BodyBuilder, distance};
    use crate::simulator::dimension::{Mass, Radius};

    #[test]
    fn test_single_particle() {
        let mut w = World::default();
        w.add_body(BodyBuilder::unitary().build());
        let runner = Runner::new(w, 1.0);
        let p = &runner.world.bodies()[0];

        assert_approx_eq(p.speed(), 0.0);
    }

    #[test]
    fn test_two_particles_attract_each_other() {
        let mut builder = BodyBuilder::unitary();
        let mut w = World::default();
        w.add_body(builder.build());
        w.add_body(builder.position(Vec3::new(10.0, 0.0, 0.0)).build());

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
        let mut builder = BodyBuilder::unitary();
        let mut w = World::default();
        w.add_body(builder.build());
        w.add_body(
            builder
                .mass(Mass::kg(1e3).unwrap())
                .radius(Radius::m(10.0).unwrap())
                .build(),
        );

        let mut runner = Runner::new(w, 1e-3);

        let (p1, p2) = (&runner.world.bodies()[0], &runner.world.bodies()[1]);
        assert_approx_eq(distance(p1, p2), 0.0);

        runner.step();

        let (p1, p2) = (&runner.world.bodies()[0], &runner.world.bodies()[1]);
        assert_approx_eq(distance(p1, p2), 0.0);
        assert_approx_eq(p1.speed(), 0.0);
        assert_approx_eq(p2.speed(), 0.0);
    }

    macro_rules! two_body_test_system {
        ($property: ident $(, setup ($p1:ident, $p2:ident) $setup_func: block)?) => {{
            let mut builder = BodyBuilder::new(Mass::kg(1e10).unwrap(), Radius::m(1.0).unwrap());
            let mut p1 = builder.position(Vec3::new(-10.0, 0.0, 0.0)).build();
            let mut p2 = builder.position(Vec3::new(10.0, 0.0, 0.0)).build();

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
        let (initial, final_) = two_body_test_system!(
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
        let (initial_com, final_com) = two_body_test_system!(center_of_mass);

        assert_approx_eq_f64(initial_com.0, final_com.0);
        assert_approx_eq_f64(initial_com.1, final_com.1);
        assert_approx_eq_f64(initial_com.2, final_com.2);
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

        planet.in_circular_orbit(G, &star, orbital_radius as f32);

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
        let relative_energy_error = ((final_energy - initial_energy) / initial_energy).abs();

        assert!(
            relative_energy_error < 0.05,
            "relative energy error: {}",
            relative_energy_error
        );
        assert!(radial_error < 0.05);
    }
}
