use crate::physics::geometry::Sphere;
use crate::physics::{Vec3, dynamic, gravity};
use crate::simulator::dimension::{Mass, Radius};
use rand::random_range;
use std::f64;
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone)]
pub struct Body {
    position: Vec3,
    velocity: Vec3,
    mass: f64,
    radius: f64,
}

impl Body {
    fn new(mass: f64, radius: f64) -> Self {
        Self {
            position: Vec3::zeros(),
            velocity: Vec3::zeros(),
            mass,
            radius,
        }
    }

    pub fn random() -> Self {
        const MIN_EXP_MASS: f64 = 1.0;
        const MAX_EXP_MASS: f64 = 19.0;
        const SPACE_RADIUS: f64 = 1e12;
        const DENSITY: f64 = 1.0e3;

        let exponent = random_range(MIN_EXP_MASS..=MAX_EXP_MASS);
        let mass = Mass::kg(10_f64.powf(exponent)).unwrap();
        let radius = Radius::by_density(DENSITY).unwrap().get(mass);
        let position = Vec3::new(
            random_range(-SPACE_RADIUS..SPACE_RADIUS),
            random_range(-SPACE_RADIUS..SPACE_RADIUS),
            random_range(-SPACE_RADIUS..SPACE_RADIUS),
        );
        Self {
            position,
            velocity: Vec3::zeros(),
            mass: mass.get(),
            radius,
        }
    }

    pub fn density(&self) -> f64 {
        let volume = Sphere::by_radius(self.radius).unwrap().volume();
        self.mass / volume
    }

    pub fn position(&self) -> Vec3 {
        self.position
    }

    pub fn radius(&self) -> f64 {
        self.radius
    }

    pub fn speed(&self) -> f64 {
        self.velocity.norm()
    }

    pub fn velocity_direction(&self) -> Vec3 {
        let speed = self.speed();
        if speed.abs() < 1e-20 {
            Vec3::zeros()
        } else {
            self.velocity / speed
        }
    }

    pub fn mass(&self) -> f64 {
        self.mass
    }

    pub fn momentum(&self) -> Vec3 {
        dynamic::momentum(self.mass, self.velocity)
    }

    pub fn angular_momentum(&self) -> Vec3 {
        self.position().cross(&self.momentum())
    }

    pub fn move_to(&mut self, p: Vec3) {
        self.position = p;
    }

    pub fn set_velocity(&mut self, v: Vec3) {
        self.velocity = v;
    }

    pub fn velocity(&self) -> Vec3 {
        self.velocity
    }

    pub fn kinetic_energy(&self) -> f64 {
        dynamic::kinetic_energy(self.mass, self.velocity)
    }

    pub fn distance_to(&self, p2: &Body) -> Vec3 {
        self.position - p2.position
    }

    pub fn accelerate(&mut self, dt: f64, a: Vec3) {
        self.velocity += dt * a;
    }

    pub fn update_position(&mut self, dt: f64) {
        self.position += dt * self.velocity;
    }

    pub fn in_circular_orbit(&mut self, gravity_constant: f64, star: &Body, distance: f64) {
        let v_squared = gravity::orbital_squared_velocity(star.mass(), distance);
        let orbital_v = (gravity_constant * v_squared).sqrt();
        self.move_to(star.position + Vec3::new(distance, 0.0, 0.0));
        self.set_velocity(star.velocity + Vec3::new(0.0, orbital_v, 0.0));
    }
}

impl Display for Body {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "[{}; r = {}m; m = {}kg; v = {}",
            self.position,
            self.radius,
            self.mass,
            self.speed()
        ))
    }
}

#[derive(Debug)]
pub struct BodyBuilder {
    mass: Mass,
    radius: Radius,
    position: Vec3,
    velocity: Vec3,
}

impl BodyBuilder {
    pub fn new(mass: Mass, radius: Radius) -> Self {
        Self {
            mass,
            radius,
            position: Vec3::default(),
            velocity: Vec3::default(),
        }
    }

    pub fn unitary() -> Self {
        Self {
            mass: Mass::kg(1.0).unwrap(),
            radius: Radius::m(1.0).unwrap(),
            position: Vec3::default(),
            velocity: Vec3::default(),
        }
    }

    pub fn build(self) -> Body {
        let mut body = Body::new(self.mass.get(), self.radius.get(self.mass));
        body.move_to(self.position);
        body.set_velocity(self.velocity);

        body
    }

    pub fn mass(mut self, mass: Mass) -> Self {
        self.mass = mass;
        self
    }

    pub fn radius(mut self, radius: Radius) -> Self {
        self.radius = radius;
        self
    }

    pub fn position(mut self, position: Vec3) -> Self {
        self.position = position;
        self
    }

    pub fn velocity(mut self, velocity: Vec3) -> Self {
        self.velocity = velocity;
        self
    }
}

pub fn distance(p1: &Body, p2: &Body) -> f64 {
    p1.distance_to(p2).norm()
}

pub fn total_momentum(bodies: &[Body]) -> Vec3 {
    bodies
        .iter()
        .fold(Vec3::zeros(), |q, body| q + body.momentum())
}

pub fn total_angular_momentum(bodies: &[Body]) -> Vec3 {
    let c = center_of_mass(bodies);
    bodies.iter().fold(Vec3::zeros(), |q, body| {
        q + (body.position - c).cross(&body.momentum())
    })
}

pub fn center_of_mass(bodies: &[Body]) -> Vec3 {
    if bodies.is_empty() {
        return Vec3::zeros();
    }

    let (m, c) = bodies.iter().fold((0.0, Vec3::zeros()), |(m, c), body| {
        (m + body.mass(), c + body.mass() * body.position())
    });

    c / m
}

pub fn kinetic_energy(bodies: &[Body]) -> f64 {
    bodies.iter().fold(0.0, |k, body| k + body.kinetic_energy())
}

pub fn potential_energy(gravity_constant: f64, bodies: &[Body]) -> f64 {
    gravity_constant
        * bodies.iter().enumerate().fold(0.0, |k, (index, body)| {
            bodies.iter().skip(index + 1).fold(k, |k, b2| {
                k + gravity::potential_energy(body.mass, b2.mass, body.distance_to(b2))
            })
        })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::physics::{G, Vec3};
    use approx::assert_abs_diff_eq;

    #[test]
    fn test_builder_builds_always_new_bodies() {
        let ones = Vec3::new(1.0, 1.0, 1.0);
        let mut b1 = BodyBuilder::unitary().build();
        let mut b2 = BodyBuilder::unitary().build();

        assert_eq!(b1.mass(), b2.mass());
        assert_eq!(b1.radius(), b2.radius());
        assert_abs_diff_eq!(distance(&b1, &b2), 0.0);

        b1.move_to(ones);
        assert_abs_diff_eq!(distance(&b1, &b2), 1.7320508, epsilon = 1e-7);

        b2.move_to(ones);
        assert_abs_diff_eq!(distance(&b1, &b2), 0.0);

        b1.move_to(Vec3::zeros());
        assert_abs_diff_eq!(distance(&b1, &b2), 1.7320508, epsilon = 1e-7);
    }

    #[test]
    fn test_distance() {
        let mut b1 = BodyBuilder::unitary().build();
        let mut b2 = BodyBuilder::unitary().build();
        assert!(distance(&b1, &b2).abs() < 0.0001);

        b1.move_to(Vec3::new(10.0, 10.0, 10.0));
        assert!((distance(&b1, &b2) - 17.320509).abs() < 0.0001);

        b2.move_to(Vec3::new(20.0, 20.0, 20.0));
        assert!((distance(&b1, &b2) - 17.320509).abs() < 0.0001);

        b1.move_to(Vec3::new(10.0, 20.0, 20.0));
        assert!((distance(&b1, &b2) - 10.0).abs() < 0.0001);
    }

    #[test]
    fn test_acceleration_with_dt_one() {
        let mut p = Body::new(1.0, 1.0);

        p.accelerate(1.0, Vec3::new(1.0, 2.0, 3.0));

        // La posizione usa la velocità precedente, che era 0.
        assert_eq!(p.position, Vec3::zeros());

        // v = v + a * dt
        assert_eq!(p.velocity, Vec3::new(1.0, 2.0, 3.0));
    }

    #[test]
    fn test_acceleration_with_fractional_dt() {
        let mut p = Body::new(1.0, 1.0);

        p.accelerate(0.5, Vec3::new(2.0, 4.0, 6.0));

        // v = 0 + a * 0.5
        assert_eq!(p.velocity, Vec3::new(1.0, 2.0, 3.0));

        // La posizione rimane invariata al primo step.
        assert_eq!(p.position, Vec3::zeros());
    }

    #[test]
    fn test_multiple_updates() {
        let mut p = Body::new(1.0, 1.0);

        p.accelerate(1.0, Vec3::new(1.0, 0.0, 0.0));

        // Dopo il primo step:
        // position = 0
        // velocity = 1
        assert_abs_diff_eq!(p.position, Vec3::zeros());
        assert_abs_diff_eq!(p.velocity, Vec3::new(1.0, 0.0, 0.0));

        p.update_position(1.0);
        p.accelerate(1.0, Vec3::new(1.0, 0.0, 0.0));

        // Dopo il secondo step:
        // position = 0 + 1 * 1 = 1
        // velocity = 1 + 1 * 1 = 2
        assert_abs_diff_eq!(p.position, Vec3::new(1.0, 0.0, 0.0));
        assert_abs_diff_eq!(p.velocity, Vec3::new(2.0, 0.0, 0.0));

        p.update_position(1.0);
        p.accelerate(1.0, Vec3::new(1.0, 0.0, 0.0));

        // Dopo il terzo step:
        // position = 1 + 2 * 1 = 3
        // velocity = 2 + 1 * 1 = 3
        assert_abs_diff_eq!(p.position, Vec3::new(3.0, 0.0, 0.0));
        assert_abs_diff_eq!(p.velocity, Vec3::new(3.0, 0.0, 0.0));
    }

    #[test]
    fn test_zero_acceleration() {
        let dt: f64 = 0.5;
        let mut p = Body::new(1.0, 1.0);

        // Impostiamo manualmente una velocità iniziale.
        p.velocity = Vec3::new(10.0, 5.0, -2.0);

        p.accelerate(dt, Vec3::zeros());

        // La velocità non cambia.
        assert_abs_diff_eq!(p.velocity, Vec3::new(10.0, 5.0, -2.0));

        // La posizione non cambia...
        assert_abs_diff_eq!(p.position, Vec3::zeros());

        // fino a che non l'aggiorno (applico la velocità)
        p.update_position(dt);
        assert_abs_diff_eq!(p.position, Vec3::new(5.0, 2.5, -1.0));
    }

    #[test]
    fn test_acceleration_changes_velocity_linearly() {
        let mut p = Body::new(1.0, 1.0);

        let dt = 0.1;
        let acceleration = Vec3::new(10.0, 0.0, 0.0);

        for _ in 0..10 {
            p.accelerate(dt, acceleration);
        }

        // v = a * t = 10 * 1 = 10
        assert_abs_diff_eq!(p.velocity, Vec3::new(10.0, 0.0, 0.0));
    }

    #[test]
    fn test_acceleration_doesnt_change_position() {
        let mut p = Body::new(1.0, 1.0);
        let dt = 1.0;
        let acceleration = Vec3::new(10.0, 0.0, 0.0);

        // applicare solo un'accelerazione
        p.accelerate(dt, acceleration);
        // non cambia la posizione
        assert_abs_diff_eq!(p.position, Vec3::zeros());

        // ma aggiorna le velocità
        assert_abs_diff_eq!(p.velocity, Vec3::new(10.0, 0.0, 0.0));

        // se non aggiorno esplicitamente la posizione, non cambia
        p.accelerate(dt, acceleration);
        assert_abs_diff_eq!(p.position, Vec3::zeros());
    }

    #[test]
    fn test_update_position() {
        let mut p = Body::new(1.0, 1.0);

        // aggiornare la posizione con velocità 0
        p.update_position(0.5);

        // non cambia la posizione
        assert_abs_diff_eq!(p.position, Vec3::zeros());

        p.update_position(0.5);
        assert_abs_diff_eq!(p.position, Vec3::zeros());

        // se impongo una velocità, la posizione cambia
        p.set_velocity(Vec3::new(10.0, 0.0, 0.0));
        p.update_position(0.5);
        assert_abs_diff_eq!(p.position, Vec3::new(5.0, 0.0, 0.0));

        // in base al vettore velocità configurato
        p.set_velocity(Vec3::new(0.0, 10.0, 0.0));
        p.update_position(0.5);
        assert_abs_diff_eq!(p.position, Vec3::new(5.0, 5.0, 0.0));

        // ...e anche per l'asse z
        p.set_velocity(Vec3::new(0.0, 0.0, 10.0));
        p.update_position(0.5);
        assert_abs_diff_eq!(p.position, Vec3::new(5.0, 5.0, 5.0));
    }

    #[test]
    fn test_constant_acceleration() {
        let mut p = Body::new(1.0, 1.0);

        let dt = 0.1;
        let acceleration = 10.0;

        for _ in 0..10 {
            p.update_position(dt);
            p.accelerate(dt, Vec3::new(acceleration, 0.0, 0.0));
        }

        assert_abs_diff_eq!(p.velocity, Vec3::new(10.0, 0.0, 0.0), epsilon = 1e-2);
        assert_abs_diff_eq!(p.position, Vec3::new(4.5, 0.0, 0.0), epsilon = 1e-2);
    }

    #[test]
    fn test_direction() {
        let p = BodyBuilder::unitary()
            .velocity(Vec3::new(3.0, 4.0, 0.0))
            .build();

        let v = p.velocity_direction();
        assert_abs_diff_eq!(v, Vec3::new(0.6, 0.8, 0.0));
    }

    #[test]
    fn test_direction_when_stationary() {
        let p = Body::new(1.0, 1.0);
        let v = p.velocity_direction();
        assert_abs_diff_eq!(v, Vec3::zeros());
    }

    #[test]
    fn test_potential_energy_two_bodies() {
        let b1 = BodyBuilder::unitary()
            .position(Vec3::new(0.0, 0.0, 0.0))
            .build();
        let b2 = BodyBuilder::unitary()
            .position(Vec3::new(10.0, 0.0, 0.0))
            .build();
        let energy = potential_energy(G, &[b1, b2]);

        assert_abs_diff_eq!(energy, -G / 10.0);
    }

    #[test]
    fn test_potential_energy_three_bodies() {
        let bodies = [
            BodyBuilder::unitary()
                .position(Vec3::new(0.0, 0.0, 0.0))
                .build(),
            BodyBuilder::unitary()
                .mass(Mass::kg(2.0).unwrap())
                .position(Vec3::new(10.0, 0.0, 0.0))
                .build(),
            BodyBuilder::unitary()
                .mass(Mass::kg(3.0).unwrap())
                .position(Vec3::new(20.0, 0.0, 0.0))
                .build(),
        ];

        let energy = potential_energy(1.0, &bodies);
        assert_abs_diff_eq!(energy, -0.95);
    }

    #[test]
    fn test_com_2() {
        let bodies = [
            BodyBuilder::unitary()
                .position(Vec3::new(0.0, 0.0, 0.0))
                .build(),
            BodyBuilder::unitary()
                .mass(Mass::kg(9.0).unwrap())
                .position(Vec3::new(10.0, 0.0, 0.0))
                .build(),
        ];
        let com = center_of_mass(&bodies);
        assert_abs_diff_eq!(com, Vec3::new(9.0, 0.0, 0.0));
    }

    #[test]
    fn test_angular_momentum() {
        let body = BodyBuilder::unitary()
            .position(Vec3::new(2.0, 0.0, 0.0))
            .velocity(Vec3::new(0.0, 3.0, 0.0))
            .build();

        let l = body.angular_momentum();

        // r = (2, 0, 0)
        // p = m * v = (0, 3, 0)
        //
        // r × p = (0, 0, 6)

        assert_abs_diff_eq!(l, Vec3::new(0.0, 0.0, 6.0));
    }

    #[test]
    fn test_angular_momentum_depends_on_mass() {
        let body = BodyBuilder::unitary()
            .mass(Mass::kg(5.0).unwrap())
            .position(Vec3::new(2.0, 0.0, 0.0))
            .velocity(Vec3::new(0.0, 3.0, 0.0))
            .build();

        let l = body.angular_momentum();

        // p = m * v = (0, 15, 0)
        // L = r × p = (0, 0, 30)

        assert_abs_diff_eq!(l, Vec3::new(0.0, 0.0, 30.0));
    }
}
