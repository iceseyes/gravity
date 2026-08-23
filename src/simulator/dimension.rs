use crate::physics::geometry::Sphere;

pub struct Mass(f64);

impl Mass {
    pub fn kg(mass: f64) -> anyhow::Result<Self> {
        if mass <= 0.0 {
            anyhow::bail!("Mass must be greater than zero");
        }

        Ok(Self(mass))
    }

    pub fn epsilon() -> Self {
        Self(f64::MIN_POSITIVE)
    }

    pub fn get(&self) -> f64 {
        self.0
    }
}

pub struct Radius(RadiusProvider, f64);

impl Radius {
    pub fn m(radius: f64) -> anyhow::Result<Self> {
        if radius <= 0.0 {
            anyhow::bail!("Radius must be greater than zero");
        }

        Ok(Self(RadiusProvider::Absolute, radius))
    }

    pub fn by_density(density: f64) -> anyhow::Result<Self> {
        if density <= 0.0 {
            anyhow::bail!("Density must be greater than zero");
        }

        Ok(Self(RadiusProvider::Density, density))
    }

    pub fn get(&self, mass: Mass) -> f64 {
        match self.0 {
            RadiusProvider::Absolute => self.1,
            RadiusProvider::Density => {
                if let Ok(sphere) = Sphere::by_volume(mass.get() / self.1) {
                    sphere.radius()
                } else {
                    f64::MIN_POSITIVE
                }
            }
        }
    }
}

enum RadiusProvider {
    Absolute,
    Density,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::physics::F64_EPSILON;
    use std::f64::consts::PI;

    const EPSILON: f64 = 1e-12;

    #[test]
    fn kg_accepts_positive_mass() {
        let mass = Mass::kg(10.0).unwrap();

        assert_eq!(mass.get(), 10.0);
    }

    #[test]
    fn kg_rejects_zero_mass() {
        let result = Mass::kg(0.0);

        assert!(result.is_err());
    }

    #[test]
    fn kg_rejects_negative_mass() {
        let result = Mass::kg(-10.0);

        assert!(result.is_err());
    }

    #[test]
    fn epsilon_mass_is_good() {
        let result = Mass::kg(Mass::epsilon().get());
        assert!(result.is_ok());
    }

    #[test]
    fn kg_accepts_mass_greater_than_epsilon() {
        let mass = Mass::kg(F64_EPSILON * 2.0).unwrap();

        assert_eq!(mass.get(), F64_EPSILON * 2.0);
    }

    #[test]
    fn m_accepts_positive_radius() {
        let radius = Radius::m(10.0).unwrap();

        assert_eq!(radius.get(Mass::kg(100.0).unwrap()), 10.0);
    }

    #[test]
    fn m_rejects_zero_radius() {
        let result = Radius::m(0.0);

        assert!(result.is_err());
    }

    #[test]
    fn m_rejects_negative_radius() {
        let result = Radius::m(-10.0);

        assert!(result.is_err());
    }

    #[test]
    fn m_accepts_epsilon_radius() {
        let result = Radius::m(F64_EPSILON);
        assert!(result.is_ok());
    }

    #[test]
    fn m_accepts_radius_greater_than_epsilon() {
        let radius = Radius::m(F64_EPSILON * 2.0).unwrap();
        let mass = Mass::kg(1.0).unwrap();

        assert_eq!(radius.get(mass), F64_EPSILON * 2.0);
    }

    #[test]
    fn by_density_accepts_positive_density() {
        let density = Radius::by_density(1000.0).unwrap();
        let mass = Mass::kg(1000.0).unwrap();

        let radius = density.get(mass);

        assert!(radius > 0.0);
    }

    #[test]
    fn by_density_rejects_zero_density() {
        let result = Radius::by_density(0.0);

        assert!(result.is_err());
    }

    #[test]
    fn by_density_rejects_negative_density() {
        let result = Radius::by_density(-1000.0);

        assert!(result.is_err());
    }

    #[test]
    fn by_density_epsilon_density() {
        let result = Radius::by_density(F64_EPSILON).unwrap();
        assert!(result.get(Mass::epsilon()) > 0.0);
    }

    #[test]
    fn by_density_accepts_density_greater_than_epsilon() {
        let density = Radius::by_density(F64_EPSILON * 2.0).unwrap();
        let mass = Mass::kg(1.0).unwrap();

        assert!(density.get(mass) > 0.0);
    }

    #[test]
    fn by_density_calculates_correct_radius() {
        let mass = 1000.0;
        let density = 1000.0;

        let expected_volume = mass / density;
        let expected_radius = (expected_volume / ((4.0 / 3.0) * PI)).cbrt();

        let radius = Radius::by_density(density)
            .unwrap()
            .get(Mass::kg(mass).unwrap());

        assert!((radius - expected_radius).abs() < EPSILON);
    }

    #[test]
    fn density_radius_is_independent_of_mass_object() {
        let density = Radius::by_density(1000.0).unwrap();

        let mass_1 = Mass::kg(1000.0).unwrap();
        let mass_2 = Mass::kg(8000.0).unwrap();

        let radius_1 = density.get(mass_1);
        let radius_2 = density.get(mass_2);

        assert!(radius_2 > radius_1);
    }

    #[test]
    fn by_density_produces_radius_with_correct_volume() {
        let mass = 8000.0;
        let density = 1000.0;
        let radius = Radius::by_density(density)
            .unwrap()
            .get(Mass::kg(mass).unwrap());

        let sphere = Sphere::by_radius(radius).unwrap();
        let expected_volume = mass / density;

        assert!((sphere.volume() - expected_volume).abs() < 1e-12);
    }
}
