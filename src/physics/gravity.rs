use crate::physics::EPSILON;

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

    if distance_squared > EPSILON {
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
