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
    let vx = vx as f64;
    let vy = vy as f64;
    let vz = vz as f64;
    let v2 = (vx * vx) + (vy * vy) + (vz * vz);
    0.5 * mass as f64 * v2
}
