use eframe::egui;
use crate::tools::{View, Demo, Tab, TabNumber};


pub struct Selection {
	pub start: usize,
	pub end: usize,
}


impl Default for Selection {
	fn default() -> Self {
		Self {
			start: 0,
			end: 0,
		}
	}
}


pub struct SearchWindow {
    search_text: String,
	result: Selection,
	across_documents: bool,
	replace_text: String,
}


impl Default for SearchWindow {
    fn default() -> Self {
        Self {
            search_text: "".into(),
			result: Selection::default(),
			across_documents: false,
			replace_text: "".into(),
        }
    }
}


impl Demo for SearchWindow {
    fn name(&self) -> &str { //'static
        "Search"
    }

    fn show(&mut self, ctx: &egui::Context, open: &mut bool, tabs: &mut Vec<Tab>, selected_tab: &mut TabNumber) {
        egui::Window::new(self.name())
            .open(open)
            .vscroll(true)
            .hscroll(true)
            .show(ctx, |ui| self.ui(ui));
    }
}


impl View for SearchWindow {
    fn ui(&mut self, ui: &mut egui::Ui) {
        ui.set_min_width(250.0);

		ui.horizontal(|ui| {
			let Self { search_text, .. } = self;
			ui.add(egui::TextEdit::singleline(search_text).desired_width(120.0).lock_focus(true));

			if ui.add(egui::Button::new("<")).clicked() {
				self.find_previous();
			}

			if ui.add(egui::Button::new(">")).clicked() {
				self.find_next();
			}
		});
		
		ui.checkbox(&mut self.across_documents, "Across documents");

        egui::CollapsingHeader::new("Replace")
            .default_open(false)
            .show(ui, |ui| {
				ui.horizontal(|ui| {
					let Self { replace_text, .. } = self;
					ui.add(egui::TextEdit::singleline(replace_text).desired_width(120.0).lock_focus(true));
					if ui.add(egui::Button::new("Replace")).clicked() {
						self.replace_text();
					}
				});
            });
    }
}

impl SearchWindow {
	fn find_previous(&self) {
		println!("Searched for previous");
	}

	fn find_next(&self) {
		println!("Searched for next");
	}

	fn replace_text(&self) {
		println!("Searched to replace {} with {}", &self.search_text, &self.replace_text);
	}
}