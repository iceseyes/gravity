use crate::physics;
use crate::physics::dynamic::momentum;
use rand::random_range;
use std::f32::consts::PI;
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
        const MIN_EXP_MASS: f32 = 1.0;
        const MAX_EXP_MASS: f32 = 19.0;
        const SPACE_RADIUS: f32 = 1e12;
        const DENSITY: f32 = 1.0e3;

        let exponent = random_range(MIN_EXP_MASS..=MAX_EXP_MASS);
        let mass = 10.0_f32.powf(exponent);
        let radius = (3.0 * mass / (4.0 * PI * DENSITY)).cbrt();
        Self {
            x: random_range(-SPACE_RADIUS..SPACE_RADIUS),
            y: random_range(-SPACE_RADIUS..SPACE_RADIUS),
            z: random_range(-SPACE_RADIUS..SPACE_RADIUS),
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

    pub fn velocity(&self) -> (f32, f32, f32) {
        (self.velocity_x, self.velocity_y, self.velocity_z)
    }

    pub fn distance_to(&self, p2: &Body) -> (f32, f32, f32) {
        let dx = self.x - p2.x;
        let dy = self.y - p2.y;
        let dz = self.z - p2.z;

        (dx, dy, dz)
    }

    pub fn accelerate(&mut self, dt: f32, ax: f32, ay: f32, az: f32) {
        self.velocity_x += dt * ax;
        self.velocity_y += dt * ay;
        self.velocity_z += dt * az;
    }

    pub fn update_position(&mut self, dt: f32) {
        self.x += dt * self.velocity_x;
        self.y += dt * self.velocity_y;
        self.z += dt * self.velocity_z;
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

pub fn distance(p1: &Body, p2: &Body) -> f32 {
    let (dx, dy, dz) = p1.distance_to(p2);
    (dx * dx + dy * dy + dz * dz).sqrt()
}

pub fn total_momentum(bodies: &[Body]) -> (f64, f64, f64) {
    bodies.iter().fold((0.0, 0.0, 0.0), |(px, py, pz), body| {
        let (vx, vy, vz) = body.velocity();
        let (mx, my, mz) = momentum(body.mass(), vx, vy, vz);

        (px + mx, py + my, pz + mz)
    })
}

pub fn center_of_mass(bodies: &[Body]) -> (f64, f64, f64) {
    let (m, bx, by, bz) = bodies
        .iter()
        .fold((0.0, 0.0, 0.0, 0.0), |(m, px, py, pz), body| {
            let (bx, by, bz) = physics::dynamic::center_of_mass(body.mass, body.x, body.y, body.z);

            (m + body.mass() as f64, px + bx, py + by, pz + bz)
        });

    (bx / m, by / m, bz / m)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::physics::assert_approx_eq;

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

        p.accelerate(1.0, 1.0, 2.0, 3.0);

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

        p.accelerate(0.5, 2.0, 4.0, 6.0);

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

        p.accelerate(1.0, 1.0, 0.0, 0.0);

        // Dopo il primo step:
        // position = 0
        // velocity = 1
        assert_approx_eq(p.x, 0.0);
        assert_approx_eq(p.velocity_x, 1.0);

        p.update_position(1.0);
        p.accelerate(1.0, 1.0, 0.0, 0.0);

        // Dopo il secondo step:
        // position = 0 + 1 * 1 = 1
        // velocity = 1 + 1 * 1 = 2
        assert_approx_eq(p.x, 1.0);
        assert_approx_eq(p.velocity_x, 2.0);

        p.update_position(1.0);
        p.accelerate(1.0, 1.0, 0.0, 0.0);

        // Dopo il terzo step:
        // position = 1 + 2 * 1 = 3
        // velocity = 2 + 1 * 1 = 3
        assert_approx_eq(p.x, 3.0);
        assert_approx_eq(p.velocity_x, 3.0);
    }

    #[test]
    fn test_zero_acceleration() {
        let dt: f32 = 0.5;
        let mut p = Body::new(1.0, 1.0);

        // Impostiamo manualmente una velocità iniziale.
        p.velocity_x = 10.0;
        p.velocity_y = 5.0;
        p.velocity_z = -2.0;

        p.accelerate(dt, 0.0, 0.0, 0.0);

        // La velocità non cambia.
        assert_approx_eq(p.velocity_x, 10.0);
        assert_approx_eq(p.velocity_y, 5.0);
        assert_approx_eq(p.velocity_z, -2.0);

        // La posizione non cambia...
        assert_approx_eq(p.x, 0.0);
        assert_approx_eq(p.y, 0.0);
        assert_approx_eq(p.z, 0.0);

        // fino a che non l'aggiorno (applico la velocità)
        p.update_position(dt);
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
            p.accelerate(dt, acceleration, 0.0, 0.0);
        }

        // v = a * t = 10 * 1 = 10
        assert_approx_eq(p.velocity_x, 10.0);
    }

    #[test]
    fn test_acceleration_doesnt_change_position() {
        let mut p = Body::new(1.0, 1.0);
        let dt = 1.0;
        let acceleration = 10.0;

        // applicare solo un'accelerazione
        p.accelerate(dt, acceleration, 0.0, 0.0);
        // non cambia la posizione
        assert_approx_eq(p.x, 0.0);
        assert_approx_eq(p.y, 0.0);
        assert_approx_eq(p.z, 0.0);

        // ma aggiorna le velocità
        assert_approx_eq(p.velocity_x, 10.0);
        assert_approx_eq(p.velocity_y, 0.0);
        assert_approx_eq(p.velocity_z, 0.0);

        // se non aggiorno esplicitamente la posizione, non cambia
        p.accelerate(dt, acceleration, 0.0, 0.0);
        assert_approx_eq(p.x, 0.0);
    }

    #[test]
    fn test_update_position() {
        let mut p = Body::new(1.0, 1.0);

        // aggiornare la posizione con velocità 0
        p.update_position(0.5);

        // non cambia la posizione
        assert_approx_eq(p.x, 0.0);
        assert_approx_eq(p.y, 0.0);
        assert_approx_eq(p.z, 0.0);

        p.update_position(0.5);
        assert_approx_eq(p.x, 0.0);
        assert_approx_eq(p.y, 0.0);
        assert_approx_eq(p.z, 0.0);

        // se impongo una velocità, la posizione cambia
        p.set_velocity(10.0, 0.0, 0.0);
        p.update_position(0.5);
        assert_approx_eq(p.x, 5.0);
        assert_approx_eq(p.y, 0.0);
        assert_approx_eq(p.z, 0.0);

        // in base al vettore velocità configurato
        p.set_velocity(0.0, 10.0, 0.0);
        p.update_position(0.5);
        assert_approx_eq(p.x, 5.0);
        assert_approx_eq(p.y, 5.0);
        assert_approx_eq(p.z, 0.0);

        // ...e anche per l'asse z
        p.set_velocity(0.0, 0.0, 10.0);
        p.update_position(0.5);
        assert_approx_eq(p.x, 5.0);
        assert_approx_eq(p.y, 5.0);
        assert_approx_eq(p.z, 5.0);
    }

    #[test]
    fn test_constant_acceleration() {
        let mut p = Body::new(1.0, 1.0);

        let dt = 0.1;
        let acceleration = 10.0;

        for _ in 0..10 {
            p.update_position(dt);
            p.accelerate(dt, acceleration, 0.0, 0.0);
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
