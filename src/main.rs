
mod tools;

use eframe::egui;
use egui_code_editor::{CodeEditor, ColorTheme};
use std::{path::Path, path::PathBuf, fs, io, env, cmp::max, cmp::min};

const TERMINAL_HEIGHT : f32 = 200.0;
const RED : egui::Color32 = egui::Color32::from_rgb(235, 108, 99);
const HISTORY_LENGTH : usize = 2;


fn main() -> Result<(), eframe::Error> {
	tools::loaded();
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1200.0, 800.0])
            .with_drag_and_drop(true),
        ..Default::default()
    };
    eframe::run_native(
        "Calcifer",
        options,
        Box::new(|_cc| Box::<Calcifer>::default()),
    )
}


struct Calcifer {
    selected_tab : tools::TabNumber,
    tabs: Vec<tools::Tab>,
    command: String,
    command_history: Vec<tools::CommandEntry>,
    theme: ColorTheme,
}


impl Default for Calcifer {
    fn default() -> Self {
        Self {
            selected_tab : tools::TabNumber::Zero,
			tabs: vec![ tools::Tab::default()],
            command: "".into(),
            command_history: Vec::new(),
            theme: tools::themes::CustomColorTheme::fire()
        }
    }
}


impl eframe::App for Calcifer {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
		if ctx.input( |i| i.key_pressed(egui::Key::S) && i.modifiers.ctrl) {
			if let Some(path) = self.save_tab() {
				println!("File saved successfully at: {:?}", path);
				self.tabs[self.selected_tab.to_n()].path = path;
			} else {
				println!("File save failed.");
			}
		}
		
		if ctx.input( |i| i.key_pressed(egui::Key::S) && i.modifiers.ctrl && i.modifiers.shift) {
			if let Some(path) = self.save_tab_as() {
				println!("File saved successfully at: {:?}", path);
				self.tabs[self.selected_tab.to_n()].path = path;
			} else {
				println!("File save failed.");
			}
		}
		
		if ctx.input( |i| i.key_pressed(egui::Key::Z) && i.modifiers.ctrl) {
			self.undo();
		}
		
		self.draw_settings(ctx);
        self.draw_tree_panel(ctx);
        self.draw_terminal_panel(ctx);
        self.draw_tab_panel(ctx);
        self.draw_content_panel(ctx);
	}
}


impl Calcifer {
	fn draw_settings(&mut self, ctx: &egui::Context) {
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
							ui.selectable_value(&mut self.theme, ColorTheme::SONOKAI, "Sonokai");
							ui.selectable_value(&mut self.theme, ColorTheme::AYU_DARK, "Ayu Dark");
							ui.selectable_value(&mut self.theme, ColorTheme::AYU_MIRAGE, "Ayu Mirage");
							ui.selectable_value(&mut self.theme, ColorTheme::GITHUB_DARK, "Github Dark");
							ui.selectable_value(&mut self.theme, ColorTheme::GRUVBOX, "Gruvbox");
							ui.selectable_value(&mut self.theme, tools::themes::CustomColorTheme::fire(), "Fire");
							ui.selectable_value(&mut self.theme, tools::themes::CustomColorTheme::ash(), "Ash");
						});
				});
			});
	}
	
    fn draw_tree_panel(&mut self, ctx: &egui::Context) {
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

    fn draw_terminal_panel(&mut self, ctx: &egui::Context) {
        egui::TopBottomPanel::bottom("terminal")
            .default_height(TERMINAL_HEIGHT.clone())
            .min_height(0.0)
            .show(ctx, |ui| {
                ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
                    ui.label("");
                    ui.horizontal(|ui| {
						ui.style_mut().visuals.extreme_bg_color = egui::Color32::from_hex("#101010").expect("Could not convert color");
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
										ui.colored_label(RED, &entry.error);
										ui.end_row();
									}
								}
							});
						});
                    });
                });
            });
    }
    
    fn draw_tab_panel(&mut self, ctx: &egui::Context) {
		egui::TopBottomPanel::top("tabs")
			.resizable(false)
			.show(ctx, |ui| {
				ui.horizontal(|ui| {
					ui.style_mut().visuals.selection.bg_fill = egui::Color32::from_hex("#b56524").expect("Could not convert color");
					ui.style_mut().visuals.hyperlink_color = egui::Color32::from_hex("#ffad69").expect("Could not convert color");
					for (index, tab) in self.tabs.clone().iter().enumerate() {
						let mut title = tab.get_name();
						if !tab.saved {
							title += " ~";
						}
						ui.selectable_value(&mut self.selected_tab, tools::TabNumber::from_n(index), title);
						if ui.link("X").clicked() {
							self.selected_tab = self.delete_tab(index);
						}
						ui.separator();
					}
					if tools::TabNumber::from_n(self.tabs.len()) != tools::TabNumber::None {
						ui.selectable_value(&mut self.selected_tab, tools::TabNumber::Open, "+");
					}
					if self.selected_tab == tools::TabNumber::Open {
						self.selected_tab = self.new_tab();
					}
				});
			});
	}

    fn draw_content_panel(&mut self, ctx: &egui::Context) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal(|ui| {
				ui.label("Picked file:");
				ui.monospace(self.tabs[self.selected_tab.to_n()].path.to_string_lossy().to_string());
			});
			
			if self.selected_tab == tools::TabNumber::None {
				return
			}
			
			self.draw_code_file(ui);
        });
    }
    
    fn draw_code_file(&mut self, ui: &mut egui::Ui) {
		let current_tab = &mut self.tabs[self.selected_tab.to_n()];
		let lines = current_tab.code.chars().filter(|&c| c == '\n').count() + 1;
		
		egui::ScrollArea::vertical().show(ui, |ui| {
			CodeEditor::default()
					  .id_source("code editor")
					  .with_rows(max(80, lines))
					  .with_fontsize(14.0)
					  .with_theme(self.theme)
					  .with_syntax(tools::to_syntax(&current_tab.language))
					  .with_numlines(true)
					  .show(ui, &mut current_tab.code);
		});
		
		if current_tab.history.len() < 1 {
			current_tab.history.push(current_tab.code.clone());
		}
		
		if &current_tab.code != current_tab.history.last().expect("There should be an history") {
			current_tab.history.push(current_tab.code.clone());
			current_tab.saved = false;
			if current_tab.history.len() > HISTORY_LENGTH {
				current_tab.history.remove(0);
			}
		}
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
		if tools::TabNumber::from_n(self.tabs.len()) == tools::TabNumber::None {
			return tools::TabNumber::None
		}
		
		let new_tab = tools::Tab {
			path: path.into(),
			code: fs::read_to_string(path).expect("Not able to read the file"),
			language: path.to_str().unwrap().split('.').last().unwrap().into(),
			saved: true,
			history: vec![],
		};
		self.tabs.push(new_tab);
		
		return tools::TabNumber::from_n(self.tabs.len() - 1)
	}
	
	fn new_tab(&mut self) -> tools::TabNumber {
		self.tabs.push(tools::Tab::default());
		return tools::TabNumber::from_n(self.tabs.len() - 1)
	}
	
	fn delete_tab(&mut self, index : usize) -> tools::TabNumber {
		self.tabs.remove(index);
		return tools::TabNumber::from_n(min(index, self.tabs.len() - 1))
	}
	
	fn save_tab(&self) -> Option<PathBuf> {
		if self.tabs[self.selected_tab.to_n()].path.file_name().expect("Could not get Tab Name").to_string_lossy().to_string() == "untitled" {
			return self.save_tab_as();
		} else {
			if let Err(err) = fs::write(&self.tabs[self.selected_tab.to_n()].path, &self.tabs[self.selected_tab.to_n()].code) {
				eprintln!("Error writing file: {}", err);
				return None;
			}
			return Some(self.tabs[self.selected_tab.to_n()].path.clone())
		}
	}
	
	fn save_tab_as(&self) -> Option<PathBuf> {
		if let Some(path) = rfd::FileDialog::new().save_file() {
			if let Err(err) = fs::write(&path, &self.tabs[self.selected_tab.to_n()].code) {
				eprintln!("Error writing file: {}", err);
				return None;
			}
			return Some(path);
		}
		return None
	}
	
	fn undo(&mut self) {
		let current_tab = &mut self.tabs[self.selected_tab.to_n()];
		if current_tab.history.len() < 2 {
			return
		}
		current_tab.code = current_tab.history[current_tab.history.len() - 2].clone();
		current_tab.history.pop();
	}
}

