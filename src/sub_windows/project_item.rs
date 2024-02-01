use eframe::egui;

use crate::panels;

pub struct ProjectItemWindow {
    pub visible: bool,
}

impl ProjectItemWindow {
    pub fn new() -> Self {
        Self { visible: false }
    }

    pub fn show(&mut self, ctx: &egui::Context, item: &mut panels::Item) {
        let mut visible = self.visible;
        egui::Window::new("Project Item")
            .open(&mut visible)
            .vscroll(true)
            .hscroll(true)
            .show(ctx, |ui| self.ui(ui, item));
        self.visible = self.visible && visible;
    }

    fn ui(&mut self, ui: &mut egui::Ui, item: &mut panels::Item) {
        ui.set_min_width(250.0);
        ui.label(item.name.clone());
    }
}
