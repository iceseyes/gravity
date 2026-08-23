use crate::ui::camera_2d::Camera2D;
use eframe::egui;
use gravity::simulator::Body;

pub(crate) fn draw_body(body: &Body, camera: &Camera2D, painter: &egui::Painter, rect: egui::Rect) {
    let speed = body.speed();
    let p = body.position();
    let v = body.velocity_direction();
    let radius = body.radius();
    let position = camera.point_to_screen(&rect, p.x as f32, p.y as f32, p.z as f32);
    let speed_line_length = radius + camera.length_to_world(30.0) as f64;
    let velocity = camera.point_to_screen(
        &rect,
        (p.x + v.x * speed_line_length) as f32,
        (p.y + v.y * speed_line_length) as f32,
        p.z as f32,
    );
    let radius = camera.length_to_screen(body.radius() as f32);

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
