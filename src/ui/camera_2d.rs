use eframe::emath::{Pos2, Rect, Vec2};
use gravity::simulator::World;

pub struct Camera2D {
    center: Pos2, // meters (world space)
    scale: f32,   // pixels per meter
    padding: f32, // percentage of empty space around the world
}

impl Camera2D {
    pub fn new(center: Pos2, scale: f32) -> Self {
        Self {
            center,
            scale,
            padding: 0.1,
        }
    }

    pub fn scale(&self) -> f32 {
        self.scale
    }

    pub fn set_scale(&mut self, pixel_per_meter: f32) {
        self.scale = pixel_per_meter.clamp(1e-12, 1e12);
    }

    pub fn pan(&mut self, delta: Vec2) {
        self.center.x -= delta.x / self.scale;
        self.center.y += delta.y / self.scale;
    }

    pub fn zoom_at(&mut self, viewport: &Rect, factor: f32, screen_pos: Pos2) {
        let world_pos = self.point_to_world(viewport, screen_pos);
        self.set_scale(self.scale * factor);
        let offset = screen_pos - viewport.center();

        self.center = Pos2::new(
            world_pos.0 - offset.x / self.scale,
            world_pos.1 + offset.y / self.scale,
        );
    }

    pub fn fit(&mut self, viewport: &Rect, world: &World) {
        let (ox, oy) = world.origin();
        let ox = ox + world.width() / 2.0;
        let oy = oy + world.height() / 2.0;
        let width = world.width().max(1.0) * (1.0 + self.padding);
        let height = world.height().max(1.0) * (1.0 + self.padding);
        let scale_x = viewport.width() / width;
        let scale_y = viewport.height() / height;

        self.center = Pos2::new(ox, oy);
        self.set_scale(scale_x.min(scale_y));
    }

    pub fn point_to_screen(&self, viewport: &Rect, x: f32, y: f32, _z: f32) -> Pos2 {
        let viewport_center = viewport.center();
        let (x, y) = (x - self.center.x, y - self.center.y);
        let (sx, sy) = (x * self.scale, y * self.scale);

        Pos2::new(viewport_center.x + sx, viewport_center.y - sy)
    }

    pub fn point_to_world(&self, viewport: &Rect, point: Pos2) -> (f32, f32, f32) {
        let viewport_center = viewport.center();
        let (x, y) = (point.x - viewport_center.x, viewport_center.y - point.y);
        let (sx, sy) = (x / self.scale, y / self.scale);

        (self.center.x + sx, self.center.y + sy, 0.0)
    }

    pub fn length_to_screen(&self, length: f32) -> f32 {
        length * self.scale
    }

    pub fn length_to_world(&self, length: f32) -> f32 {
        length / self.scale
    }
}

impl Default for Camera2D {
    fn default() -> Self {
        Self::new(Pos2::ZERO, 1.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use gravity::physics::assert_approx_eq;
    use gravity::simulator::Body;
    use gravity::simulator::world::World;

    fn viewport() -> Rect {
        Rect::from_min_max(Pos2::new(0.0, 0.0), Pos2::new(1000.0, 500.0))
    }

    #[test]
    fn test_origin() {
        let camera = Camera2D::default();

        let screen = camera.point_to_screen(&viewport(), 0.0, 0.0, 0.0);

        assert_approx_eq(screen.x, 500.0);
        assert_approx_eq(screen.y, 250.0);
    }

    #[test]
    fn test_positive_x() {
        let camera = Camera2D::default();

        let screen = camera.point_to_screen(&viewport(), 10.0, 0.0, 0.0);

        assert_approx_eq(screen.x, 510.0);
        assert_approx_eq(screen.y, 250.0);
    }

    #[test]
    fn test_negative_x() {
        let camera = Camera2D::default();

        let screen = camera.point_to_screen(&viewport(), -10.0, 0.0, 0.0);

        assert_approx_eq(screen.x, 490.0);
        assert_approx_eq(screen.y, 250.0);
    }

    #[test]
    fn test_positive_y() {
        let camera = Camera2D::default();

        let screen = camera.point_to_screen(&viewport(), 0.0, 10.0, 0.0);

        // World +Y corrisponde a screen -Y.
        assert_approx_eq(screen.x, 500.0);
        assert_approx_eq(screen.y, 240.0);
    }

    #[test]
    fn test_negative_y() {
        let camera = Camera2D::default();

        let screen = camera.point_to_screen(&viewport(), 0.0, -10.0, 0.0);

        assert_approx_eq(screen.x, 500.0);
        assert_approx_eq(screen.y, 260.0);
    }

    #[test]
    fn test_scale() {
        let camera = Camera2D::new(Pos2::ZERO, 10.0);

        let screen = camera.point_to_screen(&viewport(), 10.0, 5.0, 0.0);

        assert_approx_eq(screen.x, 600.0);
        assert_approx_eq(screen.y, 200.0);
    }

    #[test]
    fn test_camera_center() {
        let camera = Camera2D::new(Pos2::new(100.0, 50.0), 2.0);

        let screen = camera.point_to_screen(&viewport(), 100.0, 50.0, 0.0);

        // Il centro della camera deve essere il centro del viewport.
        assert_approx_eq(screen.x, 500.0);
        assert_approx_eq(screen.y, 250.0);
    }

    #[test]
    fn test_point_relative_to_camera_center() {
        let camera = Camera2D::new(Pos2::new(100.0, 50.0), 2.0);

        let screen = camera.point_to_screen(&viewport(), 110.0, 60.0, 0.0);

        // +10m X -> +20px
        // +10m Y -> -20px sullo schermo
        assert_approx_eq(screen.x, 520.0);
        assert_approx_eq(screen.y, 230.0);
    }

    #[test]
    fn test_z_is_ignored() {
        let camera = Camera2D::default();

        let p1 = camera.point_to_screen(&viewport(), 10.0, 20.0, 0.0);

        let p2 = camera.point_to_screen(&viewport(), 10.0, 20.0, 1000.0);

        assert_approx_eq(p1.x, p2.x);
        assert_approx_eq(p1.y, p2.y);
    }

    #[test]
    fn test_scale_is_pixels_per_meter() {
        let camera = Camera2D::new(Pos2::ZERO, 5.0);

        let origin = camera.point_to_screen(&viewport(), 0.0, 0.0, 0.0);

        let point = camera.point_to_screen(&viewport(), 1.0, 0.0, 0.0);

        assert_approx_eq(point.x - origin.x, 5.0);
        assert_approx_eq(point.y - origin.y, 0.0);
    }

    #[test]
    fn test_new() {
        let camera = Camera2D::new(Pos2::new(10.0, 20.0), 5.0);

        assert_approx_eq(camera.center.x, 10.0);
        assert_approx_eq(camera.center.y, 20.0);
        assert_approx_eq(camera.scale, 5.0);
        assert_approx_eq(camera.padding, 0.1);
    }

    #[test]
    fn test_fit_center() {
        let mut p1 = Body::new(1.0, 1.0);
        let mut p2 = Body::new(1.0, 1.0);

        p1.move_to(-100.0, -50.0, 0.0);
        p2.move_to(100.0, 50.0, 0.0);

        let mut world = World::default();
        world.add_body(p1);
        world.add_body(p2);

        let mut camera = Camera2D::default();

        camera.fit(&viewport(), &world);

        // Centro del world:
        //
        // x = (-101 + 101) / 2 = 0
        // y = (-51 + 51) / 2 = 0
        assert_approx_eq(camera.center.x, 0.0);
        assert_approx_eq(camera.center.y, 0.0);
    }

    #[test]
    fn test_fit_center_with_offset_world() {
        let mut p1 = Body::new(1.0, 1.0);
        let mut p2 = Body::new(1.0, 1.0);

        p1.move_to(100.0, 200.0, 0.0);
        p2.move_to(300.0, 400.0, 0.0);

        let mut world = World::default();
        world.add_body(p1);
        world.add_body(p2);

        let mut camera = Camera2D::default();

        camera.fit(&viewport(), &world);

        // Bounds considerando il radius:
        //
        // x = 99 .. 301
        // y = 199 .. 401
        //
        // centro = (200, 300)
        assert_approx_eq(camera.center.x, 200.0);
        assert_approx_eq(camera.center.y, 300.0);
    }

    #[test]
    fn test_fit_uses_smallest_scale() {
        let mut p1 = Body::new(1.0, 0.0);
        let mut p2 = Body::new(1.0, 0.0);

        // World: 1000 x 100
        p1.move_to(0.0, 0.0, 0.0);
        p2.move_to(1000.0, 100.0, 0.0);

        let mut world = World::default();
        world.add_body(p1);
        world.add_body(p2);

        let mut camera = Camera2D::default();

        camera.fit(&viewport(), &world);

        let world_width = 1000.0f32 * 1.1;
        let world_height = 100.0f32 * 1.1;

        let scale_x = 1000.0 / world_width;
        let scale_y = 500.0 / world_height;

        let expected_scale = scale_x.min(scale_y);

        assert_approx_eq(camera.scale, expected_scale);
    }

    #[test]
    fn test_fit_with_padding() {
        let mut p1 = Body::new(1.0, 0.0);
        let mut p2 = Body::new(1.0, 0.0);

        p1.move_to(0.0, 0.0, 0.0);
        p2.move_to(100.0, 100.0, 0.0);

        let mut world = World::default();
        world.add_body(p1);
        world.add_body(p2);

        let mut camera = Camera2D::new(Pos2::ZERO, 1.0);

        camera.fit(&viewport(), &world);

        // Il world è 100x100.
        // Con 10% di padding:
        //
        // 100 * 1.1 = 110
        //
        // Il viewport è 1000x500, quindi:
        //
        // scale_x = 1000 / 110
        // scale_y = 500 / 110
        //
        // viene scelta scale_y.
        let expected_scale = 500.0 / 110.0;

        assert_approx_eq(camera.scale, expected_scale);
    }

    #[test]
    fn test_fit_does_not_distort_aspect_ratio() {
        let mut p1 = Body::new(1.0, 0.0);
        let mut p2 = Body::new(1.0, 0.0);

        // World 200 x 100
        p1.move_to(0.0, 0.0, 0.0);
        p2.move_to(100.0, 100.0, 0.0);

        let mut world = World::default();
        world.add_body(p1);
        world.add_body(p2);

        let mut camera = Camera2D::default();

        camera.fit(&viewport(), &world);

        let scale_x = 1000.0 / (100.0 * 1.1);
        let scale_y = 500.0 / (100.0 * 1.1);

        // scale_y deve essere quella utilizzata.
        assert_approx_eq(camera.scale, scale_y);
        assert!(
            camera.scale < scale_x,
            "scale = {}, scale_x = {}, scale_y = {}",
            camera.scale,
            scale_x,
            scale_y
        );
    }

    #[test]
    fn test_fit_single_particle() {
        let particle = Body::new(1.0, 1.0);

        let mut world = World::default();
        world.add_body(particle);

        let mut camera = Camera2D::default();

        camera.fit(&viewport(), &world);

        // Con una sola particella width e height sono 2
        // perché il radius viene considerato nei bounds.
        //
        // width  = 2
        // height = 2
        //
        // padding = 10%
        // => 2.2
        //
        // scale = min(1000/2.2, 500/2.2)
        let expected_scale = 500.0 / 2.2;

        assert_approx_eq(camera.center.x, 0.0);
        assert_approx_eq(camera.center.y, 0.0);
        assert_approx_eq(camera.scale, expected_scale);
    }

    #[test]
    fn test_world_screen_round_trip() {
        let camera = Camera2D::new(Pos2::new(100.0, 50.0), 10.0);
        let viewport = viewport();
        let original = (120.0, 70.0, 0.0);
        let screen = camera.point_to_screen(&viewport, original.0, original.1, original.2);
        let result = camera.point_to_world(&viewport, screen);

        assert_approx_eq(result.0, original.0);
        assert_approx_eq(result.1, original.1);
        assert_approx_eq(result.2, original.2);
    }

    #[test]
    fn test_zoom_at_viewport_center() {
        let viewport = viewport();

        let mut camera = Camera2D::new(Pos2::ZERO, 10.0);

        let mouse = viewport.center();

        let before = camera.point_to_world(&viewport, mouse);

        camera.zoom_at(&viewport, 2.0, mouse);

        let after = camera.point_to_world(&viewport, mouse);

        assert_approx_eq(before.0, after.0);
        assert_approx_eq(before.1, after.1);

        assert_approx_eq(camera.scale, 20.0);
    }

    #[test]
    fn test_zoom_at_mouse_position() {
        let viewport = viewport();

        let mut camera = Camera2D::new(Pos2::ZERO, 10.0);

        let mouse = Pos2::new(700.0, 150.0);

        let before = camera.point_to_world(&viewport, mouse);

        camera.zoom_at(&viewport, 2.0, mouse);

        let after = camera.point_to_world(&viewport, mouse);

        assert_approx_eq(before.0, after.0);
        assert_approx_eq(before.1, after.1);
    }
}
