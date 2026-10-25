use eframe::egui;
use gravity::simulator::World;
use std::fmt::Debug;

pub mod renderer_2d;
pub mod renderer_3d;

pub trait Renderer: Debug {
    fn setup(&mut self, ui: &mut egui::Ui, world: &World) -> egui::Rect;
    fn scale(&self) -> f32;
    fn fit(&mut self, viewport: &egui::Rect, world: &World);
    fn render(&self, world: &World, painter: &egui::Painter, viewport: &egui::Rect);
}
