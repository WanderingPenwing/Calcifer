use eframe::egui;
use crate::TIME_LABELS;


pub struct ProfilerWindow {
	pub visible: bool,
}


impl ProfilerWindow {
	pub fn new() -> Self {
		Self {
			visible: false,
		}
	}
	
	
	pub fn show(&mut self, ctx: &egui::Context, time_watch: Vec<f32>) {
		let mut visible = self.visible.clone();
		egui::Window::new("Profiler")
			.open(&mut visible) //I want it to be able to change its visibility (if user close manually)
			.vscroll(true)
			.hscroll(true)
			.show(ctx, |ui| self.ui(ui, time_watch)); //but I want to edit the rest of the parameters and maybe close automatically
		self.visible = self.visible.clone() && visible;
	}
	
	
	fn ui(&mut self, ui: &mut egui::Ui, time_watch: Vec<f32>) {
		ui.set_min_width(100.0);
		
		for (index, entry) in TIME_LABELS.iter().enumerate() {
			ui.label(format!("{} : {:.1} ms", entry, time_watch[index]));
		}
		ui.separator();
		ui.label(&format!("total : {:.1} ms", time_watch.clone().iter().sum::<f32>()));
	}
}