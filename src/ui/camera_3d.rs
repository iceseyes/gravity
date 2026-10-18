use nalgebra::{Matrix4, Translation3, UnitQuaternion, Vector3};
use std::f64::consts::PI;

type Vec3 = Vector3<f64>;
type Rotation3 = UnitQuaternion<f64>;

pub struct Camera3DBuilder {
    position: Vec3,
    look_at: Vec3,
    fov_y: f64,
    aspect_ratio: f64,
    near: f64,
    far: f64,
}

impl Camera3DBuilder {
    pub fn new(position: Vec3, look_at: Vec3) -> Self {
        Self {
            position,
            look_at,
            fov_y: Camera3D::NORMAL_FOV_Y,
            aspect_ratio: Camera3D::DEFAULT_ASPECT_RATIO,
            near: Camera3D::DEFAULT_NEAR,
            far: Camera3D::DEFAULT_FAR,
        }
    }

    pub fn fov_y(mut self, fov_y: f64) -> Self {
        self.fov_y = fov_y;
        self
    }

    pub fn viewport_size(mut self, width: f64, height: f64) -> Self {
        assert!(width > 0.0 && height > 0.0);
        self.aspect_ratio = width / height;
        self
    }

    pub fn clipping_planes(mut self, near: f64, far: f64) -> Self {
        self.near = near;
        self.far = far;
        self
    }

    pub fn build(self) -> Camera3D {
        // nalgebra face_towards set the local z-axis towards the look_at point, so we need to negate
        // the look_at vector to make it point away from the camera. The up vector is y-axis
        Camera3D {
            position: self.position,
            orientation: Rotation3::face_towards(&(self.position - self.look_at), &Vec3::y()),
            fov_y: self.fov_y,
            aspect_ratio: self.aspect_ratio,
            near: self.near,
            far: self.far,
        }
    }
}

impl Default for Camera3DBuilder {
    fn default() -> Self {
        Self::new(Vec3::zeros(), Vec3::x())
    }
}

#[derive(Debug)]
pub struct Camera3D {
    position: Vec3,
    orientation: Rotation3,
    fov_y: f64,
    aspect_ratio: f64,
    near: f64,
    far: f64,
}

impl Camera3D {
    pub const NORMAL_FOV_Y: f64 = PI / 4.0;
    pub const DEFAULT_ASPECT_RATIO: f64 = 1.0;
    pub const DEFAULT_NEAR: f64 = 0.1;
    pub const DEFAULT_FAR: f64 = 100.0;

    pub fn move_to(&mut self, position: &Vec3) {
        self.position = *position;
    }

    pub fn orientation(&self) -> &Rotation3 {
        &self.orientation
    }

    pub fn look_at(&mut self, target: &Vec3) {
        let direction = target - self.position;

        if direction.norm_squared() > f64::EPSILON {
            self.orientation = Rotation3::face_towards(&direction, &Vec3::y());
        }
    }

    pub fn fov_y(&self) -> f64 {
        self.fov_y
    }

    pub fn fov_x(&self) -> f64 {
        2.0 * ((self.fov_y / 2.0).tan() * self.aspect_ratio).atan()
    }

    pub fn aspect_ratio(&self) -> f64 {
        self.aspect_ratio
    }

    pub fn near(&self) -> f64 {
        self.near
    }

    pub fn far(&self) -> f64 {
        self.far
    }

    /// Transforms world-space coordinates into camera-space coordinates.
    pub fn view_matrix(&self) -> Matrix4<f64> {
        let inverse_rotation = self.orientation.inverse();
        let inverse_translation = Translation3::from(-self.position);

        inverse_rotation.to_homogeneous() * inverse_translation.to_homogeneous()
    }
}

#[cfg(test)]
mod tests {
    use crate::ui::camera_3d::{Camera3D, Camera3DBuilder, Vec3};
    use approx::assert_abs_diff_eq;
    use nalgebra::Matrix4;
    use std::f64::consts::PI;

    #[test]
    fn test_fov_x() {
        let camera = Camera3DBuilder::default()
            .fov_y(Camera3D::NORMAL_FOV_Y)
            .viewport_size(1.0, 1.0)
            .build();
        assert_abs_diff_eq!(camera.fov_x(), PI / 4.0, epsilon = 1e-12);

        let camera = Camera3DBuilder::default()
            .fov_y(Camera3D::NORMAL_FOV_Y)
            .viewport_size(16.0, 9.0)
            .build();
        assert_abs_diff_eq!(camera.fov_x(), 72.734_f64.to_radians(), epsilon = 1e-3);
    }

    #[test]
    fn test_view_matrix_identity() {
        let camera = Camera3DBuilder::new(Vec3::zeros(), Vec3::new(0.0, 0.0, -1.0)).build();

        assert_abs_diff_eq!(camera.view_matrix(), Matrix4::identity(), epsilon = 1e-12);
    }

    #[test]
    fn test_view_matrix_translation() {
        use nalgebra::Vector4;

        let camera = Camera3DBuilder::new(Vec3::new(0.0, 0.0, 5.0), Vec3::zeros()).build();

        let point = Vector4::new(0.0, 0.0, 0.0, 1.0);
        let camera_point = camera.view_matrix() * point;

        assert!((camera_point.z + 5.0).abs() < 1e-12);
    }
}
