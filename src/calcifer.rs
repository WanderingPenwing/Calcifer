use eframe::egui;
use egui::{text::CCursor, text_edit::CCursorRange};
use std::{env, path::Path, path::PathBuf, cmp::max, io, fs, cmp::min};
use crate::tools;
use crate::TIME_LABELS;

pub mod code_editor;
use code_editor::CodeEditor;
use code_editor::themes::DEFAULT_THEMES;



impl super::Calcifer {
	pub fn draw_settings(&mut self, ctx: &egui::Context) {
		egui::TopBottomPanel::top("settings")
			.resizable(false)
			.show(ctx, |ui| {
				ui.horizontal(|ui| {
					ui.label("Theme ");
					egui::ComboBox::from_label("")
						.selected_text(format!("{}", self.theme.name))
						.show_ui(ui, |ui| {
							ui.style_mut().wrap = Some(false);
							ui.set_min_width(60.0);
							for theme in DEFAULT_THEMES {
								ui.selectable_value(&mut self.theme, theme, theme.name);
							}
						});
						
					ui.separator();
					ui.checkbox(&mut self.debug_display, "Debug display");
					ui.separator();
					
					if self.debug_display {
						let combined_string: Vec<String> = TIME_LABELS.into_iter().zip(self.time_watch.clone().into_iter())
					        .map(|(s, v)| format!("{} : {:.1} ms", s, v)).collect();
					
					    let mut result = combined_string.join(" ;  ");
						result.push_str(&format!("    total : {:.1}", self.time_watch.clone().iter().sum::<f32>()));
						ui.label(result);
					}
				});
			});
	}
	
	pub fn draw_tree_panel(&mut self, ctx: &egui::Context) {
		egui::SidePanel::left("file_tree_panel").show(ctx, |ui| {
			ui.heading("Bookshelf");
			if ui.add(egui::Button::new("open file")).clicked() {
				if let Some(path) = rfd::FileDialog::new().pick_file() {
					self.selected_tab = self.open_file(&path);
				}
			}
			ui.separator();
			let _ = self.list_files(ui, Path::new("/home/penwing/Documents/"));
			ui.separator();
		});
	}

	pub fn draw_terminal_panel(&mut self, ctx: &egui::Context) {
		egui::TopBottomPanel::bottom("terminal")
			.default_height(super::TERMINAL_HEIGHT.clone())
			.min_height(0.0)
			.show(ctx, |ui| {
				ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
					ui.label("");
					ui.horizontal(|ui| {
						ui.style_mut().visuals.extreme_bg_color = egui::Color32::from_hex(self.theme.bg).expect("Could not convert color");
						let Self { command, .. } = self;
						ui.label(format!("{}>", env::current_dir().expect("Could not find Shell Environnment").file_name().expect("Could not get Shell Environnment Name").to_string_lossy().to_string()));
						let response = ui.add(egui::TextEdit::singleline(command).desired_width(f32::INFINITY).lock_focus(true));

						if response.lost_focus() && ctx.input(|i| i.key_pressed(egui::Key::Enter)) {
							self.command_history.push(tools::run_command(self.command.clone()));
							self.command = "".into();
							response.request_focus();
						}
					});
					ui.separator();
					egui::ScrollArea::vertical().stick_to_bottom(true).show(ui, |ui| {
						ui.with_layout(egui::Layout::top_down(egui::Align::LEFT), |ui| {
							ui.separator();
							ui.horizontal_wrapped(|ui| {
								ui.spacing_mut().item_spacing.y = 0.0;
								for entry in &self.command_history {
									ui.label(format!("{}> {}", entry.env, entry.command));
									ui.end_row();
									if entry.output != "" {
										ui.label(&entry.output);
										ui.end_row();
									}
									if entry.error != "" {
										ui.colored_label(super::RED, &entry.error);
										ui.end_row();
									}
								}
							});
						});
					});
				});
			});
	}
	
	pub fn draw_tab_panel(&mut self, ctx: &egui::Context) {
		egui::TopBottomPanel::top("tabs")
			.resizable(false)
			.show(ctx, |ui| {
				ui.horizontal(|ui| {
					ui.style_mut().visuals.selection.bg_fill = egui::Color32::from_hex(self.theme.functions).expect("Could not convert color");
					ui.style_mut().visuals.hyperlink_color = egui::Color32::from_hex(self.theme.functions).expect("Could not convert color");
					for (index, tab) in self.tabs.clone().iter().enumerate() {
						let mut title = tab.get_name();
						if !tab.saved {
							title += " ~";
						}
						if self.selected_tab == tools::TabNumber::from_index(index) {
							ui.style_mut().visuals.override_text_color = Some(egui::Color32::from_hex(self.theme.bg).expect("Could not convert color"));
						}
						ui.selectable_value(&mut self.selected_tab, tools::TabNumber::from_index(index), title);
						
						ui.style_mut().visuals.override_text_color = None;
						
						if ui.link("X").clicked() {
							self.selected_tab = self.delete_tab(index);
						}
						ui.separator();
					}
					if tools::TabNumber::from_index(self.tabs.len()) != tools::TabNumber::None {
						ui.selectable_value(&mut self.selected_tab, tools::TabNumber::Open, "+");
					}
					if self.selected_tab == tools::TabNumber::Open {
						self.selected_tab = self.new_tab();
					}
				});
			});
	}

	pub fn draw_content_panel(&mut self, ctx: &egui::Context) {
		egui::CentralPanel::default().show(ctx, |ui| {
			ui.horizontal(|ui| {
				ui.label("Picked file:");
				ui.monospace(self.tabs[self.selected_tab.to_index()].path.to_string_lossy().to_string());
			});
			
			if self.selected_tab == tools::TabNumber::None {
				return
			}
			
			self.draw_code_file(ui);
		});
	}
	
	fn draw_code_file(&mut self, ui: &mut egui::Ui) {
		let current_tab = &mut self.tabs[self.selected_tab.to_index()];
		let lines = current_tab.code.chars().filter(|&c| c == '\n').count() + 1;
		let mut override_cursor : Option<CCursorRange> = None;

		if !self.search.result_selected && self.search.tab_selected {
			override_cursor = Some(CCursorRange::two(
							CCursor::new(self.search.get_cursor_start()),
							CCursor::new(self.search.get_cursor_end()),
						));
			self.search.result_selected = true;
		}
		
		CodeEditor::default().id_source("code editor")
					 	 .with_rows(max(45,lines))
					  	.with_fontsize(14.0)
					  	.with_theme(self.theme)
					  	.with_syntax(tools::to_syntax(&current_tab.language))
					  	.with_numlines(true)
					  	.show(ui, &mut current_tab.code, &mut current_tab.saved, &mut current_tab.last_cursor, &mut current_tab.scroll_offset, override_cursor);
	}

	pub fn save_tab(&self) -> Option<PathBuf> {
		if self.tabs[self.selected_tab.to_index()].path.file_name().expect("Could not get Tab Name").to_string_lossy().to_string() == "untitled" {
			return self.save_tab_as();
		} else {
			if let Err(err) = fs::write(&self.tabs[self.selected_tab.to_index()].path, &self.tabs[self.selected_tab.to_index()].code) {
				eprintln!("Error writing file: {}", err);
				return None;
			}
			return Some(self.tabs[self.selected_tab.to_index()].path.clone())
		}
	}
	
	pub fn save_tab_as(&self) -> Option<PathBuf> {
		if let Some(path) = rfd::FileDialog::new().save_file() {
			if let Err(err) = fs::write(&path, &self.tabs[self.selected_tab.to_index()].code) {
				eprintln!("Error writing file: {}", err);
				return None;
			}
			return Some(path);
		}
		return None
	}
	
	pub fn handle_save_file(&mut self, path_option : Option<PathBuf>) {
		if let Some(path) = path_option {
			println!("File saved successfully at: {:?}", path);
			self.tabs[self.selected_tab.to_index()].path = path;
			self.tabs[self.selected_tab.to_index()].saved = true;
		} else {
			println!("File save failed.");
		}
	}
	
	pub fn from_app_state(app_state: tools::AppState) -> Self {
		let mut new = Self {
			theme: DEFAULT_THEMES[min(app_state.theme, DEFAULT_THEMES.len() - 1)],
			tabs: Vec::new(),
			..Default::default()
		};
		
		for path in app_state.tabs {
			if path.file_name().expect("Could not get Tab Name").to_string_lossy().to_string() != "untitled" {
				new.open_file(&path);
			}
		}
		
		if new.tabs == vec![] {
			new.new_tab();
		}
		
		new
	}
	
	pub fn save_state(&self) {
		let mut state_theme : usize = 0;
		if let Some(theme) = DEFAULT_THEMES.iter().position(|&r| r == self.theme) {
			state_theme = theme;
		}
		
		let mut state_tabs = vec![];
		
		for tab in &self.tabs {
			state_tabs.push(tab.path.clone());
		}
		let app_state = tools::AppState {
			tabs: state_tabs,
			theme: state_theme,
		};
		
		let _ = tools::save_state(&app_state, super::SAVE_PATH);
	}
	
	pub fn indent_with_tabs(&mut self) {
		let current_tab = &mut self.tabs[self.selected_tab.to_index()];
		current_tab.code = current_tab.code.replace("	", "\t")
	}
	
	fn list_files(&mut self, ui: &mut egui::Ui, path: &Path) -> io::Result<()> {
		if let Some(name) = path.file_name() {
			if path.is_dir() {
				egui::CollapsingHeader::new(name.to_string_lossy()).show(ui, |ui| {
					let mut paths: Vec<_> = fs::read_dir(&path).expect("Failed to read dir").map(|r| r.unwrap()).collect();
												  
					// Sort the vector using the custom sorting function
					paths.sort_by(|a, b| tools::sort_directories_first(a, b));

					for result in paths {
						//let result = path_result.expect("Failed to get path");
						//let full_path = result.path();
						let _ = self.list_files(ui, &result.path());
					}
				});
			} else {
				//ui.label(name.to_string_lossy());
				if ui.button(name.to_string_lossy()).clicked() {
					self.selected_tab = self.open_file(&path);
				}
			}
		}
		Ok(())
	}
	
	fn open_file(&mut self, path: &Path) -> tools::TabNumber {
		if tools::TabNumber::from_index(self.tabs.len()) == tools::TabNumber::None {
			return tools::TabNumber::None
		}
		
		let new_tab = tools::Tab {
			path: path.into(),
			code: fs::read_to_string(path).expect("Not able to read the file").replace("	", "\t"),
			language: path.to_str().unwrap().split('.').last().unwrap().into(),
			saved: true,
			..tools::Tab::default()
		};
		self.tabs.push(new_tab);
		
		return tools::TabNumber::from_index(self.tabs.len() - 1)
	}

	fn new_tab(&mut self) -> tools::TabNumber {
		self.tabs.push(tools::Tab::default());
		return tools::TabNumber::from_index(self.tabs.len() - 1)
	}
	
	fn delete_tab(&mut self, index : usize) -> tools::TabNumber {
		self.tabs.remove(index);
		return tools::TabNumber::from_index(min(index, self.tabs.len() - 1))
	}
}
