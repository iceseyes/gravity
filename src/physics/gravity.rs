use crate::physics::F64_EPSILON;

pub const MIN_DISTANCE_SQUARED: f32 = 1e-12;

/// Returns the intensity of the gravitational field at a given distance for a given mass,
/// excluding the gravitational constant.
///
/// For instance, given a 1kg body at a distance of 10m, the intensity of the gravitational field is:
/// ```
/// use gravity::physics::{gravity::gravity_field, G};
///
/// let (ax, ay, az) = gravity_field(1.0, 10.0, 0.0, 0.0);
///
/// assert_eq!(G * ax, 6.674081e-13);
/// assert_eq!(G * ay, 0.0);
/// assert_eq!(G * az, 0.0);
/// ```
pub fn gravity_field(m: f32, dx: f32, dy: f32, dz: f32) -> (f32, f32, f32) {
    let distance_squared = dx * dx + dy * dy + dz * dz;

    if distance_squared > MIN_DISTANCE_SQUARED {
        let distance = distance_squared.sqrt();
        let factor = m / (distance_squared * distance);

        (dx * factor, dy * factor, dz * factor)
    } else {
        (0.0, 0.0, 0.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::physics::{G, assert_approx_eq};

    #[test]
    fn test_gravity_field() {
        let (ax, ay, az) = gravity_field(1.0, 10.0, 0.0, 0.0);
        assert_approx_eq(ax, 0.01);
        assert_approx_eq(ay, 0.0);
        assert_approx_eq(az, 0.0);

        assert_eq!(G * ax, 6.674081e-13);
    }
}

/// Compute Potential Energy.
///
/// This function computes Potential Energy between two bodies using $G=1$. To get
/// the potential energy, you have to apply the gravitational constant.
///
/// Example:
/// ```
/// use gravity::physics::gravity::potential_energy;
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
/// use gravity::physics::gravity::orbital_squared_velocity;
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
