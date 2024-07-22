use eframe::egui;
use egui::Color32;
use std::{cmp::min, fs, path::Path, path::PathBuf};

use crate::core;
use crate::editor::themes::DEFAULT_THEMES;
use crate::panels;
use crate::save_path;
use crate::sub_windows;
use crate::Calcifer;
use crate::TIME_LABELS;

impl Calcifer {
	pub fn handle_confirm(&mut self) {
		if self.close_tab_confirm.proceed {
			self.close_tab_confirm.close();
			self.delete_tab(self.tab_to_close);
		}

		if self.refresh_confirm.proceed {
			self.refresh_confirm.close();
			self.tabs[self.selected_tab].refresh();
		}
	}

	pub fn save_tab(&self) -> Option<PathBuf> {
		if self.tabs[self.selected_tab]
			.path
			.file_name()
			.map_or(true, |name| name.to_string_lossy() == "untitled")
		{
			self.save_tab_as()
		} else {
			if let Err(err) = fs::write(
				&self.tabs[self.selected_tab].path,
				&self.tabs[self.selected_tab].code,
			) {
				eprintln!("Error writing file: {}", err);
				return self.save_tab_as();
			}
			Some(self.tabs[self.selected_tab].path.clone())
		}
	}

	pub fn save_tab_as(&self) -> Option<PathBuf> {
		let default_path = self.home.join("untitled");
		
		let save_path = if self.tabs[self.selected_tab].path.file_name().map_or(true, |name| name.to_string_lossy() == "untitled")
		{
			default_path.to_string_lossy()
		} else {
			self.tabs[self.selected_tab].path.to_string_lossy()
		};
		println!("app : open dialog at {}", save_path);
		if let Some(path_string) = tinyfiledialogs::save_file_dialog("Save as", &save_path)
		{
			let path = PathBuf::from(path_string);
			if let Err(err) = fs::write(&path, &self.tabs[self.selected_tab].code) {
				eprintln!("Error writing file: {}", err);
				return None;
			}
			return Some(path);
		}
		None
	}

	pub fn handle_save_file(&mut self, path_option: Option<PathBuf>) {
		if let Some(path) = path_option {
			println!("File saved successfully at: {:?}", path);
			self.tabs[self.selected_tab].path = path;
			self.tabs[self.selected_tab].saved = true;
		} else {
			println!("File save failed.");
		}
	}

	pub fn from_app_state(app_state: core::AppState, file_to_open: Option<PathBuf>) -> Self {
		let mut new = Self {
			theme: DEFAULT_THEMES[min(app_state.theme, DEFAULT_THEMES.len() - 1)],
			tabs: Vec::new(),
			settings_menu: sub_windows::SettingsWindow::new(DEFAULT_THEMES[app_state.theme]),
			..Default::default()
		};

		if app_state.zoom != 0.0 {
			new.zoom = app_state.zoom;
		}

		for path in app_state.tabs {
			if !path
				.file_name()
				.map_or(true, |name| name.to_string_lossy() == "untitled")
			{
				new.open_file(Some(&path));
			}
		}
		
		if let Some(path) = file_to_open {
			new.open_file(Some(&path));
		}

		if new.tabs == vec![] {
			new.open_file(None);
		}

		new
	}

	pub fn save_state(&self) {
		let mut state_theme: usize = 0;
		if let Some(theme) = DEFAULT_THEMES.iter().position(|&r| r == self.theme) {
			state_theme = theme;
		}

		let mut state_tabs = vec![];

		for tab in &self.tabs {
			state_tabs.push(tab.path.clone());
		}
		let app_state = core::AppState {
			tabs: state_tabs,
			theme: state_theme,
			zoom: self.zoom,
		};

		let _ = core::save_state(&app_state, save_path().as_path());
	}

	pub fn move_through_tabs(&mut self, forward: bool) {
		let new_index = if forward {
			(self.selected_tab + 1) % self.tabs.len()
		} else {
			self.selected_tab
				.checked_sub(1)
				.unwrap_or(self.tabs.len() - 1)
		};
		self.selected_tab = new_index;
	}

	pub fn open_file(&mut self, path_option: Option<&Path>) {
		if let Some(path) = path_option {
			for (index, tab) in self.tabs.clone().iter().enumerate() {
				if tab.path == path {
					self.selected_tab = index;
					return;
				}
			}
		}
		if let Some(path) = path_option {
			self.tabs.push(panels::Tab::new(path.to_path_buf()));
		} else {
			self.tabs.push(panels::Tab::default());
		}
		self.selected_tab = self.tabs.len() - 1;
	}

	pub fn delete_tab(&mut self, index: usize) {
		self.tabs.remove(index);
		self.selected_tab = min(index, self.tabs.len() - 1);
	}

	pub fn toggle(&self, ui: &mut egui::Ui, display: bool, title: &str) -> bool {
		let bg_color: Color32;
		let text_color: Color32;

		if display {
			bg_color = hex_str_to_color(self.theme.functions);
			text_color = hex_str_to_color(self.theme.bg);
		} else {
			bg_color = hex_str_to_color(self.theme.bg);
			text_color = hex_str_to_color(self.theme.literals);
		};

		ui.style_mut().visuals.override_text_color = Some(text_color);

		if ui.add(egui::Button::new(title).fill(bg_color)).clicked() {
			return !display;
		}
		ui.style_mut().visuals.override_text_color = None;

		display
	}

	pub fn profiler(&self) -> String {
		if !self.profiler_visible {
			return "".to_string();
		}
		let combined_string: Vec<String> = TIME_LABELS
			.into_iter()
			.zip(self.time_watch.clone())
			.map(|(s, v)| format!("{} : {:.1} ms", s, v))
			.collect();

		let mut result = combined_string.join(" ;  ");
		result.push_str(&format!(
			"	total : {:.1} ms",
			self.time_watch.clone().iter().sum::<f32>()
		));
		result
	}

	pub fn list_files(
		&mut self,
		ui: &mut egui::Ui,
		file: &panels::FileEntry,
		n_files: &mut usize,
	) -> bool {
		*n_files += 1;

		if let Some(folder_content) = &file.folder_content {
			let mut check_for_update: bool = false;
			let collapsing_response = egui::CollapsingHeader::new(file.name.clone())
				.default_open(self.tree_dir_opened.contains(&file.name))
				.show(ui, |ui| {
					if !self.tree_dir_opened.contains(&file.name) {
						return;
					}
					for deeper_file in folder_content {
						if self.list_files(ui, deeper_file, n_files) {
							check_for_update = true;
						}
					}
				});
			if collapsing_response.fully_closed() {
				self.tree_dir_opened.retain(|s| s != &file.name);
			} else if !self.tree_dir_opened.contains(&file.name) {
				self.tree_dir_opened.push(file.name.clone());
				return !file.content_checked;
			}
			return check_for_update;
		} else if ui.button(&file.name).clicked() {
			self.open_file(Some(&file.path));
		}

		false
	}
}

#[allow(clippy::unnecessary_lazy_evaluations)]
pub fn hex_str_to_color(hex_str: &str) -> Color32 {
	Color32::from_hex(hex_str).unwrap_or_else(|_| Color32::BLACK)
}
