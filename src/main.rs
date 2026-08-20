extern crate core;

pub mod ui;

use anyhow::Context;
use gravity::simulator;
use ui::app::GravityApp;

fn main() -> anyhow::Result<()> {
    let native_options = eframe::NativeOptions::default();
    let (simulator, snapshot) = simulator::random(10000);

    let ris = eframe::run_native(
        "Gravity Simulator",
        native_options,
        Box::new(|cc| Ok(Box::new(GravityApp::new(cc, snapshot)))),
    )
    .context("Failed to run eframe");

    ris
}
