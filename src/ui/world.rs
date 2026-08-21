use crate::ui::body::draw_body;
use crate::ui::camera_2d::Camera2D;
use crate::ui::grid::draw_grid;
use eframe::egui;
use gravity::simulator::World;

pub(crate) fn setup_viewport(camera: &mut Camera2D, ui: &mut egui::Ui) -> egui::Rect {
    let (rect, response) = ui.allocate_exact_size(ui.available_size(), egui::Sense::drag());

    if response.dragged() {
        let delta = response.drag_motion();

        camera.pan(delta);
    }

    let scroll = ui.input(|i| i.smooth_scroll_delta);
    if response.hovered() && scroll.y != 0.0 {
        let factor = 1.1_f32.powf(scroll.y / 100.0);
        if let Some(mouse_pos) = ui.ctx().pointer_latest_pos() {
            camera.zoom_at(&rect, factor, mouse_pos);
        } else {
            camera.set_scale(camera.scale() * factor);
        }
    }

    rect
}

pub(crate) fn draw_world(
    world: &World,
    camera: &Camera2D,
    painter: &egui::Painter,
    rect: egui::Rect,
) {
    draw_grid(camera, painter, rect);
    draw_origin(camera, painter, rect);
    world
        .bodies()
        .iter()
        .for_each(|body| draw_body(body, camera, painter, rect));
}

fn draw_origin(camera: &Camera2D, painter: &egui::Painter, rect: egui::Rect) {
    painter.circle_filled(
        camera.point_to_screen(&rect, 0.0, 0.0, 0.0),
        camera.length_to_screen(1.0),
        egui::Color32::GREEN,
    );
}
