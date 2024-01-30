use eframe::egui;
use egui::{text::CCursor, text_edit::CCursorRange, Rangef};
use std::{cmp::max, env, ffi::OsStr, path::Component, path::Path, path::PathBuf};

use crate::core;
use crate::editor;
use crate::panels;
use crate::Calcifer;
use crate::DISPLAY_PATH_DEPTH;
use crate::MAX_TABS;
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
                        if let Some(path) = rfd::FileDialog::new()
                            .set_directory(self.home.as_path())
                            .pick_file()
                        {
                            self.open_file(Some(&path));
                        }
                    }
                    ui.separator();
                    self.tree_visible = self.toggle(ui, self.tree_visible, "📦");
                    ui.separator();
                    self.terminal_visible = self.toggle(ui, self.terminal_visible, "🖵");
                    ui.separator();
                    self.search_menu.visible = self.toggle(ui, self.search_menu.visible, "🔍");
                    ui.separator();
                    self.settings_menu.visible = self.toggle(ui, self.settings_menu.visible, "⚙");
                    ui.separator();
                    self.shortcuts_menu.visible = self.toggle(ui, self.shortcuts_menu.visible, "⌨");
                    ui.separator();
                    self.profiler_visible = self.toggle(ui, self.profiler_visible, "⚡");

                    if self.tabs[self.selected_tab.to_index()].language == PROJECT_EXTENSION {
                        ui.separator();
                        self.project_mode = self.toggle(ui, self.project_mode, "✒");
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
                ui.label("Bookshelf ");
                if ui.add(egui::Button::new("📖")).clicked() {
                    self.file_tree = panels::generate_file_tree(self.home.as_path(), 7);
                }
            });
            ui.separator();
            let mut n_files: usize = 0;
            if let Some(file_tree) = self.file_tree.clone() {
                self.list_files(ui, &file_tree, 1, &mut n_files);
            } else {
                ui.label("No book on the Bookshelf");
            }
            ui.separator();
            ui.label(format!("{} files displayed", n_files));
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

                                for entry in &mut self.command_history {
                                    ui.label("");
                                    ui.horizontal(|ui| {
                                        if !entry.finished {
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
                                        ui.colored_label(color, &line.text);
                                    }
                                }
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
                    ui.style_mut().visuals.selection.bg_fill =
                        core::hex_str_to_color(self.theme.functions);
                    ui.style_mut().visuals.hyperlink_color =
                        core::hex_str_to_color(self.theme.functions);
                    for (index, tab) in self.tabs.clone().iter().enumerate() {
                        let mut title = tab.get_name();
                        if !tab.saved {
                            title += " ~";
                        }
                        if self.selected_tab == panels::TabNumber::from_index(index) {
                            ui.style_mut().visuals.override_text_color =
                                Some(core::hex_str_to_color(self.theme.bg));
                        }
                        ui.selectable_value(
                            &mut self.selected_tab,
                            panels::TabNumber::from_index(index),
                            title,
                        );

                        ui.style_mut().visuals.override_text_color = None;

                        if ui.link("X").clicked() && !self.close_tab_confirm.visible {
                            if self.tabs.len() > 1 {
                                if tab.saved {
                                    self.delete_tab(index);
                                } else {
                                    self.close_tab_confirm.ask();
                                    self.tab_to_close = index;
                                }
                            } else {
                                egui::Context::send_viewport_cmd(ctx, egui::ViewportCommand::Close);
                            }
                        }
                        ui.separator();
                    }
                    if self.tabs.len() < MAX_TABS {
                        ui.selectable_value(&mut self.selected_tab, panels::TabNumber::Open, "+");
                    }
                    if self.selected_tab == panels::TabNumber::Open {
                        self.open_file(None);
                    }
                });
            });
    }

    pub fn draw_content_panel(&mut self, ctx: &egui::Context) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal(|ui| {
                if ui
                    .add(egui::Button::new("open directory in terminal"))
                    .clicked()
                {
                    let mut path = self.tabs[self.selected_tab.to_index()].path.clone();
                    path.pop();
                    panels::send_command(format!("cd {}", path.display()));
                }

                ui.label("Picked file:");
                ui.monospace(
                    self.tabs[self.selected_tab.to_index()]
                        .path
                        .to_string_lossy()
                        .to_string(),
                );
            });

            ui.separator();
            if self.project_mode
                && self.tabs[self.selected_tab.to_index()].language == PROJECT_EXTENSION
            {
                self.draw_project_file(ui);
            } else {
                self.draw_code_file(ui);
            }
        });
    }

    fn draw_code_file(&mut self, ui: &mut egui::Ui) {
        let current_tab = &mut self.tabs[self.selected_tab.to_index()];
        let lines = current_tab.code.chars().filter(|&c| c == '\n').count() + 1;
        let mut override_cursor: Option<CCursorRange> = None;

        if !self.search_menu.result_selected {
            override_cursor = Some(CCursorRange::two(
                CCursor::new(self.search_menu.get_cursor_start()),
                CCursor::new(self.search_menu.get_cursor_end()),
            ));
            self.search_menu.result_selected = true;
        }

        CodeEditor::default()
            .id_source("code editor")
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

    fn draw_project_file(&mut self, ui: &mut egui::Ui) {
        ui.label("project mode");
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
}

fn to_syntax(language: &str) -> Syntax {
    match language {
        "py" => Syntax::python(),
        "rs" => Syntax::rust(),
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
