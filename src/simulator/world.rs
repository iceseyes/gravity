use crate::physics;
use crate::physics::{Vec3, gravity};
use crate::simulator::body::{
    Body, center_of_mass, kinetic_energy, potential_energy, total_angular_momentum, total_momentum,
};

#[derive(Debug, Clone)]
pub struct World {
    gravity_constant: f64,
    bodies: Vec<Body>,
    min_x: f64,
    max_x: f64,
    min_y: f64,
    max_y: f64,
}

impl World {
    pub fn new(gravity_constant: f64) -> Self {
        let min_x = f64::INFINITY;
        let max_x = f64::NEG_INFINITY;
        let min_y = f64::INFINITY;
        let max_y = f64::NEG_INFINITY;

        Self {
            gravity_constant,
            bodies: Vec::new(),
            min_x,
            max_x,
            min_y,
            max_y,
        }
    }

    pub fn copy_from(&mut self, other: &Self) {
        self.gravity_constant = other.gravity_constant;
        self.bodies = other.bodies.clone();
        self.min_x = other.min_x;
        self.max_x = other.max_x;
        self.min_y = other.min_y;
        self.max_y = other.max_y;
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

    pub fn origin(&self) -> (f64, f64) {
        if self.bodies.is_empty() {
            (0.0, 0.0)
        } else {
            (self.min_x, self.min_y)
        }
    }

    pub fn width(&self) -> f64 {
        if self.bodies.is_empty() {
            0.0
        } else {
            self.max_x - self.min_x
        }
    }

    pub fn height(&self) -> f64 {
        if self.bodies.is_empty() {
            0.0
        } else {
            self.max_y - self.min_y
        }
    }

    pub fn gravity_constant(&self) -> f64 {
        self.gravity_constant
    }

    pub fn bodies(&self) -> &[Body] {
        &self.bodies
    }

    pub fn mut_bodies(&mut self) -> &mut [Body] {
        &mut self.bodies
    }

    pub fn accelerations(&self) -> Vec<Vec3> {
        self.bodies
            .iter()
            .enumerate()
            .map(|(index, p1)| {
                let a = self
                    .bodies
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
                self.gravity_constant * a
            })
            .collect()
    }
    pub fn total_momentum(&self) -> Vec3 {
        total_momentum(&self.bodies)
    }

    pub fn total_angular_momentum(&self) -> Vec3 {
        total_angular_momentum(&self.bodies)
    }

    pub fn center_of_mass(&self) -> Vec3 {
        center_of_mass(&self.bodies)
    }

    pub fn kinetic_energy(&self) -> f64 {
        kinetic_energy(self.bodies())
    }

    pub fn potential_energy(&self) -> f64 {
        potential_energy(self.gravity_constant, &self.bodies)
    }

    pub fn energy(&self) -> f64 {
        self.kinetic_energy() + self.potential_energy()
    }

    pub fn compute_world_size(&mut self) {
        self.min_x = f64::INFINITY;
        self.max_x = f64::NEG_INFINITY;
        self.min_y = f64::INFINITY;
        self.max_y = f64::NEG_INFINITY;

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
}

impl Default for World {
    fn default() -> Self {
        Self::new(physics::G)
    }
}

fn update_viewport(
    body: &Body,
    min_x: &mut f64,
    max_x: &mut f64,
    min_y: &mut f64,
    max_y: &mut f64,
) {
    let p = body.position();
    let r = body.radius();

    *min_x = min_x.min(p.x - r);
    *max_x = max_x.max(p.x + r);
    *min_y = min_y.min(p.y - r);
    *max_y = max_y.max(p.y + r);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::physics::Vec3;
    use crate::simulator::body::BodyBuilder;
    use crate::simulator::dimension::Radius;
    use approx::assert_abs_diff_eq;

    #[test]
    fn test_world_size_without_bodies() {
        let w = World::new(physics::G);
        let (x, y) = w.origin();

        assert_eq!(x, 0.0);
        assert_eq!(y, 0.0);
        assert_eq!(w.width(), 0.0);
        assert_eq!(w.height(), 0.0);
    }

    #[test]
    fn test_world_size_upon_adding_bodies() {
        let mut w = World::new(physics::G);
        w.add_body(BodyBuilder::unitary().build());

        // il corpo si trova nel punto (0, 0) con raggio 1.0
        let (x, y) = w.origin();

        assert_abs_diff_eq!(x, -1.0);
        assert_abs_diff_eq!(y, -1.0);
        assert_abs_diff_eq!(w.width(), 2.0);
        assert_abs_diff_eq!(w.height(), 2.0);

        w.add_body(
            BodyBuilder::unitary()
                .radius(Radius::m(2.0).unwrap())
                .position(Vec3::new(10.0, 10.0, 0.0))
                .build(),
        );

        // il secondo corpo si trova nel punto (10, 10) con raggio 2.0
        let (x, y) = w.origin();

        // l'origin non cambia, ma la larghezza e l'altezza aumentano perché
        // variano da -1 (0 - 1) a 12 (10 + 2)
        assert_abs_diff_eq!(x, -1.0);
        assert_abs_diff_eq!(y, -1.0);
        assert_abs_diff_eq!(w.width(), 13.0);
        assert_abs_diff_eq!(w.height(), 13.0);
    }
}
