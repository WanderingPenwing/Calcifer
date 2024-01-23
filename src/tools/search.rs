use eframe::egui;
use crate::tools::{View, Demo, Tab, TabNumber};
use std::{cmp::min};
use crate::RED;


enum Action {
	Next,
	Previous,
	Replace,
	Update,
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
	replace_text: String,

	pub initialized: bool,

	across_documents: bool,

	results: Vec<Selection>,
	current_result: usize,

	pub tab_selected: bool,
	pub result_selected: bool,
}


impl Default for SearchWindow {
    fn default() -> Self {
        Self {
            search_text: "".into(),
			searched_text: "".into(),
			replace_text: "".into(),
			
			initialized: false,

			across_documents: false,

			results: vec![],
			current_result: 0,

			tab_selected: true,
			result_selected: true,
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
			
			let response = ui.add(egui::TextEdit::singleline(search_text).desired_width(120.0).lock_focus(true));
			if response.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter)) {
				action = Action::Update;
			}
			if !self.initialized {
				response.request_focus();
				self.initialized = true;
			}
			
			if ui.add(egui::Button::new("Update")).clicked() {
				action = Action::Update;
			}

			if ui.add(egui::Button::new("<")).clicked() {
				action = Action::Previous;
			}

			if self.search_text == self.searched_text && self.search_text.len() > 0 && self.results.len() == 0 {
				ui.colored_label(RED, " 0/0 ");
			} else {
				ui.label(format!(" {}/{} ", min(self.current_result + 1, self.results.len()), self.results.len()));
			}

			if ui.add(egui::Button::new(">")).clicked() {
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
			Action::Update => self.search(tabs, selected_tab),
			Action::Next => self.find_next(tabs, selected_tab),
			Action::Previous => self.find_previous(tabs, selected_tab),
			Action::Replace => self.replace(tabs, selected_tab),
			Action::None => (),
		}
    }
}

impl SearchWindow {
	pub fn get_tab(&self) -> TabNumber {
		self.results[self.current_result].tab.clone()
	}

	pub fn get_cursor_start(&self) -> usize {
		self.results[self.current_result].start.clone()
	}

	pub fn get_cursor_end(&self) -> usize {
		self.results[self.current_result].end.clone()
	}

	fn search(&mut self, tabs: &Vec<Tab>, selected_tab: &TabNumber) {
		if self.search_text.len() == 0 {
			return
		}

		let mut search_results: Vec<Selection> = vec![];

		if self.across_documents {
			for (index, tab) in tabs.iter().enumerate() {
				search_results.extend(self.match_text(tab.code.clone(), TabNumber::from_index(index)));
			}
		} else {
			search_results.extend(self.match_text(tabs[selected_tab.to_index()].code.clone(), selected_tab.clone()));
		}
		
		self.searched_text = self.search_text.clone();
		self.results = search_results.clone();
		
		self.current_result = 0;
		self.result_selected = false;
		self.tab_selected = false;
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
		if self.searched_text != self.search_text {
			self.search(tabs, selected_tab);
		} else if self.results.len() > 1 {
			self.current_result = (self.current_result.clone() + 1) % self.results.len();
			self.result_selected = false;
			if self.results[self.current_result].tab != *selected_tab {
				self.tab_selected = false;
			}
		}
	}

	fn find_previous(&mut self, tabs: &Vec<Tab>, selected_tab: &TabNumber) {
		if self.searched_text != self.search_text {
			self.search(tabs, selected_tab);
		} else {
			self.current_result = self.current_result.checked_sub(1).unwrap_or(self.results.len() - 1);
			self.result_selected = false;
			if self.results[self.current_result].tab != *selected_tab {
				self.tab_selected = false;
			}
		}
	}

	fn replace(&mut self, tabs: &Vec<Tab>, selected_tab: &TabNumber) {
		println!("Searched to replace {} with {}, tab lang : {} ", &self.search_text, &self.replace_text, tabs[selected_tab.to_index()].language);
	}
}