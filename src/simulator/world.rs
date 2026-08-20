use crate::physics;
use crate::physics::EPSILON;
use crate::simulator::body::Body;

pub const DEFAULT_DTIME: f32 = 1e-3; // seconds

#[derive(Debug)]
pub struct World {
    gravity_constant: f32,
    dtime: f32,
    bodies: Vec<Body>,
    min_x: f32,
    max_x: f32,
    min_y: f32,
    max_y: f32,
}

impl World {
    pub fn new(gravity_constant: f32, dtime: f32) -> Self {
        let min_x = f32::INFINITY;
        let max_x = f32::NEG_INFINITY;
        let min_y = f32::INFINITY;
        let max_y = f32::NEG_INFINITY;

        Self {
            gravity_constant,
            dtime,
            bodies: Vec::new(),
            min_x,
            max_x,
            min_y,
            max_y,
        }
    }

    pub fn add_body(&mut self, p: Body) {
        update_viewport(
            &p,
            &mut self.min_x,
            &mut self.max_x,
            &mut self.min_y,
            &mut self.max_y,
        );

        self.bodies.push(p);
    }

    pub fn origin(&self) -> (f32, f32) {
        if self.bodies.is_empty() {
            (0.0, 0.0)
        } else {
            (self.min_x, self.min_y)
        }
    }

    pub fn width(&self) -> f32 {
        let w = self.max_x - self.min_x;

        if w.abs() > 1.0 { w } else { 1.0 }
    }

    pub fn height(&self) -> f32 {
        let h = self.max_y - self.min_y;

        if h.abs() > 1.0 { h } else { 1.0 }
    }

    pub fn bodies(&self) -> &[Body] {
        &self.bodies
    }

    pub fn reset_viewport(&mut self) {
        self.min_x = f32::INFINITY;
        self.max_x = f32::NEG_INFINITY;
        self.min_y = f32::INFINITY;
        self.max_y = f32::NEG_INFINITY;

        self.bodies.iter().for_each(|p| {
            update_viewport(
                p,
                &mut self.min_x,
                &mut self.max_x,
                &mut self.min_y,
                &mut self.max_y,
            );
        });
    }

    pub fn update(&mut self) {
        self.bodies = self
            .bodies
            .iter()
            .enumerate()
            .map(|(i, p1)| {
                let (mut ax, mut ay, mut az) = (0.0, 0.0, 0.0);

                self.bodies().iter().enumerate().for_each(|(index2, p2)| {
                    if i != index2 {
                        let (dx, dy, dz) = p2.distance_to(p1);
                        let distance_squared = dx * dx + dy * dy + dz * dz;

                        if distance_squared > EPSILON {
                            let distance = distance_squared.sqrt();
                            let factor = p2.mass() / (distance_squared * distance);

                            ax += dx * factor;
                            ay += dy * factor;
                            az += dz * factor;
                        }
                    }
                });

                let mut p = p1.clone();
                p.update_position(self.dtime);
                p.accelerate(
                    self.dtime,
                    self.gravity_constant * ax,
                    self.gravity_constant * ay,
                    self.gravity_constant * az,
                );

                p
            })
            .collect();
    }
}

impl Default for World {
    fn default() -> Self {
        Self::new(physics::G, DEFAULT_DTIME)
    }
}

fn update_viewport(p: &Body, min_x: &mut f32, max_x: &mut f32, min_y: &mut f32, max_y: &mut f32) {
    let (x, y, _) = p.position();

    *min_x = min_x.min(x - p.radius());
    *max_x = max_x.max(x + p.radius());
    *min_y = min_y.min(y - p.radius());
    *max_y = max_y.max(y + p.radius());
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::physics::assert_approx_eq;
    use crate::simulator::body::distance;

    #[test]
    fn test_single_particle() {
        let mut w = World::default();
        w.add_body(Body::new(1.0, 1.0));
        w.update();

        let p = &w.bodies()[0];

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

        w.update();

        let particles = w.bodies();

        assert!(particles[0].speed() > 0.0);
        assert!(particles[1].speed() > 0.0);

        assert_approx_eq(particles[0].velocity_direction().0, 1.0);
        assert_approx_eq(particles[0].velocity_direction().1, 0.0);
        assert_approx_eq(particles[0].velocity_direction().2, 0.0);

        assert_approx_eq(particles[1].velocity_direction().0, -1.0);
        assert_approx_eq(particles[1].velocity_direction().1, 0.0);
        assert_approx_eq(particles[1].velocity_direction().2, 0.0);
    }

    #[test]
    fn test_singularity() {
        let mut w = World::default();
        w.add_body(Body::new(1.0, 1.0));
        w.add_body(Body::new(1000.0, 10.0));

        let (p1, p2) = (&w.bodies()[0], &w.bodies()[1]);
        assert_approx_eq(distance(p1, p2), 0.0);

        w.update();
        let (p1, p2) = (&w.bodies()[0], &w.bodies()[1]);
        assert_approx_eq(distance(p1, p2), 0.0);
        assert_approx_eq(p1.speed(), 0.0);
        assert_approx_eq(p2.speed(), 0.0);
    }
}
