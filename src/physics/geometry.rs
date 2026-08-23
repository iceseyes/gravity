use anyhow::{Result, bail};
use std::f64::consts::PI;

#[derive(Debug, Copy, Clone, PartialOrd, PartialEq)]
pub struct Sphere(f64);

impl Sphere {
    const MIN_VOLUME: f64 = f64::MIN_POSITIVE;
    const SHAPE_FACTOR: f64 = (4.0 / 3.0) * PI;

    pub fn by_radius(radius: f64) -> Result<Self> {
        if radius <= Self::min_radius() {
            bail!("Radius must be positive")
        } else {
            Ok(Self(radius))
        }
    }

    pub fn by_volume(volume: f64) -> Result<Self> {
        if volume <= Self::MIN_VOLUME {
            bail!("Volume must be positive")
        } else {
            Ok(Self((volume / Self::SHAPE_FACTOR).cbrt()))
        }
    }

    pub fn radius(&self) -> f64 {
        self.0
    }

    pub fn volume(&self) -> f64 {
        Self::SHAPE_FACTOR * self.0.powi(3)
    }

    #[inline]
    fn min_radius() -> f64 {
        (f64::MIN_POSITIVE / Self::SHAPE_FACTOR).cbrt()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::f64::consts::PI;

    const EPSILON: f64 = 1e-12;

    #[test]
    fn by_radius_creates_sphere_with_given_radius() {
        let sphere = Sphere::by_radius(10.0).unwrap();
        assert_eq!(sphere.radius(), 10.0);
    }

    #[test]
    fn by_radius_rejects_zero() {
        let sphere = Sphere::by_radius(0.0);
        assert!(sphere.is_err());
    }

    #[test]
    fn by_radius_rejects_negative_radius_to_min_positive() {
        let sphere = Sphere::by_radius(-10.0);
        assert!(sphere.is_err());
    }

    #[test]
    fn by_radius_assure_min_positive_as_volume() {
        let sphere = Sphere::by_radius(f64::MIN_POSITIVE);
        assert!(sphere.is_err());

        let sphere = Sphere::by_radius(f64::MIN_POSITIVE.sqrt());
        assert!(sphere.is_err());

        let sphere = Sphere::by_radius((f64::MIN_POSITIVE / Sphere::SHAPE_FACTOR).cbrt());
        assert!(sphere.is_err());

        let sphere = Sphere::by_radius(f64::MIN_POSITIVE.cbrt());
        assert!(sphere.is_ok());

        let sphere =
            Sphere::by_radius((f64::MIN_POSITIVE / Sphere::SHAPE_FACTOR).cbrt() + f64::EPSILON);
        assert!(sphere.is_ok());
    }

    #[test]
    fn volume_is_correct() {
        let radius = 2.0;
        let sphere = Sphere::by_radius(radius).unwrap();
        let expected = (4.0 / 3.0) * PI * radius.powi(3);

        assert!((sphere.volume() - expected).abs() < EPSILON);
    }

    #[test]
    fn by_volume_creates_sphere_with_correct_radius() {
        let radius = 2.0f64;
        let volume = (4.0 / 3.0) * PI * radius.powi(3);
        let sphere = Sphere::by_volume(volume).unwrap();

        assert!((sphere.radius() - radius).abs() < EPSILON);
    }

    #[test]
    fn by_volume_round_trip() {
        let sphere = Sphere::by_radius(10.0).unwrap();
        let volume = sphere.volume();
        let reconstructed = Sphere::by_volume(volume).unwrap();

        assert!((reconstructed.radius() - sphere.radius()).abs() < EPSILON);
    }

    #[test]
    fn by_volume_clamps_zero_to_min_positive() {
        let sphere = Sphere::by_volume(0.0);
        assert!(sphere.is_err());
    }

    #[test]
    fn by_volume_clamps_negative_volume_to_min_positive() {
        let sphere = Sphere::by_volume(-100.0);
        assert!(sphere.is_err());
    }

    #[test]
    fn volume_of_minimum_sphere_is_positive() {
        let sphere = Sphere::by_radius(f64::MIN_POSITIVE.cbrt()).unwrap();
        assert!(
            sphere.volume() > f64::MIN_POSITIVE,
            "volume: {}",
            sphere.volume()
        );
    }
}
