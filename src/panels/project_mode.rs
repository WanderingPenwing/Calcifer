use eframe::egui;
use std::sync::atomic::{AtomicUsize, Ordering};

use crate::MAX_PROJECT_COLUMNS;
use crate::core::hex_str_to_color;
use crate::editor::ColorTheme;


#[derive(Clone)]
pub struct Project {
	categories: Vec<Category>,
	selected_item: Option<[usize; 2]>,
}

impl Project {
	pub fn new() -> Self {
		Self {
			categories: vec![Category::create()],
			selected_item: None,
		}
	}
	
	fn add_category(&mut self) {
		let last = self.categories.len() - 1;
		self.categories[last].initialize();
		if self.categories.len() < MAX_PROJECT_COLUMNS {
			self.categories.push(Category::create());
		}
	}
	
	fn delete_category(&mut self, index: usize) {
		self.categories.remove(index);
		let last = self.categories.len() - 1;
		if self.categories[last].name != "+" {
			self.categories.push(Category::create());
		}
	}
}

#[derive(Clone)]
struct Category {
	name: String,
	content: Vec<Item>,
}

impl Category {
	fn create() -> Self {
		Self {
			name: "+".into(),
			content: vec![],
		}
	}
	
	fn initialize(&mut self) {
		self.name = "untitled".into();
	}
}


#[derive(Clone, Hash)]
struct Item {
	name: String,
	description: String,
	id: usize,
}

impl Item {
	fn new(name: &str) -> Self {
		Self {
			name: name.to_string(),
			description: "".to_string(),
			id: get_id(),
		}
	}
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Location {
	category: usize,
	row: usize,
}


fn get_id() -> usize {
	static COUNTER:AtomicUsize = AtomicUsize::new(1);
	COUNTER.fetch_add(1, Ordering::Relaxed)
}

pub fn draw_project(ui: &mut egui::Ui, theme: ColorTheme, project: &mut Project) {
	ui.columns(MAX_PROJECT_COLUMNS, |uis| {
		for (category_index, category) in project.categories.clone().into_iter().enumerate() {
			let ui = &mut uis[category_index];

			if category.name == "+" {
				if ui.add(egui::Button::new("+")).clicked() {
					project.add_category();
				}
			} else {
				let response = ui.add(egui::TextEdit::singleline(&mut project.categories[category_index].name).desired_width(f32::INFINITY));
				if response.lost_focus() && project.categories[category_index].name.is_empty() {
					project.delete_category(category_index);
				}
			}
			
			for (item_index, item) in category.content.iter().enumerate() {
				ui.with_layout(egui::Layout::right_to_left(egui::Align::TOP), |ui| {
					ui.style_mut().visuals.override_text_color = Some(hex_str_to_color(theme.literals));
					if ui.add(egui::Button::new("✒")).clicked() {
					    println!("yes");
					}
					
					if project.selected_item == Some([category_index, item_index]) {
						ui.style_mut().visuals.override_text_color = Some(hex_str_to_color(theme.bg));
						ui.add(egui::Button::new(item.name.clone()).fill(hex_str_to_color(theme.functions)));
					} else {
						ui.style_mut().visuals.override_text_color = Some(hex_str_to_color(theme.literals));
						if ui.add(egui::Button::new(item.name.clone()).fill(hex_str_to_color(theme.bg))).clicked() {
							project.selected_item = Some([category_index, item_index]);
						}
					}
				});
			}
			
			if category.name != "+" {
				if ui.add(egui::Button::new("+")).clicked() {
					project.categories[category_index].content.push(Item::new("item"));
				}
			}
		}
	});
}