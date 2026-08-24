use crate::ui::app::AppAction;
use crate::ui::app::AppAction::ResetViewport;
use crate::ui::camera_2d::Camera2D;
use crate::ui::format_duration;
use eframe::egui;
use eframe::egui::RichText;
use gravity::simulator::simulation::{SimulationCommand, SimulationSnapshot};
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

        handle_scale(ui, camera, &mut actions);
        ui.separator();

        handle_simulation_state(ui, simulation, &mut actions);
        ui.separator();

        show_simulation_time(ui, simulation);
        ui.separator();

        show_samples_per_second(ui, simulation);
        ui.separator();
    });

    Ok(actions)
}

fn handle_scale(ui: &mut egui::Ui, camera: &Camera2D, actions: &mut Vec<AppAction>) {
    ui.add(egui::Label::new(format!(
        "Scale: {:.4e} pixel/meter",
        camera.scale()
    )));

    if ui.add(egui::Button::new("Fit View")).clicked() {
        actions.push(ResetViewport)
    }
}

fn handle_simulation_state(
    ui: &mut egui::Ui,
    simulation: &mut SimulationSnapshot,
    actions: &mut Vec<AppAction>,
) {
    if ui
        .add(egui::Checkbox::new(&mut simulation.running, "Running"))
        .changed()
    {
        println!("Simulation is now {}", simulation.running);
        let command = if simulation.running {
            SimulationCommand::Resume
        } else {
            SimulationCommand::Pause
        };

        actions.push(AppAction::SendSimulationCommand(command));
    }
}
fn show_simulation_time(ui: &mut egui::Ui, simulation: &SimulationSnapshot) {
    let time = Duration::from_secs_f32(simulation.time as f32);
    ui.label(RichText::new(format!("Time: {}", format_duration(time))).monospace());
}

fn show_samples_per_second(ui: &mut egui::Ui, simulation: &SimulationSnapshot) {
    ui.label(
        RichText::new(format!(
            "Samples/Second: {:.2}",
            simulation.samples_per_second
        ))
        .monospace(),
    );
}
