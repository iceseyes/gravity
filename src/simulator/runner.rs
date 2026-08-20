use crate::physics::gravity;
use crate::simulator::World;

#[derive(Debug)]
pub struct Runner {
    world: World,
    time: f64,
    dt: f32,
    running: bool,
}

impl Runner {
    pub fn new(world: World, dt: f32) -> Self {
        Self {
            world,
            dt,
            time: 0.0,
            running: false,
        }
    }

    pub fn time(&self) -> f64 {
        self.time
    }

    pub fn world(&self) -> World {
        self.world.clone()
    }

    pub fn is_running(&self) -> bool {
        self.running
    }

    pub fn stop(&mut self) {
        self.running = false;
    }

    pub fn start(&mut self) {
        self.running = true;
    }

    pub fn run_once(&mut self) {
        if self.running {
            self.step();
        }
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
    use crate::physics::assert_approx_eq;
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
}
