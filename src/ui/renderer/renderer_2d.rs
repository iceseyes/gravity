use crate::ui::camera::camera_2d::Camera2D;
use crate::ui::renderer::Renderer;
use eframe::egui;
use eframe::egui::{Painter, Rect, Ui};
use gravity::simulator::{Body, World};

#[derive(Debug, Default)]
pub struct Renderer2D {
    camera: Camera2D,
}

impl Renderer2D {
    pub fn new(camera: Camera2D) -> Self {
        Self { camera }
    }

    fn draw_origin(&self, painter: &Painter, rect: &Rect) {
        painter.circle_filled(
            self.camera.point_to_screen(&rect, 0.0, 0.0, 0.0),
            self.camera.length_to_screen(1.0),
            egui::Color32::GREEN,
        );
    }

    fn draw_grid(&self, painter: &Painter, rect: &Rect) {
        Self::draw_box(painter, &rect);

        for index in 0..10 {
            let w = rect.left() + rect.width() * index as f32 / 10.0;
            let h = rect.top() + rect.height() * index as f32 / 10.0;
            let (x, y, _) = self.camera.point_to_world(&rect, egui::pos2(w, h));
            painter.text(
                egui::pos2(w, rect.bottom()),
                egui::Align2::CENTER_BOTTOM,
                format!("{:.2e}m", x),
                egui::FontId::monospace(12.0),
                egui::Color32::WHITE,
            );
            painter.text(
                egui::pos2(10.0, h),
                egui::Align2::LEFT_CENTER,
                format!("{:.2e}m", y),
                egui::FontId::monospace(12.0),
                egui::Color32::WHITE,
            );
            painter.line(
                vec![egui::pos2(w, 0.0), egui::pos2(w, rect.bottom())],
                egui::Stroke::new(0.5, egui::Color32::from_rgba_premultiplied(64, 64, 64, 0)),
            );
            painter.line(
                vec![egui::pos2(0.0, h), egui::pos2(rect.right(), h)],
                egui::Stroke::new(0.5, egui::Color32::from_rgba_premultiplied(64, 64, 64, 0)),
            );
        }
    }

    fn draw_box(painter: &Painter, rect: &Rect) {
        painter.rect(
            *rect,
            egui::CornerRadius::default(),
            egui::Color32::from_rgba_premultiplied(0, 0, 10, 0),
            egui::Stroke::new(2.0, egui::Color32::DARK_GRAY),
            egui::StrokeKind::Inside,
        );
    }

    fn draw_body(&self, body: &Body, painter: &Painter, rect: &Rect) {
        let speed = body.speed();
        let p = body.position();
        let v = body.velocity_direction();
        let radius = body.radius();
        let position = self
            .camera
            .point_to_screen(&rect, p.x as f32, p.y as f32, p.z as f32);
        let speed_line_length = radius + self.camera.length_to_world(30.0) as f64;
        let velocity = self.camera.point_to_screen(
            &rect,
            (p.x + v.x * speed_line_length) as f32,
            (p.y + v.y * speed_line_length) as f32,
            p.z as f32,
        );
        let radius = self.camera.length_to_screen(body.radius() as f32);

        painter.circle_filled(position, radius, egui::Color32::WHITE);
        painter.text(
            position,
            egui::Align2::CENTER_CENTER,
            format!("{:.2e}kg", body.mass()),
            egui::FontId::monospace(8.0),
            egui::Color32::DARK_GRAY,
        );

        if body.speed().abs() > 0.001 {
            let direction = vec![position, velocity];
            painter.line(
                direction,
                egui::Stroke::new(2.0, egui::Color32::from_rgba_premultiplied(128, 0, 0, 0)),
            );
            painter.text(
                velocity,
                egui::Align2::CENTER_CENTER,
                format!("{:.2e}m/s", speed),
                egui::FontId::monospace(8.0),
                egui::Color32::RED,
            );
        }
    }
}

impl Renderer for Renderer2D {
    fn setup(&mut self, ui: &mut Ui, _world: &World) -> Rect {
        let (rect, response) = ui.allocate_exact_size(ui.available_size(), egui::Sense::drag());

        if response.dragged() {
            let delta = response.drag_motion();

            self.camera.pan(delta);
        }

        let scroll = ui.input(|i| i.smooth_scroll_delta);
        if response.hovered() && scroll.y != 0.0 {
            let factor = 1.1_f32.powf(scroll.y / 100.0);
            if let Some(mouse_pos) = ui.ctx().pointer_latest_pos() {
                self.camera.zoom_at(&rect, factor, mouse_pos);
            } else {
                self.camera.set_scale(self.camera.scale() * factor);
            }
        }

        rect
    }

    fn scale(&self) -> f32 {
        self.camera.scale()
    }

    fn fit(&mut self, viewport: &Rect, world: &World) {
        self.camera.fit(viewport, world);
    }

    fn render(&self, world: &World, painter: &Painter, viewport: &Rect) {
        self.draw_grid(painter, viewport);
        self.draw_origin(painter, viewport);
        world
            .bodies()
            .iter()
            .for_each(|body| self.draw_body(body, painter, viewport));
    }
}
