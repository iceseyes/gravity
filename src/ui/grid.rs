use crate::ui::camera_2d::Camera2D;
use eframe::egui;

pub(crate) fn draw_grid(camera: &Camera2D, painter: &egui::Painter, rect: egui::Rect) {
    draw_box(painter, rect);

    for index in 0..10 {
        let w = rect.left() + rect.width() * index as f32 / 10.0;
        let h = rect.top() + rect.height() * index as f32 / 10.0;
        let (x, y, _) = camera.point_to_world(&rect, egui::pos2(w, h));
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

fn draw_box(painter: &egui::Painter, rect: egui::Rect) {
    painter.rect(
        rect,
        egui::CornerRadius::default(),
        egui::Color32::from_rgba_premultiplied(0, 0, 10, 0),
        egui::Stroke::new(2.0, egui::Color32::DARK_GRAY),
        egui::StrokeKind::Inside,
    );
}
