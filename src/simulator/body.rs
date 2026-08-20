use rand::random_range;
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone)]
pub struct Body {
    x: f32,
    y: f32,
    z: f32,
    velocity_x: f32,
    velocity_y: f32,
    velocity_z: f32,
    mass: f32,
    radius: f32,
}

impl Body {
    pub fn new(mass: f32, radius: f32) -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            z: 0.0,
            velocity_x: 0.0,
            velocity_y: 0.0,
            velocity_z: 0.0,
            mass,
            radius,
        }
    }

    pub fn random() -> Self {
        let mass = random_range(0.001..1e12);
        let density = random_range(mass / 10.0..10.0 * mass);
        let radius = mass / density;
        Self {
            x: random_range(0.0..100.0),
            y: random_range(0.0..100.0),
            z: random_range(0.0..100.0),
            velocity_x: 0.0,
            velocity_y: 0.0,
            velocity_z: 0.0,
            mass,
            radius,
        }
    }

    pub fn position(&self) -> (f32, f32, f32) {
        (self.x, self.y, self.z)
    }

    pub fn radius(&self) -> f32 {
        self.radius
    }

    pub fn speed(&self) -> f32 {
        (self.velocity_x.powi(2) + self.velocity_y.powi(2) + self.velocity_z.powi(2)).sqrt()
    }

    pub fn velocity_direction(&self) -> (f32, f32, f32) {
        let speed = self.speed();
        if speed.abs() < 1e-20 {
            (0.0, 0.0, 0.0)
        } else {
            (
                self.velocity_x / speed,
                self.velocity_y / speed,
                self.velocity_z / speed,
            )
        }
    }

    pub fn mass(&self) -> f32 {
        self.mass
    }

    pub fn move_to(&mut self, x: f32, y: f32, z: f32) {
        self.x = x;
        self.y = y;
        self.z = z;
    }

    pub fn set_velocity(&mut self, vx: f32, vy: f32, vz: f32) {
        self.velocity_x = vx;
        self.velocity_y = vy;
        self.velocity_z = vz;
    }

    pub fn update(&mut self, dt: f32, ax: f32, ay: f32, az: f32) {
        self.x += dt * self.velocity_x;
        self.y += dt * self.velocity_y;
        self.z += dt * self.velocity_z;
        self.velocity_x += dt * ax;
        self.velocity_y += dt * ay;
        self.velocity_z += dt * az;
    }
}

impl Display for Body {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "[({}, {}, {}); r = {}m; m = {}kg; v = {}",
            self.x,
            self.y,
            self.z,
            self.radius,
            self.mass,
            self.speed()
        ))
    }
}

pub fn vector_distance(p1: &Body, p2: &Body) -> (f32, f32, f32) {
    let dx = p1.x - p2.x;
    let dy = p1.y - p2.y;
    let dz = p1.z - p2.z;

    (dx, dy, dz)
}

pub fn distance(p1: &Body, p2: &Body) -> f32 {
    let (dx, dy, dz) = vector_distance(p1, p2);
    (dx * dx + dy * dy + dz * dz).sqrt()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::physics::{EPSILON, assert_approx_eq};

    #[test]
    fn test_distance() {
        let mut p1 = Body::new(1.0, 1.0);
        let mut p2 = Body::new(1.0, 1.0);
        assert!(distance(&p1, &p2).abs() < 0.0001);

        p1.move_to(10.0, 10.0, 10.0);
        assert!((distance(&p1, &p2) - 17.320509).abs() < 0.0001);

        p2.move_to(20.0, 20.0, 20.0);
        assert!((distance(&p1, &p2) - 17.320509).abs() < 0.0001);

        p1.move_to(10.0, 20.0, 20.0);
        assert!((distance(&p1, &p2) - 10.0).abs() < 0.0001);
    }

    #[test]
    fn test_acceleration_with_dt_one() {
        let mut p = Body::new(1.0, 1.0);

        p.update(1.0, 1.0, 2.0, 3.0);

        // La posizione usa la velocità precedente, che era 0.
        assert_approx_eq(p.x, 0.0);
        assert_approx_eq(p.y, 0.0);
        assert_approx_eq(p.z, 0.0);

        // v = v + a * dt
        assert_approx_eq(p.velocity_x, 1.0);
        assert_approx_eq(p.velocity_y, 2.0);
        assert_approx_eq(p.velocity_z, 3.0);
    }

    #[test]
    fn test_acceleration_with_fractional_dt() {
        let mut p = Body::new(1.0, 1.0);

        p.update(0.5, 2.0, 4.0, 6.0);

        // v = 0 + a * 0.5
        assert_approx_eq(p.velocity_x, 1.0);
        assert_approx_eq(p.velocity_y, 2.0);
        assert_approx_eq(p.velocity_z, 3.0);

        // La posizione rimane invariata al primo step.
        assert_approx_eq(p.x, 0.0);
        assert_approx_eq(p.y, 0.0);
        assert_approx_eq(p.z, 0.0);
    }

    #[test]
    fn test_multiple_updates() {
        let mut p = Body::new(1.0, 1.0);

        p.update(1.0, 1.0, 0.0, 0.0);

        // Dopo il primo step:
        // position = 0
        // velocity = 1
        assert_approx_eq(p.x, 0.0);
        assert_approx_eq(p.velocity_x, 1.0);

        p.update(1.0, 1.0, 0.0, 0.0);

        // Dopo il secondo step:
        // position = 0 + 1 * 1 = 1
        // velocity = 1 + 1 * 1 = 2
        assert_approx_eq(p.x, 1.0);
        assert_approx_eq(p.velocity_x, 2.0);

        p.update(1.0, 1.0, 0.0, 0.0);

        // Dopo il terzo step:
        // position = 1 + 2 * 1 = 3
        // velocity = 2 + 1 * 1 = 3
        assert_approx_eq(p.x, 3.0);
        assert_approx_eq(p.velocity_x, 3.0);
    }

    #[test]
    fn test_zero_acceleration() {
        let mut p = Body::new(1.0, 1.0);

        // Impostiamo manualmente una velocità iniziale.
        p.velocity_x = 10.0;
        p.velocity_y = 5.0;
        p.velocity_z = -2.0;

        p.update(0.5, 0.0, 0.0, 0.0);

        // La velocità non cambia.
        assert_approx_eq(p.velocity_x, 10.0);
        assert_approx_eq(p.velocity_y, 5.0);
        assert_approx_eq(p.velocity_z, -2.0);

        // La posizione continua a muoversi.
        assert_approx_eq(p.x, 5.0);
        assert_approx_eq(p.y, 2.5);
        assert_approx_eq(p.z, -1.0);
    }

    #[test]
    fn test_acceleration_changes_velocity_linearly() {
        let mut p = Body::new(1.0, 1.0);

        let dt = 0.1;
        let acceleration = 10.0;

        for _ in 0..10 {
            p.update(dt, acceleration, 0.0, 0.0);
        }

        // v = a * t = 10 * 1 = 10
        assert_approx_eq(p.velocity_x, 10.0);
    }

    #[test]
    fn test_constant_acceleration() {
        let mut p = Body::new(1.0, 1.0);

        let dt = 0.1;
        let acceleration = 10.0;

        for _ in 0..10 {
            p.update(dt, acceleration, 0.0, 0.0);
        }

        assert_approx_eq(p.velocity_x, 10.0);
        assert_approx_eq(p.x, 4.5);
    }

    #[test]
    fn test_direction() {
        let mut p = Body::new(1.0, 1.0);

        p.velocity_x = 3.0;
        p.velocity_y = 4.0;
        p.velocity_z = 0.0;

        let (x, y, z) = p.velocity_direction();

        assert_approx_eq(x, 0.6);
        assert_approx_eq(y, 0.8);
        assert_approx_eq(z, 0.0);
    }

    #[test]
    fn test_direction_when_stationary() {
        let p = Body::new(1.0, 1.0);

        let (x, y, z) = p.velocity_direction();

        assert_approx_eq(x, 0.0);
        assert_approx_eq(y, 0.0);
        assert_approx_eq(z, 0.0);
    }
}
