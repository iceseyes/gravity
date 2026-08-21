use crate::ui::app::AppAction;
use crate::ui::app::AppAction::ResetViewport;
use crate::ui::camera_2d::Camera2D;
use crate::ui::format_duration;
use eframe::egui;
use eframe::egui::RichText;
use gravity::simulator::simulation::{SimulationCommand, SimulationSnapshot, SimulationWarning};
use std::time::Duration;

pub(crate) fn draw_control_panel(
    ui: &mut egui::Ui,
    camera: &Camera2D,
    simulation: &mut SimulationSnapshot,
) -> anyhow::Result<Vec<AppAction>> {
    let mut actions = Vec::new();

    ui.horizontal_wrapped(|ui| {
        ui.label("Gravity Simulator");
        ui.separator();

        ui.add(egui::Label::new(format!(
            "Scale: {:.4e} pixel/meter",
            camera.scale()
        )));

        if ui.add(egui::Button::new("Fit View")).clicked() {
            actions.push(ResetViewport)
        }

        if ui
            .add(egui::Checkbox::new(&mut simulation.running, "Running"))
            .changed()
        {
            println!("Simulation is now {}", simulation.running);
            let command = if simulation.running {
                SimulationCommand::Restart
            } else {
                SimulationCommand::Pause
            };

            actions.push(AppAction::SendSimulationCommand(command));
        }

        ui.separator();

        let time = Duration::from_secs_f32(simulation.time as f32);
        ui.label(RichText::new(format!("Time: {}", format_duration(time))).monospace());

        ui.separator();

        ui.label(
            RichText::new(format!(
                "Samples/Second: {:.2}",
                simulation.samples_per_second
            ))
            .monospace(),
        );

        ui.separator();

        if simulation.warning != SimulationWarning::None {
            ui.label(format!("Warning: {:?}", simulation.warning));
            actions.push(AppAction::SendSimulationCommand(
                SimulationCommand::ResetWarning,
            ));
        }
    });

    Ok(actions)
}
