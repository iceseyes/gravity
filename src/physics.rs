use nalgebra::Vector3;

pub mod dynamic;
pub mod geometry;
pub mod gravity;

pub const G: f64 = 6.67408e-11;
pub const SPEED_OF_LIGHT: f32 = 299792458.0;
pub const EPSILON: f32 = 1e-6;
pub const F64_EPSILON: f64 = 1e-20;

pub type Vec3 = Vector3<f64>;
