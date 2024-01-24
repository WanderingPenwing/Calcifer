use eframe::egui;
use crate::tools::{Tab, TabNumber};


pub struct ConfirmWindow {
	pub visible: bool,
	pub proceed: bool,
	prompt: String,
	id: String,
}


impl ConfirmWindow {
	pub fn new(prompt: &str, id: &str) -> Self {
		Self {
			visible: false,
			proceed: false,
			prompt: prompt.to_string(),
			id: id.to_string(),
		}
	}
	
	
	pub fn show(&mut self, ctx: &egui::Context, tabs: &mut Vec<Tab>, selected_tab: &mut TabNumber) {
		let mut visible = self.visible.clone();
		egui::Window::new(self.id.clone())
			.open(&mut visible) //I want it to be able to change its visibility (if user close manually)
			.vscroll(true)
			.hscroll(true)
			.show(ctx, |ui| self.ui(ui, tabs, selected_tab)); //but I want to edit the rest of the parameters and maybe close automatically
		self.visible = self.visible.clone() && visible;
	}
	
	
	fn ui(&mut self, ui: &mut egui::Ui, _tabs: &mut Vec<Tab>, _selected_tab: &mut TabNumber) {
		ui.set_min_width(250.0);
		ui.label(self.prompt.clone());
		ui.vertical_centered(|ui| {
			if ui.add(egui::Button::new("Yes")).clicked() {
				self.proceed = true;
			}
			
			if ui.add(egui::Button::new("No")).clicked() {
				self.visible = false;
			}
		});
	}
	
	pub fn ask(&mut self) {
		self.visible = true;
		self.proceed = false;
	}
	
	
	pub fn close(&mut self) {
		self.visible = false;
		self.proceed = false;
	}
}