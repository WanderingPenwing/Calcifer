use eframe::egui;


pub struct ConfirmWindow {
	pub visible: bool,
	pub proceed: bool,
	pub prompt: String,
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
	
	
	pub fn show(&mut self, ctx: &egui::Context) {
		let mut visible = self.visible.clone();
		egui::Window::new(self.id.clone())
			.open(&mut visible)
			.vscroll(true)
			.hscroll(true)
			.show(ctx, |ui| self.ui(ui));
		self.visible = self.visible.clone() && visible;
	}
	
	
	fn ui(&mut self, ui: &mut egui::Ui) {
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