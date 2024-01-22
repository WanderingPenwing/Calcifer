use eframe::egui;
use crate::tools::{View, Demo, Tab, TabNumber};


enum Action {
	Next,
	Previous,
	Replace,
	None,
}


#[derive(Clone)]
pub struct Selection {
	pub tab: TabNumber,
	pub start: usize,
	pub end: usize,
}


impl Default for Selection {
	fn default() -> Self {
		Self {
			tab: TabNumber::Zero,
			start: 0,
			end: 0,
		}
	}
}


pub struct SearchWindow {
    search_text: String,
	searched_text: String,
	results: Vec<Selection>,
	current_result: Selection,
	across_documents: bool,
	replace_text: String,
}


impl Default for SearchWindow {
    fn default() -> Self {
        Self {
            search_text: "".into(),
			searched_text: "".into(),
			results: vec![],
			current_result: Selection::default(),
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
            .show(ctx, |ui| self.ui(ui, tabs, selected_tab));
    }
}


impl View for SearchWindow {
    fn ui(&mut self, ui: &mut egui::Ui, tabs: &mut Vec<Tab>, selected_tab: &mut TabNumber) {
        ui.set_min_width(250.0);

		let mut action : Action = Action::None;

		ui.horizontal(|ui| {
			let Self { search_text, .. } = self;
			ui.add(egui::TextEdit::singleline(search_text).desired_width(120.0).lock_focus(true));

			ui.label(format!("{} ", self.results.len()));

			if ui.add(egui::Button::new("<")).clicked() {
				action = Action::Previous;
			} else if ui.add(egui::Button::new(">")).clicked() {
				action = Action::Next;
			}
		});

		let previous_bool_state = self.across_documents.clone();
		ui.checkbox(&mut self.across_documents, "Across documents");
		if previous_bool_state != self.across_documents {
			self.searched_text = "".into();
		}

        egui::CollapsingHeader::new("Replace")
            .default_open(false)
            .show(ui, |ui| {
				ui.horizontal(|ui| {
					let Self { replace_text, .. } = self;
					ui.add(egui::TextEdit::singleline(replace_text).desired_width(120.0).lock_focus(true));
					if ui.add(egui::Button::new("Replace")).clicked() {
						action = Action::Replace;
					}
				});
            });

		match action {
			Action::Next => self.find_next(tabs, selected_tab),
			Action::Previous => self.find_previous(tabs, selected_tab),
			Action::Replace => self.replace(tabs, selected_tab),
			Action::None => (),
		}
    }
}

impl SearchWindow {
	fn search(&mut self, tabs: &Vec<Tab>, selected_tab: &TabNumber) {
		let mut search_results: Vec<Selection> = vec![];

		if self.across_documents {
			for (index, tab) in tabs.iter().enumerate() {
				search_results.extend(self.match_text(tab.code.clone(), TabNumber::from_index(index)));
			}
		} else {
			search_results.extend(self.match_text(tabs[TabNumber::to_index(&selected_tab.clone())].code.clone(), selected_tab.clone()));
		}
		
		println!("Found {} results", search_results.len());
		self.searched_text = self.search_text.clone();
		self.results = search_results.clone();
		
		if self.results.len() > 0 {
			self.current_result = self.results[0].clone();
		}
	}

	fn match_text(&self, tab_text: String, tab_number: TabNumber) -> Vec<Selection> {
		let matches = tab_text.match_indices(&self.search_text.clone()).map(|(i, _)| Selection {
					tab : tab_number.clone(),
					start: i,
					end: i + self.search_text.len(),
				}).collect();

		matches
	}

	fn find_next(&mut self, tabs: &Vec<Tab>, selected_tab: &TabNumber) {
		if self.searched_text != self.search_text && self.search_text.len() > 1 {
			self.search(tabs, selected_tab);
		} else {
			println!("just need to get next result");
		}
	}

	fn find_previous(&mut self, tabs: &Vec<Tab>, selected_tab: &TabNumber) {
		if self.searched_text != self.search_text && self.search_text.len() > 1 {
			self.search(tabs, selected_tab);
		} else {
			println!("just need to get next result");
		}
	}

	fn replace(&mut self, tabs: &Vec<Tab>, selected_tab: &TabNumber) {
		println!("Searched to replace {} with {}", &self.search_text, &self.replace_text);
	}
}