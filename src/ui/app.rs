use crate::ui::camera_2d::Camera2D;
use crate::ui::control_panel::draw_control_panel;
use crate::ui::world::{draw_world, setup_viewport};
use eframe::egui;
use gravity::simulator::Snapshot;
use gravity::simulator::simulation::{SimulationCommand, SimulationSnapshot};
use std::sync::mpsc;
use std::time::Instant;

pub(crate) enum AppAction {
    ResetViewport,
    SendSimulationCommand(SimulationCommand),
}

#[derive(Debug)]
pub struct GravityApp {
    handle: mpsc::Sender<SimulationCommand>,
    snapshot: Snapshot,
    camera: Camera2D,
    reset_viewport: bool,
    simulation: SimulationSnapshot,
    last_update: Instant,
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
            last_update: Instant::now(),
        }
    }

    fn handle(&mut self, command: AppAction) {
        match command {
            AppAction::ResetViewport => self.reset_viewport = true,
            AppAction::SendSimulationCommand(command) => {
                if let Err(e) = self.handle.send(command) {
                    eprintln!("{}", e);
                }
            }
        }
    }
}

impl eframe::App for GravityApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        let fps = 1.0 / self.last_update.elapsed().as_secs_f32();
        self.last_update = Instant::now();

        if let Ok(snapshot) = self.snapshot.read() {
            self.simulation = snapshot.clone();
        }

        if let Err(e) = self.handle.send(SimulationCommand::TakeSnapshot) {
            eprintln!("{}", e);
        }

        egui::Panel::top("control_panel").show(ui, |ui| {
            match draw_control_panel(ui, &self.camera, &mut self.simulation) {
                Ok(mut actions) => actions.drain(..).for_each(|action| self.handle(action)),
                Err(e) => {
                    eprintln!("ERROR: {}", e);
                }
            }
        });

        egui::Panel::bottom("status_panel").show(ui, |ui| {
            ui.label(format!("FPS: {:.2}", fps));
        });

        egui::CentralPanel::default().show(ui, |ui| {
            let rect = setup_viewport(&mut self.camera, ui);
            if self.reset_viewport {
                self.camera.fit(&rect, &self.simulation.world);
                self.reset_viewport = false;
            }

            draw_world(&self.simulation.world, &self.camera, ui.painter(), rect);
        });

        // demand next frame
        ui.ctx().request_repaint();
    }
}
