use crate::physics::Vec3;

/// Momentum of a body.
///
/// Example:
/// ```
/// use approx::assert_abs_diff_eq;
/// use gravity::physics::dynamic::momentum;
/// use gravity::physics::Vec3;
///
/// let q = momentum(1.0, Vec3::new(1.0, 0.0, 0.0));
/// assert_abs_diff_eq!(q, Vec3::new(1.0, 0.0, 0.0));
///
/// let q = momentum(1.0, Vec3::new(0.0, 1.0, 0.0));
/// assert_abs_diff_eq!(q, Vec3::new(0.0, 1.0, 0.0));
///
/// let q = momentum(1.0, Vec3::new(0.0, 0.0, 1.0));
/// assert_abs_diff_eq!(q, Vec3::new(0.0, 0.0, 1.0));
///
/// let q = momentum(5.0, Vec3::new(1.0, 2.0, 3.0));
/// assert_abs_diff_eq!(q, Vec3::new(5.0, 10.0, 15.0));
/// ```
pub fn momentum(mass: f64, v: Vec3) -> Vec3 {
    mass * v
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
/// use gravity::physics::Vec3;
///
/// let ec = kinetic_energy(1.0, Vec3::new(2.0, 3.0, 4.0));
/// assert_eq!(ec, 14.5);
/// ```
pub fn kinetic_energy(mass: f64, v: Vec3) -> f64 {
    0.5 * mass * v.norm_squared()
}
