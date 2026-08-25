extern crate core;

pub mod ui;

use anyhow::Context;
use gravity::simulator;
use gravity::simulator::integrator::velocity_verlet::VelocityVerlet;
use gravity::simulator::simulation::SimulationCommand;
use ui::app::GravityApp;

fn main() -> anyhow::Result<()> {
    let native_options = eframe::NativeOptions::default();
    //let (simulator, handle, snapshot) = simulator::random(1000);
    //let (simulator, handle, snapshot) = simulator::orbit();
    let (simulator, handle, snapshot) = simulator::sol(VelocityVerlet);

    let ris = eframe::run_native(
        "Gravity Simulator",
        native_options,
        Box::new(|cc| Ok(Box::new(GravityApp::new(cc, handle.clone(), snapshot)))),
    )
    .context("Failed to run eframe");

    let _ = handle.send(SimulationCommand::Quit);
    simulator.join().unwrap();

    ris
}
