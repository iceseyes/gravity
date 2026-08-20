extern crate core;

pub mod physics;
pub mod simulator;
pub mod ui;

use anyhow::Context;
use ui::app::GravityApp;

fn main() -> anyhow::Result<()> {
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "Gravity Simulator",
        native_options,
        Box::new(|cc| Ok(Box::new(GravityApp::new(cc)))),
    )
    .context("Failed to run eframe")
}
