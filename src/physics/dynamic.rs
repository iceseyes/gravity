use crate::physics::F64_EPSILON;

/// Momentum of a body.
///
/// Example:
/// ```
/// use gravity::physics::dynamic::momentum;
///
/// let q = momentum(1.0, 1.0, 0.0, 0.0);
/// assert_eq!(q, (1.0, 0.0, 0.0));
///
/// let q = momentum(1.0, 0.0, 1.0, 0.0);
/// assert_eq!(q, (0.0, 1.0, 0.0));
///
/// let q = momentum(1.0, 0.0, 0.0, 1.0);
/// assert_eq!(q, (0.0, 0.0, 1.0));
///
/// let q = momentum(5.0, 1.0, 2.0, 3.0);
/// assert_eq!(q, (5.0, 10.0, 15.0));
/// ```
pub fn momentum(mass: f32, vx: f32, vy: f32, vz: f32) -> (f64, f64, f64) {
    (
        mass as f64 * vx as f64,
        mass as f64 * vy as f64,
        mass as f64 * vz as f64,
    )
}

/// Returns the center of mass of a body.
///
/// Example:
/// ```
/// use gravity::physics::dynamic::center_of_mass;
///
/// let b = center_of_mass(10.0, 3.0, 4.0, 5.0);
/// assert_eq!(b, (30.0, 40.0, 50.0));
/// ```
pub fn center_of_mass(mass: f32, x: f32, y: f32, z: f32) -> (f64, f64, f64) {
    (
        x as f64 * mass as f64,
        y as f64 * mass as f64,
        z as f64 * mass as f64,
    )
}

/// Compute Kinetic Energy.
///
/// $$E_k = \frac{1}{2} m v^2$$
///
/// Where $m$ is the mass of the body, $v$ is the velocity of the body.
///
/// Example:
/// ```
/// use gravity::physics::dynamic::kinetic_energy;
///
/// let ec = kinetic_energy(1.0, 2.0, 3.0, 4.0);
/// assert_eq!(ec, 14.5);
/// ```
pub fn kinetic_energy(mass: f32, vx: f32, vy: f32, vz: f32) -> f64 {
    let v2 = (vx * vx) + (vy * vy) + (vz * vz);
    0.5 * mass as f64 * v2 as f64
}

/// Compute Potential Energy.
///
/// This function computes Potential Energy between two bodies using $G=1$. To get
/// the potential energy, you have to apply the gravitational constant.
///
/// Example:
/// ```
/// use gravity::physics::dynamic::potential_energy;
/// use gravity::physics::G;
///
/// let u = potential_energy(1.0, 2.0, 3.0, 4.0, 5.0);
/// assert_eq!(G as f64 * u, G as f64 * -2.0 / 50_f64.sqrt());
/// ```
pub fn potential_energy(m1: f32, m2: f32, dx: f32, dy: f32, dz: f32) -> f64 {
    let dx = dx as f64;
    let dy = dy as f64;
    let dz = dz as f64;
    let r2 = (dx * dx) + (dy * dy) + (dz * dz);

    if r2 < F64_EPSILON {
        0.0
    } else {
        -(m1 as f64 * m2 as f64 / r2.sqrt())
    }
}

/// Compute the orthogonal velocity to stay in circular orbit by the given mass.
/// The value is the squared velocity using $G=1$. To get the velocity, you have to apply the
/// gravitational constant and compute the squared root of the result.
///
/// Example:
/// ```
/// use gravity::physics::dynamic::orbital_squared_velocity;
/// use gravity::physics::G;
///
/// let v_squared = orbital_squared_velocity(1.0e30, 1.0e11);
/// let v = (G as f64 * v_squared).sqrt();
/// assert!((v - 2.583424e4).abs() < 1e-2);
/// ```
pub fn orbital_squared_velocity(mass: f32, distance: f32) -> f64 {
    let distance = distance as f64;
    mass as f64 / distance
}
