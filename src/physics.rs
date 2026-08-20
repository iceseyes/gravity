pub mod gravity;

pub const G: f32 = 6.67408e-11;
pub const SPEED_OF_LIGHT: f32 = 299792458.0;
pub const EPSILON: f32 = 1e-6;

pub fn assert_approx_eq(actual: f32, expected: f32) {
    assert!(
        (actual - expected).abs() < EPSILON,
        "expected {expected}, got {actual}"
    );
}
