use eframe::egui;

use crate::panels;

pub struct ProjectItemWindow {
	pub visible: bool,
}

impl ProjectItemWindow {
	pub fn new() -> Self {
		Self { visible: false }
	}

	pub fn show(&mut self, ctx: &egui::Context, item: &mut panels::Item) -> bool {
		let mut visible = self.visible;
		let maybe_response = egui::Window::new("Item")
			.open(&mut visible)
			.vscroll(true)
			.hscroll(true)
			.show(ctx, |ui| self.ui(ui, item));
		self.visible = self.visible && visible;
		
		if let Some(response) = maybe_response {
			if let Some(delete_option) = response.inner {
				return delete_option
			}
		}
		return false
	}

	fn ui(&mut self, ui: &mut egui::Ui, item: &mut panels::Item) -> bool {
		let mut delete_item = false;
		ui.set_min_width(250.0);
		ui.set_min_height(250.0);
		ui.with_layout(egui::Layout::right_to_left(egui::Align::TOP), |ui| {
			if ui.add(egui::Button::new("delete")).clicked() {
				delete_item = true;
			}
			ui.add(egui::TextEdit::singleline(&mut item.name).desired_width(f32::INFINITY));
		});
		ui.separator();
		ui.add_sized(
			ui.available_size(),
			egui::TextEdit::multiline(&mut item.description),
		);
		return delete_item.clone()
	}
}
