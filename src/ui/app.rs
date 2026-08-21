use crate::ui::camera_2d::Camera2D;
use crate::ui::format_duration;
use eframe::egui;
use eframe::egui::{Rect, RichText};
use gravity::simulator::Snapshot;
use gravity::simulator::simulation::{SimulationCommand, SimulationSnapshot, SimulationWarning};
use std::sync::mpsc;
use std::time::Duration;

#[derive(Debug)]
pub struct GravityApp {
    handle: mpsc::Sender<SimulationCommand>,
    snapshot: Snapshot,
    camera: Camera2D,
    reset_viewport: bool,
    simulation: SimulationSnapshot,
}

impl GravityApp {
    pub fn new(
        _cc: &eframe::CreationContext<'_>,
        handle: mpsc::Sender<SimulationCommand>,
        snapshot: Snapshot,
    ) -> Self {
        // Customize egui here with cc.egui_ctx.set_fonts and cc.egui_ctx.set_global_style.
        // Restore app state using cc.storage (requires the "persistence" feature).
        // Use the cc.gl (a glow::Context) to create graphics shaders and buffers that you can use
        // for e.g. egui::PaintCallback.
        let simulation = snapshot.read().unwrap().clone();
        Self {
            handle,
            snapshot,
            camera: Camera2D::default(),
            reset_viewport: true,
            simulation,
        }
    }

    fn world_viewport(&mut self, ui: &mut egui::Ui) -> Rect {
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

    fn draw_world(&mut self, ui: &mut egui::Ui, rect: Rect) {
        let world = &self.simulation.world;
        let painter = ui.painter_at(rect);
        painter.rect(
            rect,
            egui::CornerRadius::default(),
            egui::Color32::from_rgba_premultiplied(0, 0, 10, 0),
            egui::Stroke::new(2.0, egui::Color32::DARK_GRAY),
            egui::StrokeKind::Inside,
        );

        painter.circle_filled(
            self.camera.point_to_screen(&rect, 0.0, 0.0, 0.0),
            self.camera.length_to_screen(1.0),
            egui::Color32::GREEN,
        );

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

        for p in world.bodies() {
            let speed = p.speed();
            let (x, y, z) = p.position();
            let (vx, vy, _) = p.velocity_direction();
            let radius = p.radius();
            let position = self.camera.point_to_screen(&rect, x, y, z);
            let speed_line_length = radius + self.camera.length_to_world(30.0);
            let velocity = self.camera.point_to_screen(
                &rect,
                x + vx * speed_line_length,
                y + vy * speed_line_length,
                z,
            );
            let radius = self.camera.length_to_screen(p.radius());

            painter.circle_filled(position, radius, egui::Color32::WHITE);
            painter.text(
                position,
                egui::Align2::CENTER_CENTER,
                format!("{:.2e}kg", p.mass()),
                egui::FontId::monospace(8.0),
                egui::Color32::DARK_GRAY,
            );

            if p.speed().abs() > 0.001 {
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
}

impl eframe::App for GravityApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        if let Ok(snapshot) = self.snapshot.read() {
            self.simulation = snapshot.clone();
        }

        egui::Panel::top("top_panel").show(ui, |ui| {
            ui.horizontal_wrapped(|ui| {
                ui.label("Gravity Simulator");

                ui.add(egui::Label::new(format!(
                    "Scale: {:.4e} pixel/meter",
                    self.camera.scale()
                )));

                if ui.add(egui::Button::new("Fit View")).clicked() {
                    self.reset_viewport = true;
                }

                if ui
                    .add(egui::Checkbox::new(&mut self.simulation.running, "Running"))
                    .changed()
                {
                    println!("Simulation is now {}", self.simulation.running);
                    let command = if self.simulation.running {
                        SimulationCommand::Restart
                    } else {
                        SimulationCommand::Pause
                    };

                    if let Err(e) = self.handle.send(command) {
                        println!("Failed to send command: {}", e);
                    }
                }

                ui.separator();

                let time = Duration::from_secs_f32(self.simulation.time as f32);
                ui.label(RichText::new(format!("Time: {}", format_duration(time))).monospace());

                ui.separator();

                ui.label(
                    RichText::new(format!(
                        "Samples/Second: {:.2}",
                        self.simulation.samples_per_second
                    ))
                    .monospace(),
                );

                ui.separator();

                if self.simulation.warning != SimulationWarning::None {
                    ui.label(format!("Warning: {:?}", self.simulation.warning));
                    let _ = self.handle.send(SimulationCommand::ResetWarning);
                }
            });
        });

        egui::CentralPanel::default().show(ui, |ui| {
            let rect = self.world_viewport(ui);
            if self.reset_viewport {
                self.camera.fit(&rect, &self.simulation.world);
                self.reset_viewport = false;
            }

            self.draw_world(ui, rect);
        });

        // demand next frame
        ui.ctx().request_repaint();
    }
}
