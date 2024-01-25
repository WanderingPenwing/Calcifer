use std::{cmp::min};
use eframe::egui;

use crate::RED;
use crate::tools::{tabs::Tab, tabs::TabNumber};


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
			tab: TabNumber::from_index(0),
			start: 0,
			end: 0,
		}
	}
}


pub struct SearchWindow {
	pub visible: bool,
	
	search_text: String,
	searched_text: String,
	replace_text: String,

	pub initialized: bool,

	across_documents: bool,

	results: Vec<Selection>,
	current_result: usize,

	pub result_selected: bool,
	
	row_height: f32,
}


impl Default for SearchWindow {
	fn default() -> Self {
		Self {
			visible: false,
			
			search_text: "".into(),
			searched_text: "".into(),
			replace_text: "".into(),
			
			initialized: false,

			across_documents: false,

			results: vec![],
			current_result: 0,

			result_selected: true,
			
			row_height: 0.0,
		}
	}
}


impl SearchWindow {
	pub fn show(&mut self, ctx: &egui::Context, tabs: &mut Vec<Tab>, selected_tab: &mut TabNumber) {
		let mut visible = self.visible.clone();
		egui::Window::new("Search")
			.open(&mut visible) //I want it to be able to change its visibility (if user close manually)
			.vscroll(true)
			.hscroll(true)
			.show(ctx, |ui| self.ui(ui, tabs, selected_tab)); //but I want to edit the rest of the parameters and maybe close automatically
		self.visible = self.visible.clone() && visible;
	}
	
	
	fn ui(&mut self, ui: &mut egui::Ui, tabs: &mut Vec<Tab>, selected_tab: &mut TabNumber) {
		ui.set_min_width(250.0);
		
		let font_id = egui::TextStyle::Body.resolve(ui.style());
		self.row_height = ui.fonts(|f| f.row_height(&font_id)); //+ ui.spacing().item_spacing.y;

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
			Action::Next => self.find_result(tabs, selected_tab, 1),
			Action::Previous => self.find_result(tabs, selected_tab, -1),
			Action::Replace => self.replace(tabs, selected_tab),
			Action::None => (),
		}
	}
	
	
	pub fn get_cursor_start(&self) -> usize {
		self.results[self.current_result].start.clone()
	}


	pub fn get_cursor_end(&self) -> usize {
		self.results[self.current_result].end.clone()
	}
	

	fn search(&mut self, tabs: &mut Vec<Tab>, selected_tab: &mut TabNumber) {
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
		if self.results.len() > 0 {
			self.find_result(tabs, selected_tab, 0);
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


	fn find_result(&mut self, tabs: &mut Vec<Tab>, selected_tab: &mut TabNumber, direction: i32) {
		if self.searched_text != self.search_text {
			self.search(tabs, &mut *selected_tab);
		} else if self.results.len() > 0 {
			self.current_result = (self.current_result as i32 + direction + self.results.len() as i32) as usize % self.results.len();
			self.result_selected = false;
			*selected_tab = self.results[self.current_result].tab.clone();
			
			let target = self.results[self.current_result].start;
			let code = tabs[selected_tab.to_index()].code.clone();
			let (upstream, _downstream) = code.split_at(target);
			let row = upstream.match_indices(&"\n".to_string()).collect::<Vec<_>>().len();
			tabs[selected_tab.to_index()].scroll_offset = self.row_height * row.saturating_sub(5) as f32;
		}
	}


	fn replace(&mut self, tabs: &mut Vec<Tab>, selected_tab: &mut TabNumber) {
		if self.searched_text != self.search_text {
			self.search(tabs, &mut *selected_tab);
		}
		
		let mut done : Vec<TabNumber> = vec![];
		for element in &self.results {
			if done.contains(&element.tab) {
				continue;
			}
			tabs[element.tab.to_index()].code = tabs[element.tab.to_index()].code.replace(&self.search_text, &self.replace_text);
			tabs[element.tab.to_index()].saved = false;
			done.push(element.tab.clone())
		}
	}
}