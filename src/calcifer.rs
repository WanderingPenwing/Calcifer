use eframe::egui;
use egui::{text::CCursor, text_edit::CCursorRange};
use std::{env, path::Path, cmp::max};

use crate::tools;
use crate::Calcifer;
use crate::TIME_LABELS;
use crate::PATH_ROOT;
use crate::MAX_TABS;

pub mod code_editor;
use code_editor::CodeEditor;
use code_editor::themes::DEFAULT_THEMES;

mod app_base;


impl Calcifer {
	pub fn draw_settings(&mut self, ctx: &egui::Context) {
		egui::TopBottomPanel::top("settings")
			.resizable(false)
			.show(ctx, |ui| {
				ui.horizontal(|ui| {
					if ui.add(egui::Button::new("open file")).clicked() {
						if let Some(path) = rfd::FileDialog::new().set_directory(Path::new(&PATH_ROOT)).pick_file() {
							self.open_file(Some(&path));
						}
					}
					ui.separator();
					
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
					self.tree_display = self.toggle(ui, self.tree_display, "Tree");
					ui.separator();
					self.debug_display = self.toggle(ui, self.debug_display, "Debug");
					ui.separator();
					
					if self.debug_display {
						let combined_string: Vec<String> = TIME_LABELS.into_iter().zip(self.time_watch.clone().into_iter())
							.map(|(s, v)| format!("{} : {:.1} ms", s, v)).collect();
					
						let mut result = combined_string.join(" ;  ");
						result.push_str(&format!("	total : {:.1}", self.time_watch.clone().iter().sum::<f32>()));
						ui.label(result);
					}
				});
			});
	}
	
	
	pub fn draw_tree_panel(&mut self, ctx: &egui::Context) {
		if !self.tree_display {
			return
		}
		egui::SidePanel::left("file_tree_panel").show(ctx, |ui| {
			ui.heading("Bookshelf");
			ui.separator();
			let _ = self.list_files(ui, Path::new(&PATH_ROOT));
			ui.separator();
		});
	}
	
	
	pub fn draw_terminal_panel(&mut self, ctx: &egui::Context) {
		egui::TopBottomPanel::bottom("terminal")
			.default_height(super::TERMINAL_HEIGHT.clone())
			.min_height(0.0)
			.show(ctx, |ui| {
				ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
					let command_color = egui::Color32::from_hex(self.theme.functions).expect("Theme color issue (functions)");
					let entry_color = egui::Color32::from_hex(self.theme.literals).expect("Theme color issue (literals)");
					let bg_color = egui::Color32::from_hex(self.theme.bg).expect("Theme color issue (bg)");
					
					ui.label("");
					ui.horizontal(|ui| {
						ui.style_mut().visuals.extreme_bg_color = bg_color;
						let Self { command, .. } = self;
						ui.colored_label(command_color.clone(), tools::format_path(&env::current_dir().expect("Could not find Shell Environnment")));
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
								for entry in &mut self.command_history {
									entry.update();
									ui.colored_label(command_color, format!("\n{} {}", entry.env, entry.command));
									ui.end_row();
									if entry.output != "" {
										ui.colored_label(entry_color, &entry.output);
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
						
						if ui.link("X").clicked() && !self.close_tab_confirm.visible {
							if tab.saved {
								self.delete_tab(index);
							} else {
								self.close_tab_confirm.ask();
								self.tab_to_close = index;
							}
						}
						ui.separator();
					}
					if self.tabs.len() < MAX_TABS {
						ui.selectable_value(&mut self.selected_tab, tools::TabNumber::Open, "+");
					}
					if self.selected_tab == tools::TabNumber::Open {
						self.open_file(None);
					}
				});
			});
	}
	
	
	pub fn draw_content_panel(&mut self, ctx: &egui::Context) {
		egui::CentralPanel::default().show(ctx, |ui| {
			ui.horizontal(|ui| {
				if ui.add(egui::Button::new("open in terminal")).clicked() {
					let mut path = self.tabs[self.selected_tab.to_index()].path.clone();
					path.pop();
					tools::run_command(format!("cd {}", path.display()));
				}
				
				ui.label("Picked file:");
				ui.monospace(self.tabs[self.selected_tab.to_index()].path.to_string_lossy().to_string());
			});
			
			ui.separator();
			
			self.draw_code_file(ui);
		});
	}
	
	
	fn draw_code_file(&mut self, ui: &mut egui::Ui) {
		let current_tab = &mut self.tabs[self.selected_tab.to_index()];
		let lines = current_tab.code.chars().filter(|&c| c == '\n').count() + 1;
		let mut override_cursor : Option<CCursorRange> = None;

		if !self.search.result_selected {
			override_cursor = Some(CCursorRange::two(
							CCursor::new(self.search.get_cursor_start()),
							CCursor::new(self.search.get_cursor_end()),
						));
			self.search.result_selected = true;
		}
		
		CodeEditor::default().id_source("code editor")
					 	 .with_rows(max(45,lines))
					  	.with_fontsize(self.font_size)
					  	.with_theme(self.theme)
					  	.with_syntax(tools::to_syntax(&current_tab.language))
					  	.with_numlines(true)
					  	.show(ui, &mut current_tab.code, &mut current_tab.saved, &mut current_tab.last_cursor, &mut current_tab.scroll_offset, override_cursor);
	}
}
