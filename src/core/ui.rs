use eframe::egui;
use egui::{text::CCursor, text_edit::CCursorRange, Rangef};
use egui_extras::{Size, StripBuilder};
use std::{cmp::max, cmp::min, env, ffi::OsStr, path::Component, path::Path, path::PathBuf};

use crate::core;
use crate::editor;
use crate::panels;
use crate::Calcifer;
use crate::DISPLAY_PATH_DEPTH;
use crate::PROJECT_EXTENSION;
use crate::RED;
use crate::TERMINAL_HEIGHT;
use crate::TERMINAL_RANGE;
use editor::{CodeEditor, Syntax};

impl Calcifer {
	pub fn draw_settings(&mut self, ctx: &egui::Context) {
		egui::SidePanel::left("settings")
			.resizable(false)
			.exact_width(self.font_size * 1.8)
			.show(ctx, |ui| {
				ui.vertical(|ui| {
					if ui.add(egui::Button::new("📁")).clicked() {
						// if let Some(path_string) = tinyfiledialogs::open_file_dialog(
						//	 "Open File",
						//	 &self.home.to_string_lossy(),
						//	 None,
						// ) {
						//	 self.open_file(Some(&Path::new(&path_string)));
						// }
					}
					ui.separator();

					self.tree_visible = self.toggle(ui, self.tree_visible, "📦");
					ui.separator();

					let toggle_terminal = self.toggle(ui, self.terminal_visible, "🖵");
					if toggle_terminal && !self.terminal_visible {
						let mut path = self.tabs[self.selected_tab].path.clone();
						path.pop();
						panels::send_command(format!("cd {}", path.display()));
					}
					self.terminal_visible = toggle_terminal;
					ui.separator();

					self.search_menu.visible = self.toggle(ui, self.search_menu.visible, "🔍");
					ui.separator();

					self.settings_menu.visible = self.toggle(ui, self.settings_menu.visible, "⚙");
					ui.separator();

					self.shortcuts_menu.visible = self.toggle(ui, self.shortcuts_menu.visible, "⌨");
					ui.separator();

					self.profiler_visible = self.toggle(ui, self.profiler_visible, "⚡");

					if self.tabs[self.selected_tab].language == PROJECT_EXTENSION {
						ui.separator();
						self.project_content.item_window.visible =
							self.toggle(ui, self.project_content.item_window.visible, "🖊 ");
					}
				});
			});
	}

	pub fn draw_tree_panel(&mut self, ctx: &egui::Context) {
		if !self.tree_visible {
			return;
		}
		egui::SidePanel::left("file_tree_panel").show(ctx, |ui| {
			ui.horizontal(|ui| {
				ui.label(format!("Bookshelf   ({} files)	", self.n_file_displayed));
				if ui.button("↺").clicked() {
					self.file_tree = None;
				}
				if ui.button("🗙").clicked() {
					self.file_tree = None;
					self.tree_dir_opened = vec![];
				}
			});
			ui.separator();

			let mut init_update: bool = false;
			if self.file_tree.is_none() {
				self.file_tree = Some(panels::generate_folder_entry(self.home.as_path()));
				init_update = true
			}
			let mut n_files: usize = 0;

			egui::ScrollArea::vertical().show(ui, |ui| {
				if let Some(file_tree) = self.file_tree.clone() {
					let update_requested = self.list_files(ui, &file_tree, &mut n_files);
					if update_requested || init_update {
						self.file_tree = Some(panels::update_file_tree(
							file_tree,
							self.tree_dir_opened.clone(),
						));
					}
				} else {
					ui.label("No book on the Bookshelf");
				}
				ui.separator();
			});

			self.n_file_displayed = n_files;
		});
	}

	pub fn draw_bottom_tray(&mut self, ctx: &egui::Context) {
		egui::TopBottomPanel::bottom("tray")
			.default_height(self.font_size * 1.2)
			.resizable(false)
			.show(ctx, |ui| {
				ui.label(self.profiler());
			});
	}

	pub fn draw_terminal_panel(&mut self, ctx: &egui::Context) {
		if !self.terminal_visible {
			return;
		}
		egui::TopBottomPanel::bottom("terminal")
			.default_height(TERMINAL_HEIGHT)
			.height_range(Rangef::new(TERMINAL_RANGE.start, TERMINAL_RANGE.end))
			.resizable(true)
			.show(ctx, |ui| {
				ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
					let command_color = core::hex_str_to_color(self.theme.functions);
					let entry_color = core::hex_str_to_color(self.theme.literals);
					let bg_color = core::hex_str_to_color(self.theme.bg);

					ui.label("");

					ui.horizontal(|ui| {
						if ui.add(egui::Button::new("⟳")).clicked() {
							self.command_history.retain(|e| !e.finished);
						}
						ui.style_mut().visuals.extreme_bg_color = bg_color;
						let Self { command, .. } = self;
						ui.colored_label(
							command_color,
							format_path(&env::current_dir().unwrap_or_else(|_| PathBuf::from("/"))),
						);
						let response = ui.add(
							egui::TextEdit::singleline(command)
								.desired_width(f32::INFINITY)
								.lock_focus(true),
						);

						if response.lost_focus() && ctx.input(|i| i.key_pressed(egui::Key::Enter)) {
							self.command_history
								.push(panels::send_command(self.command.clone()));
							self.command = "".into();
							response.request_focus();
						}
					});
					ui.separator();
					egui::ScrollArea::vertical()
						.stick_to_bottom(true)
						.show(ui, |ui| {
							ui.with_layout(egui::Layout::top_down(egui::Align::LEFT), |ui| {
								ui.separator();
								ui.spacing_mut().item_spacing.y = 0.0;
								ui.style_mut().visuals.hyperlink_color =
									core::hex_str_to_color(self.theme.keywords);

								let mut running_command = false;

								for entry in &mut self.command_history {
									ui.label("");
									ui.horizontal(|ui| {
										if !entry.finished {
											running_command = true;
											entry.update();
											let _ = ui.link("(⌛)");
										} else if ui.link("(🗐)").clicked() {
											entry.copy_error_code();
										}
										ui.colored_label(
											command_color,
											format!("{} {}", entry.env, entry.command),
										);
									});

									for line in &entry.result {
										let color = if line.error { RED } else { entry_color };
										ui.add(
											egui::Label::new(
												egui::RichText::new(&line.text)
													.monospace()
													.color(color),
											), //.selectable(true)
										);
									}
								}

								self.running_command = running_command;
							});
						});
				});
			});
	}

	pub fn draw_tab_panel(&mut self, ctx: &egui::Context) {
		egui::TopBottomPanel::top("tabs")
			.resizable(false)
			.show(ctx, |ui| {
				ui.painter().rect_filled(
					ui.available_rect_before_wrap(),
					0.0,
					core::hex_str_to_color(self.theme.bg),
				);
				let response = StripBuilder::new(ui)
					.sizes(Size::remainder(), self.tab_area_size())
					.sense(egui::Sense::click())
					.horizontal(|mut strip| {
						for (index, tab) in self.tabs.clone().iter().enumerate() {
							strip.cell(|ui| {
								let mut color = core::hex_str_to_color(self.theme.comments);
								if self.selected_tab == index {
									ui.painter().rect_filled(
										ui.available_rect_before_wrap(),
										0.0,
										core::hex_str_to_color(self.theme.functions),
									);
									color = core::hex_str_to_color(self.theme.bg)
								}
								let unsaved_indicator = if tab.saved { "" } else { "~ " };
								ui.with_layout(
									egui::Layout::right_to_left(egui::Align::TOP),
									|ui| {
										ui.label("");
										if ui
											.add(
												egui::Label::new(
													egui::RichText::new(" ❌ ").color(color),
												)
												.sense(egui::Sense::click()),
											)
											.clicked()
											&& !self.close_tab_confirm.visible
										{
											if tab.saved {
												self.delete_tab(index);
											} else {
												self.close_tab_confirm.ask();
												self.tab_to_close = index;
											}
										}
										ui.with_layout(
											egui::Layout::left_to_right(egui::Align::TOP),
											|ui| {
												if ui
													.add(
														egui::Label::new(
															egui::RichText::new(format!(
																" {}{}",
																unsaved_indicator,
																tab.get_name()
															))
															.color(color),
														)
														.truncate(true)
														.sense(egui::Sense::click()),
													)
													.clicked()
													|| ui
														.add_sized(
															ui.available_size(),
															egui::Label::new("")
																.sense(egui::Sense::click()),
														)
														.clicked()
												{
													self.selected_tab = index;
												}
											},
										);
									},
								);
							});
						}
						strip.cell(|ui| {
							if ui
								.add(egui::Label::new("  ➕").sense(egui::Sense::click()))
								.clicked()
							{
								self.open_file(None);
							}
						});
					});
				self.tab_rect = response.rect;
			});
	}

	pub fn draw_content_panel(&mut self, ctx: &egui::Context) {
		egui::CentralPanel::default().show(ctx, |ui| {
			if self.selected_tab >= self.tabs.len() {
				return;
			}
			ui.horizontal(|ui| {
				ui.style_mut().visuals.hyperlink_color =
					core::hex_str_to_color(self.theme.comments);
				if ui
					.link(
						self.tabs[self.selected_tab]
							.path
							.to_string_lossy()
							.to_string(),
					)
					.clicked()
				{
					let mut current_path = self.tabs[self.selected_tab].path.clone();
					current_path.pop();

					while let Some(parent) = current_path.parent() {
						let dir_id = panels::get_file_path_id(&current_path);
						if !self.tree_dir_opened.contains(&dir_id) {
							self.tree_dir_opened.push(dir_id);
						}
						current_path = parent.to_path_buf();
					}

					self.tree_visible = true;
					self.file_tree = None;
				}
			});

			ui.separator();
			if self.tabs[self.selected_tab].language == PROJECT_EXTENSION {
				self.draw_project_file(ctx, ui);
			} else {
				self.draw_code_file(ui);
			}
		});
	}

	fn draw_code_file(&mut self, ui: &mut egui::Ui) {
		let current_tab = &mut self.tabs[self.selected_tab];
		let lines = current_tab.code.chars().filter(|&c| c == '\n').count() + 1;
		let mut override_cursor: Option<CCursorRange> = None;

		if !self.search_menu.result_selected {
			override_cursor = Some(CCursorRange::two(
				CCursor::new(self.search_menu.get_cursor_start()),
				CCursor::new(self.search_menu.get_cursor_end()),
			));
			self.search_menu.result_selected = true;
		}

		let tab_id = current_tab.path.clone().to_string_lossy().to_string();

		if self.got_focus {
			CodeEditor::default()
				.id_source(&tab_id)
				.with_rows(max(45, lines))
				.with_fontsize(self.font_size)
				.with_theme(self.theme)
				.with_syntax(to_syntax(&current_tab.language))
				.with_numlines(true)
				.show(
					ui,
					&mut current_tab.code.clone(),
					&mut current_tab.saved.clone(),
					&mut current_tab.last_cursor.clone(),
					&mut current_tab.scroll_offset.clone(),
					override_cursor.clone(),
				);
			return;
		}

		CodeEditor::default()
			.id_source(&tab_id)
			.with_rows(max(45, lines))
			.with_fontsize(self.font_size)
			.with_theme(self.theme)
			.with_syntax(to_syntax(&current_tab.language))
			.with_numlines(true)
			.show(
				ui,
				&mut current_tab.code,
				&mut current_tab.saved,
				&mut current_tab.last_cursor,
				&mut current_tab.scroll_offset,
				override_cursor,
			);
	}

	fn draw_project_file(&mut self, ctx: &egui::Context, ui: &mut egui::Ui) {
		let current_tab = &mut self.tabs[self.selected_tab];

		self.project_content
			.update_from_code(current_tab.code.clone());
		panels::draw_project(ui, self.theme, &mut self.project_content);

		self.project_content.selected_item.category = min(
			self.project_content.categories.len() - 2,
			self.project_content.selected_item.category,
		);
		while self.project_content.categories[self.project_content.selected_item.category]
			.content
			.is_empty()
			&& self.project_content.selected_item.category > 0
		{
			self.project_content.selected_item.category -= 1;
		}
		if !self.project_content.categories[self.project_content.selected_item.category]
			.content
			.is_empty()
		{
			self.project_content.selected_item.row = min(
				self.project_content.categories[self.project_content.selected_item.category]
					.content
					.len()
					- 1,
				self.project_content.selected_item.row,
			);
		} else {
			self.project_content.selected_item.row = 0;
		}

		if self.project_content.item_window.visible {
			if self.project_content.categories.len() > 1
				&& !self.project_content.categories[self.project_content.selected_item.category]
					.content
					.is_empty()
			{
				let delete_item = self.project_content.item_window.show(
					ctx,
					&mut self.project_content.categories
						[self.project_content.selected_item.category]
						.content[self.project_content.selected_item.row],
				);

				if delete_item {
					self.project_content.item_window.visible = false;
					self.project_content.categories[self.project_content.selected_item.category]
						.content
						.remove(self.project_content.selected_item.row);
					if self.project_content.selected_item.row
						>= self.project_content.categories
							[self.project_content.selected_item.category]
							.content
							.len()
						&& self.project_content.selected_item.row > 0
					{
						self.project_content.selected_item.row -= 1;
					}
				}
			} else {
				self.project_content.item_window.visible = false;
			}
		}

		match self.project_content.save_to_code() {
			Ok(code) => {
				if current_tab.code != code {
					current_tab.code = code;
					current_tab.saved = false;
				}
			}
			Err(_err) => (),
		}
	}

	pub fn draw_windows(&mut self, ctx: &egui::Context) {
		if self.search_menu.visible {
			self.search_menu
				.show(ctx, &mut self.tabs, &mut self.selected_tab);
		}
		if self.close_tab_confirm.visible {
			self.close_tab_confirm.show(ctx);
		}
		if self.refresh_confirm.visible {
			self.refresh_confirm.show(ctx);
		}
		if self.exit_confirm.visible {
			self.exit_confirm.show(ctx);
		}
		if self.exit_confirm.proceed {
			for tab in self.tabs.iter_mut() {
				tab.saved = true;
			}
			egui::Context::send_viewport_cmd(ctx, egui::ViewportCommand::Close);
		}
		if self.shortcuts_menu.visible {
			self.shortcuts_menu.show(ctx);
		}
		if self.settings_menu.visible {
			self.settings_menu.show(ctx);
		}
		if self.settings_menu.updated {
			self.theme = self.settings_menu.theme;
		}

		self.handle_confirm();
	}

	pub fn draw_mouse_drag(&mut self, ctx: &egui::Context) {
		if ctx.input(|i| i.pointer.is_decidedly_dragging()) {
			if let Some(pos) = ctx.input(|i| i.pointer.interact_pos()) {
				match self.mouse_holder {
					panels::MouseHolder::TabHolder(index) => {
						let snapped_pos = egui::Pos2::new(
							pos.x,
							(self.tab_rect.max.y + self.tab_rect.min.y) / 2.0,
						);

						egui::Area::new(egui::Id::new("mouse_holder"))
							.fixed_pos(snapped_pos)
							.show(ctx, |ui| {
								let (bg_color, text_color) = if self.selected_tab == index {
									(
										core::hex_str_to_color(self.theme.functions),
										core::hex_str_to_color(self.theme.bg),
									)
								} else {
									(
										core::hex_str_to_color(self.theme.bg),
										core::hex_str_to_color(self.theme.comments),
									)
								};

								let rect = egui::Rect::from_center_size(
									snapped_pos,
									egui::Vec2::new(
										(self.tab_rect.max.x - self.tab_rect.min.x)
											/ usize_to_f32(self.tab_area_size()),
										self.tab_rect.max.y - self.tab_rect.min.y,
									),
								);

								ui.painter().rect_filled(rect, 0.0, bg_color);
								let unsaved_indicator =
									if self.tabs[index].saved { "" } else { "~ " };

								let _ = ui.put(
									rect,
									egui::Label::new(
										egui::RichText::new(format!(
											" {}{}",
											unsaved_indicator,
											self.tabs[index].get_name()
										))
										.color(text_color),
									),
								);
							});
					}
					panels::MouseHolder::None => {
						if self.tab_rect.distance_to_pos(pos) == 0.0 {
							let hover_pos: f32 = (pos.x - self.tab_rect.min.x)
								/ ((self.tab_rect.max.x - self.tab_rect.min.x)
									/ usize_to_f32(self.tab_area_size()));

							if let Some(index) = floor_f32(hover_pos) {
								if index < self.tabs.len() {
									self.mouse_holder = panels::MouseHolder::TabHolder(index);
								}
							}
						}
					}
				}
			}
			return;
		}

		match self.mouse_holder {
			panels::MouseHolder::TabHolder(initial_index) => {
				if let Some(pos) = ctx.input(|i| i.pointer.interact_pos()) {
					let snapped_pos =
						egui::Pos2::new(pos.x, (self.tab_rect.max.y + self.tab_rect.min.y) / 2.0);
					if self.tab_rect.distance_to_pos(snapped_pos) == 0.0 {
						let hover_pos: f32 = (pos.x - self.tab_rect.min.x)
							/ ((self.tab_rect.max.x - self.tab_rect.min.x)
								/ usize_to_f32(self.tab_area_size()));

						if let Some(drop_index) = floor_f32(hover_pos) {
							let final_index = drop_index.min(self.tabs.len() - 1);
							if final_index == initial_index {
								return;
							} else if final_index < initial_index {
								self.tabs
									.insert(final_index, self.tabs[initial_index].clone());
								self.tabs.remove(initial_index + 1);
							} else {
								self.tabs
									.insert(final_index + 1, self.tabs[initial_index].clone());
								self.tabs.remove(initial_index);
							}

							if self.selected_tab == initial_index {
								self.selected_tab = final_index;
							} else if self.selected_tab < initial_index
								&& self.selected_tab >= final_index
							{
								self.selected_tab += 1;
							} else if self.selected_tab > initial_index
								&& self.selected_tab <= final_index
							{
								self.selected_tab -= 1;
							}
						}
					}
				}
			}

			panels::MouseHolder::None => {}
		}

		self.mouse_holder = panels::MouseHolder::None;
	}
}

fn to_syntax(language: &str) -> Syntax {
	match language {
		"py" => Syntax::python(),
		"rs" => Syntax::rust(),
		"js" => Syntax::javascript(),
		"dr" => Syntax::pendragon(),
		_ => Syntax::shell(),
	}
}

pub fn format_path(path: &Path) -> String {
	let components: Vec<&OsStr> = path
		.components()
		.rev()
		.take(DISPLAY_PATH_DEPTH)
		.filter_map(|component| match component {
			Component::RootDir | Component::CurDir => None,
			_ => Some(component.as_os_str()),
		})
		.collect();

	format!(
		"{}>",
		components
			.iter()
			.rev()
			.map(|&c| c.to_string_lossy())
			.collect::<Vec<_>>()
			.join("/")
	)
}

fn usize_to_f32(value: usize) -> f32 {
	const MAX_F32: f32 = f32::MAX;

	if value as f64 > MAX_F32 as f64 {
		MAX_F32
	} else {
		value as f32
	}
}

fn floor_f32(value: f32) -> Option<usize> {
	if value.is_nan() || value < 0.0 || value > usize::MAX as f32 {
		None
	} else {
		Some(value.floor() as usize)
	}
}
