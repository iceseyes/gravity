extern crate core;

pub mod ui;

use anyhow::Context;
use gravity::simulator;
use ui::app::GravityApp;

fn main() -> anyhow::Result<()> {
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "Gravity Simulator",
        native_options,
        Box::new(|cc| Ok(Box::new(GravityApp::new(cc, simulator::orbit())))),
    )
    .context("Failed to run eframe")
}
