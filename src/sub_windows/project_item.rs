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
		egui::Window::new("Item")
			.open(&mut visible)
			.vscroll(true)
			.hscroll(true)
			.show(ctx, |ui| self.ui(ui, item));
		self.visible = self.visible && visible;
	}

	fn ui(&mut self, ui: &mut egui::Ui, item: &mut panels::Item) {
		ui.set_min_width(250.0);
		ui.set_min_height(250.0);
		ui.add(egui::TextEdit::singleline(&mut item.name).desired_width(f32::INFINITY));
		ui.separator();
		ui.add_sized(ui.available_size(), egui::TextEdit::multiline(&mut item.description));
	}
}
