use crate::physics::{F64_EPSILON, Vec3};

pub const MIN_DISTANCE_SQUARED: f64 = 1e-12;

/// Returns the intensity of the gravitational field at a given distance for a given mass,
/// excluding the gravitational constant.
///
/// For instance, given a 1kg body at a distance of 10m, the intensity of the gravitational field is:
/// ```
/// use approx::assert_relative_eq;
/// use gravity::physics::{gravity::gravity_field, G, Vec3};
///
/// let a = gravity_field(1.0, Vec3::new(10.0, 0.0, 0.0));
///
/// assert_relative_eq!(G * a, Vec3::new(6.674080472394825e-13, 0.0, 0.0));
/// ```
pub fn gravity_field(m: f64, d: Vec3) -> Vec3 {
    let distance_squared = d.norm_squared();

    if distance_squared > MIN_DISTANCE_SQUARED {
        let distance = distance_squared.sqrt();
        let factor = m / (distance_squared * distance);

        d * factor
    } else {
        Vec3::zeros()
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
/// use gravity::physics::{G, Vec3};
///
/// let u = potential_energy(1.0, 2.0, Vec3::new(3.0, 4.0, 5.0));
/// assert_eq!(G * u, G as f64 * -2.0 / 50_f64.sqrt());
/// ```
pub fn potential_energy(m1: f64, m2: f64, d: Vec3) -> f64 {
    let r2 = d.norm_squared();

    if r2 < F64_EPSILON {
        0.0
    } else {
        -(m1 * m2 / r2.sqrt())
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
/// let v = (G * v_squared).sqrt();
/// assert!((v - 2.583424e4).abs() < 1e-2);
/// ```
pub fn orbital_squared_velocity(mass: f64, distance: f64) -> f64 {
    mass / distance
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::physics::{G, Vec3};
    use approx::{assert_abs_diff_eq, assert_relative_eq};

    #[test]
    fn test_gravity_field() {
        let a = gravity_field(1.0, Vec3::new(10.0, 0.0, 0.0));
        assert_abs_diff_eq!(a, Vec3::new(0.01, 0.0, 0.0));
        assert_relative_eq!((G * a).norm(), 6.674080472394825e-13);
    }
}
