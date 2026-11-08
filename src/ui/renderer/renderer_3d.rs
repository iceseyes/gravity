use crate::ui::camera::camera_3d::{Camera3D, Camera3DBuilder};
use crate::ui::renderer::Renderer;
use eframe::egui::{self, Color32, Painter, Pos2, Rect};
use gravity::math;
use gravity::math::origin;
use gravity::simulator::{Body, World};
use math::Vec3;
use nalgebra::Vector4;
use std::f64::consts::PI;

#[derive(Debug)]
pub struct Renderer3D {
    camera: Camera3D,
    target: Vec3,
    yaw: f64,
    pitch: f64,
    distance: f64,
    grid_size: i32,
}

impl Default for Renderer3D {
    fn default() -> Self {
        Self::new(Camera3DBuilder::default().build())
    }
}

impl Renderer3D {
    pub fn new(camera: Camera3D) -> Self {
        Self {
            camera,
            target: Vec3::zeros(),
            yaw: 0.0,
            pitch: 0.0,
            distance: 10.0,
            grid_size: 40,
        }
    }

    pub fn target(&self) -> &Vec3 {
        &self.target
    }

    pub fn look_at(&mut self, target: Vec3, viewport: &Rect) {
        self.target = target;
        self.rebuild_camera(viewport.width(), viewport.height());
    }

    pub fn focus_on(&mut self, body: &Body, viewport: &Rect) {
        self.look_at(body.position(), viewport);
    }

    fn rebuild_camera(&mut self, width: f32, height: f32) {
        if width <= 0.0 || height <= 0.0 {
            return;
        }

        let cp = self.pitch.cos();

        let offset = Vec3::new(
            self.distance * cp * self.yaw.sin(),
            self.distance * self.pitch.sin(),
            self.distance * cp * self.yaw.cos(),
        );

        let near = (self.distance * 0.001).max(1e-6);
        let far = (self.distance * 100.0).max(near * 100.0);

        self.camera = Camera3DBuilder::new(self.target + offset, self.target)
            .fov_y(Camera3D::NORMAL_FOV_Y)
            .viewport_size(f64::from(width), f64::from(height))
            .clipping_planes(near, far)
            .build();
    }

    fn draw_body(&self, viewport: &Rect, body: &Body) -> Option<(f64, Pos2, f32, Color32)> {
        let p = body.position();

        let camera_pos = self.camera.view_matrix() * Vector4::new(p.x, p.y, p.z, 1.0);

        let depth = -camera_pos.z;

        if !depth.is_finite() || depth < self.camera.near() || depth > self.camera.far() {
            return None;
        }

        let screen = self.camera.project(
            &p,
            f64::from(viewport.width()),
            f64::from(viewport.height()),
        )?;

        let screen_pos = Pos2::new(
            viewport.left() + screen.x as f32,
            viewport.top() + screen.y as f32,
        );

        let focal_length = f64::from(viewport.height()) / (2.0 * (self.camera.fov_y() * 0.5).tan());

        let pixel_radius = (body.radius() * focal_length / depth).clamp(1.0, 100.0) as f32;

        Some((depth, screen_pos, pixel_radius, Color32::YELLOW))
    }

    fn grid_spacing(&self) -> f64 {
        let target_step = self.distance / 10.0;
        let exponent = target_step.log10().floor();
        let base = 10.0_f64.powf(exponent);
        let fraction = target_step / base;
        if fraction < 2.0 {
            base
        } else if fraction < 5.0 {
            2.0 * base
        } else {
            5.0 * base
        }
    }

    fn draw_grid(&self, painter: &Painter, viewport: &Rect) {
        // Compute a "nice step" based on the distance of the camera from the target.
        let spacing = self.grid_spacing();
        let half_size = self.grid_size / 2;

        // Align the center to the grid spacing to avoid snapping during pan/orbit.
        let center = math::align(self.target, spacing);
        let (min, max) = math::edges_from_point(center, half_size, spacing);

        // The origin (0,0,0) belongs to every reference plane if visible, otherwise to the local center
        let plane = origin(center, min, max);

        // Stroke for base reference plane (X-Z)
        let ground_stroke =
            egui::Stroke::new(0.5, Color32::from_rgba_premultiplied(45, 45, 55, 160));

        // Stroke for verticals planes (X-Y and Y-Z), a little bit lighten to improve visibility
        let vertical_stroke =
            egui::Stroke::new(0.5, Color32::from_rgba_premultiplied(35, 35, 45, 90));

        // 1. Draw the lines of the 3 planes (excluding the main axes that we will draw above)
        for i in -half_size..=half_size {
            let p = plane.map(|c| c + f64::from(i) * spacing);

            if i != 0 {
                let lines = [
                    // Plane X-Z (floor)
                    (
                        Vec3::new(min.x, plane.y, p.z),
                        Vec3::new(max.x, plane.y, p.z),
                        ground_stroke,
                    ),
                    (
                        Vec3::new(p.x, plane.y, min.z),
                        Vec3::new(p.x, plane.y, max.z),
                        ground_stroke,
                    ),
                    // Plane X-Y (frontal)
                    (
                        Vec3::new(min.x, p.y, plane.z),
                        Vec3::new(max.x, p.y, plane.z),
                        vertical_stroke,
                    ),
                    (
                        Vec3::new(p.x, min.y, plane.z),
                        Vec3::new(p.x, max.y, plane.z),
                        vertical_stroke,
                    ),
                    // Plane Y-Z (lateral)
                    (
                        Vec3::new(plane.x, min.y, p.z),
                        Vec3::new(plane.x, max.y, p.z),
                        vertical_stroke,
                    ),
                    (
                        Vec3::new(plane.x, p.y, min.z),
                        Vec3::new(plane.x, p.y, max.z),
                        vertical_stroke,
                    ),
                ];

                for (start, end, stroke) in lines {
                    self.draw_grid_line(painter, viewport, start, end, stroke);
                }

                // 2. Coordinate Labels on Axes Every 2 Intervals
                self.draw_coordinates(painter, viewport, plane, p, spacing);
            }
        }

        // 3. Draw Main Axes X, Y, Z Over Grid With Dedicated Colors (RGB = XYZ)
        self.draw_main_axes(painter, viewport, plane, min, max);
    }

    fn draw_coordinates(
        &self,
        painter: &Painter,
        viewport: &Rect,
        plane: Vec3,
        p: Vec3,
        spacing: f64,
    ) {
        let draw = |val: f64, pos: Vec3| {
            if (val / spacing).round() as i64 % 2 == 0 {
                self.draw_grid_label(painter, viewport, pos, Self::format_coord(val));
            }
        };

        draw(p.x, Vec3::new(p.x, plane.y, plane.z));
        draw(p.y, Vec3::new(plane.x, p.y, plane.z));
        draw(p.z, Vec3::new(plane.x, plane.y, p.z));
    }

    fn draw_main_axes(
        &self,
        painter: &Painter,
        viewport: &Rect,
        plane: Vec3,
        min: Vec3,
        max: Vec3,
    ) {
        // Main Cartesian axes X (RED), Y (GREEN), Z (BLUE) - standard RGB = XYZ
        let axis_x_stroke = egui::Stroke::new(1.5, Color32::from_rgb(220, 60, 60));
        let axis_y_stroke = egui::Stroke::new(1.5, Color32::from_rgb(60, 200, 60));
        let axis_z_stroke = egui::Stroke::new(1.5, Color32::from_rgb(60, 120, 240));

        // X Axis (red)
        self.draw_grid_line(
            painter,
            viewport,
            Vec3::new(min.x, plane.y, plane.z),
            Vec3::new(max.x, plane.y, plane.z),
            axis_x_stroke,
        );
        // Y Axis (green)
        self.draw_grid_line(
            painter,
            viewport,
            Vec3::new(plane.x, min.y, plane.z),
            Vec3::new(plane.x, max.y, plane.z),
            axis_y_stroke,
        );
        // Z Axis (blue)
        self.draw_grid_line(
            painter,
            viewport,
            Vec3::new(plane.x, plane.y, min.z),
            Vec3::new(plane.x, plane.y, max.z),
            axis_z_stroke,
        );

        // Label at the origin of the axes
        self.draw_grid_label(painter, viewport, plane, "0m".to_string());

        // Axes labels to the positive ends
        self.draw_grid_label(painter, viewport, plane, "+Y".to_string());
        self.draw_grid_label(painter, viewport, plane, "+Z".to_string());
    }

    fn format_coord(val: f64) -> String {
        if val.abs() < 1e-12 {
            "0m".to_string()
        } else if val.abs() >= 1e4 || val.abs() < 0.01 {
            format!("{:.1e}m", val)
        } else {
            format!("{:.0}m", val)
        }
    }

    fn draw_grid_label(&self, painter: &Painter, viewport: &Rect, point: Vec3, text: String) {
        let width = f64::from(viewport.width());
        let height = f64::from(viewport.height());

        let Some(screen) = self.camera.project(&point, width, height) else {
            return;
        };

        let pos = Pos2::new(
            viewport.left() + screen.x as f32,
            viewport.top() + screen.y as f32,
        );

        painter.text(
            pos,
            egui::Align2::CENTER_CENTER,
            text,
            egui::FontId::monospace(10.0),
            Color32::from_rgba_premultiplied(160, 160, 160, 180),
        );
    }

    fn draw_grid_line(
        &self,
        painter: &Painter,
        viewport: &Rect,
        start: Vec3,
        end: Vec3,
        stroke: egui::Stroke,
    ) {
        let width = f64::from(viewport.width());
        let height = f64::from(viewport.height());

        let Some((a, b)) = self
            .camera
            .project_line_segment(&start, &end, width, height)
        else {
            return;
        };

        let a = Pos2::new(viewport.left() + a.x as f32, viewport.top() + a.y as f32);
        let b = Pos2::new(viewport.left() + b.x as f32, viewport.top() + b.y as f32);

        painter.line_segment([a, b], stroke);
    }

    fn draw_origin(&self, painter: &Painter, viewport: &Rect) {
        if let Some(screen) = self.camera.project(
            &Vec3::zeros(),
            f64::from(viewport.width()),
            f64::from(viewport.height()),
        ) {
            let pos = Pos2::new(
                viewport.left() + screen.x as f32,
                viewport.top() + screen.y as f32,
            );

            painter.circle_filled(pos, 3.0, Color32::GREEN);
        }
    }

    fn pick_body(&self, world: &World, viewport: &Rect, pointer: Pos2) -> Option<usize> {
        let mut best: Option<(usize, f64, f32)> = None;

        for (index, body) in world.bodies().iter().enumerate() {
            let projected = self.draw_body(viewport, body);

            let Some((depth, center, radius, _)) = projected else {
                continue;
            };

            let distance = center.distance(pointer);
            if distance <= radius.max(8.0) {
                let replace = best
                    .as_ref()
                    .is_none_or(|(_, best_depth, _)| depth < *best_depth);

                if replace {
                    best = Some((index, depth, distance));
                }
            }
        }

        best.map(|(index, _, _)| index)
    }
}

impl Renderer for Renderer3D {
    fn setup(&mut self, ui: &mut egui::Ui, world: &World) -> Rect {
        let (rect, response) =
            ui.allocate_exact_size(ui.available_size(), egui::Sense::click_and_drag());
        let size = rect.size();

        if size.x <= 0.0 || size.y <= 0.0 {
            return rect;
        }

        if response.dragged() {
            let delta = response.drag_motion();

            self.yaw -= f64::from(delta.x) * 0.01;
            self.pitch =
                (self.pitch + f64::from(delta.y) * 0.01).clamp(-PI / 2.0 + 0.01, PI / 2.0 - 0.01);

            self.rebuild_camera(size.x, size.y);
        }

        let scroll = ui.input(|i| i.smooth_scroll_delta.y);

        if response.hovered() && scroll != 0.0 {
            let factor = 1.1_f64.powf(-f64::from(scroll) / 100.0);
            self.distance = (self.distance * factor).max(1e-6);
            self.rebuild_camera(size.x, size.y);
        }

        if response.clicked()
            && let Some(pointer) = response.interact_pointer_pos()
            && let Some(index) = self.pick_body(world, &rect, pointer)
        {
            self.focus_on(&world.bodies()[index], &rect);
        }

        self.camera
            .set_aspect_ratio(f64::from(size.x) / f64::from(size.y));

        rect
    }

    fn scale(&self) -> f32 {
        (1.0 / self.distance) as f32
    }

    fn fit(&mut self, viewport: &Rect, world: &World) {
        let mut min = Vec3::repeat(f64::INFINITY);
        let mut max = Vec3::repeat(f64::NEG_INFINITY);
        let mut found = false;

        for body in world.bodies() {
            let p = body.position();
            let r = body.radius().max(0.0);

            min.x = min.x.min(p.x - r);
            min.y = min.y.min(p.y - r);
            min.z = min.z.min(p.z - r);

            max.x = max.x.max(p.x + r);
            max.y = max.y.max(p.y + r);
            max.z = max.z.max(p.z + r);

            found = true;
        }

        if found {
            self.target = (min + max) * 0.5;

            let radius = ((max - min) * 0.5).norm().max(1e-6);
            let half_fov = Camera3D::NORMAL_FOV_Y * 0.5;

            self.distance = (radius / half_fov.sin()) * 1.2;
        } else {
            self.target = Vec3::zeros();
            self.distance = 10.0;
        }

        self.rebuild_camera(viewport.width(), viewport.height());
    }

    fn render(&self, world: &World, painter: &Painter, viewport: &Rect) {
        if viewport.width() <= 0.0 || viewport.height() <= 0.0 {
            return;
        }

        painter.rect_filled(*viewport, 0.0, Color32::from_rgb(12, 15, 24));

        let painter = painter.with_clip_rect(*viewport);

        self.draw_grid(&painter, viewport);
        self.draw_origin(&painter, viewport);

        let mut visible: Vec<_> = world
            .bodies()
            .iter()
            .filter_map(|body| self.draw_body(viewport, body))
            .collect();

        visible.sort_by(|a, b| b.0.total_cmp(&a.0));

        for (_, position, radius, color) in visible {
            painter.circle_filled(position, radius, color);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_coord() {
        assert_eq!(Renderer3D::format_coord(0.0), "0m");
        assert_eq!(Renderer3D::format_coord(5.0), "5m");
        assert_eq!(Renderer3D::format_coord(-10.0), "-10m");
        assert_eq!(Renderer3D::format_coord(1.5e11), "1.5e11m");
        assert_eq!(Renderer3D::format_coord(0.001), "1.0e-3m");
    }
}
