
//mod tools;

use eframe::egui;
use std::path::Path;
use std::fs;
use std::io;
use std::process::Command;
use std::cmp::Ordering;

const TERMINAL_HEIGHT : f32 = 200.0;


fn main() -> Result<(), eframe::Error> {
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
        Box::new(|_cc| Box::<MyApp>::default()),
    )
}


fn run_command(cmd : String) -> String {
	let command = "> ".to_owned() + &cmd.clone() + "\n";
	let output = Command::new("sh")
        .arg("-c")
        .arg(cmd)
        .output()
        .expect("failed to execute process");
	(command + &String::from_utf8_lossy(&output.stdout)).to_string()
}

fn sort_directories_first(a: &std::fs::DirEntry, b: &std::fs::DirEntry) -> Ordering {
    let a_is_dir = a.path().is_dir();
    let b_is_dir = b.path().is_dir();

    // Directories come first, then files
    if a_is_dir && !b_is_dir {
        Ordering::Less
    } else if !a_is_dir && b_is_dir {
        Ordering::Greater
    } else {
        // Both are either directories or files, sort alphabetically
        a.path().cmp(&b.path())
    }
}

fn list_files(ui: &mut egui::Ui, path: &Path) -> io::Result<()> {
	if let Some(name) = path.file_name() {
		if path.is_dir() {
			egui::CollapsingHeader::new(name.to_string_lossy()).show(ui, |ui| {
                let mut paths: Vec<_> = fs::read_dir(&path).expect("Failed to read dir").map(|r| r.unwrap()).collect();
                                              
                // Sort the vector using the custom sorting function
				paths.sort_by(|a, b| sort_directories_first(a, b));

				for result in paths {
					//let result = path_result.expect("Failed to get path");
					//let full_path = result.path();
					let _ = list_files(ui, &result.path());
				}
            });
		} else {
			ui.label(name.to_string_lossy());
        }
    }
    Ok(())
}


struct MyApp {
    picked_path: Option<String>,
    language: String,
    code: String,
    command: String,
    command_history: String,
}


impl Default for MyApp {
    fn default() -> Self {
        Self {
            picked_path: None,
            language: "rs".into(),
            code: "// write here".into(),
            command: "".into(),
            command_history: "Welcome master".into(),
        }
    }
}


impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.draw_tree_panel(ctx);
        self.draw_terminal_panel(ctx);
        self.draw_code_panel(ctx);
    }
}

impl MyApp {
    fn draw_tree_panel(&self, ctx: &egui::Context) {
        egui::SidePanel::left("file_tree_panel").show(ctx, |ui| {
			ui.heading("Bookshelves");
			let _ = list_files(ui, Path::new("/home/penwing/Documents/"));
			ui.separator();
		});
    }

    fn draw_terminal_panel(&mut self, ctx: &egui::Context) {
        egui::TopBottomPanel::bottom("terminal")
            .exact_height(TERMINAL_HEIGHT.clone())
            .show(ctx, |ui| {
                ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
                    ui.label("");
                    ui.horizontal(|ui| {
                        let Self { command, .. } = self;
                        ui.label(">");
                        let response = ui.add(egui::TextEdit::singleline(command).desired_width(f32::INFINITY).lock_focus(true));

                        if response.lost_focus() && ctx.input(|i| i.key_pressed(egui::Key::Enter)) {
                            self.command_history.push_str(&("\n".to_string() + &run_command(self.command.clone())));
                            self.command = "".into();
                            response.request_focus();
                        }
                    });
                    egui::ScrollArea::vertical().show(ui, |ui| {
						ui.separator();
                        ui.label(self.command_history.clone());
                    });
                });
            });
    }

    fn draw_code_panel(&mut self, ctx: &egui::Context) {
        egui::CentralPanel::default().show(ctx, |ui| {
            if ui.button("Open file…").clicked() {
                if let Some(path) = rfd::FileDialog::new().pick_file() {
                    self.picked_path = Some(path.display().to_string());
                    let file_path = Path::new(self.picked_path.as_deref().unwrap_or_default());
                    self.code = fs::read_to_string(file_path).expect("Should have been able to read the file");
                    self.language = file_path.to_str().unwrap().split('.').last().unwrap().into();
                    println!("{}", self.language);
                }
            }

            if let Some(picked_path) = &self.picked_path {
                ui.horizontal(|ui| {
                    ui.label("Picked file:");
                    ui.monospace(picked_path);
                });
            }

            let Self { language, code, .. } = self;
            let theme = egui_extras::syntax_highlighting::CodeTheme::from_memory(ui.ctx());
            let mut layouter = |ui: &egui::Ui, string: &str, wrap_width: f32| {
                let mut layout_job = egui_extras::syntax_highlighting::highlight(ui.ctx(), &theme, string, language);
                layout_job.wrap.max_width = wrap_width;
                ui.fonts(|f| f.layout_job(layout_job))
            };

            egui::ScrollArea::vertical().show(ui, |ui| {
                ui.add(
                    egui::TextEdit::multiline(code)
                        .font(egui::FontId::monospace(60.0)) // for cursor height
                        .code_editor()
                        .lock_focus(true)
                        .desired_rows(80)
                        .lock_focus(true)
                        .desired_width(f32::INFINITY)
                        .layouter(&mut layouter),
                );
            });
        });
    }
}

