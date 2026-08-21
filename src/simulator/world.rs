use crate::physics;
use crate::simulator::body::{Body, center_of_mass, total_momentum};

pub const DEFAULT_DTIME: f32 = 1e-3; // seconds

#[derive(Debug, Clone)]
pub struct World {
    gravity_constant: f32,
    bodies: Vec<Body>,
    min_x: f32,
    max_x: f32,
    min_y: f32,
    max_y: f32,
}

impl World {
    pub fn new(gravity_constant: f32) -> Self {
        let min_x = f32::INFINITY;
        let max_x = f32::NEG_INFINITY;
        let min_y = f32::INFINITY;
        let max_y = f32::NEG_INFINITY;

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

    pub fn origin(&self) -> (f32, f32) {
        if self.bodies.is_empty() {
            (0.0, 0.0)
        } else {
            (self.min_x, self.min_y)
        }
    }

    pub fn width(&self) -> f32 {
        if self.bodies.is_empty() {
            0.0
        } else {
            let w = self.max_x - self.min_x;
            if w.abs() > 1.0 { w } else { 1.0 }
        }
    }

    pub fn height(&self) -> f32 {
        if self.bodies.is_empty() {
            0.0
        } else {
            let h = self.max_y - self.min_y;

            if h.abs() > 1.0 { h } else { 1.0 }
        }
    }

    pub fn gravity_constant(&self) -> f32 {
        self.gravity_constant
    }

    pub fn bodies(&self) -> &[Body] {
        &self.bodies
    }

    pub fn mut_bodies(&mut self) -> &mut [Body] {
        &mut self.bodies
    }

    pub fn total_momentum(&self) -> (f64, f64, f64) {
        total_momentum(&self.bodies)
    }

    pub fn center_of_mass(&self) -> (f64, f64, f64) {
        center_of_mass(&self.bodies)
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
}

impl Default for World {
    fn default() -> Self {
        Self::new(physics::G)
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
        w.add_body(Body::new(1.0, 1.0));

        // il corpo si trova nel punto (0, 0) con raggio 1.0
        let (x, y) = w.origin();

        assert_approx_eq(x, -1.0);
        assert_approx_eq(y, -1.0);
        assert_approx_eq(w.width(), 2.0);
        assert_approx_eq(w.height(), 2.0);

        let mut b1 = Body::new(1.0, 2.0);
        b1.move_to(10.0, 10.0, 10.0);
        w.add_body(b1);

        // il secondo corpo si trova nel punto (10, 10) con raggio 2.0
        let (x, y) = w.origin();

        // l'origin non cambia, ma la larghezza e l'altezza aumentano perché
        // variano da -1 (0 - 1) a 12 (10 + 2)
        assert_approx_eq(x, -1.0);
        assert_approx_eq(y, -1.0);
        assert_approx_eq(w.width(), 13.0);
        assert_approx_eq(w.height(), 13.0);
    }
}
