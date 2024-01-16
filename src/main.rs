
//mod tools;

use eframe::egui;
use std::fs;
use std::path::Path;

const TERMINAL_HEIGHT : f32 = 200.0;


fn main() -> Result<(), eframe::Error> {
	//tools::code_editor::linked();
	
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
	cmd
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
            code: "// A very simple example\nfn main() {\n\tprintln!(\"Hello world!\");\n}\n".into(),
            command: "".into(),
            command_history: "Welcome master".into(),
        }
    }
}


impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
		
		//tree panel
		egui::SidePanel::left("tree").show(ctx, |ui| {
			ui.label("Tree ?");
		});
		
		//terminal panel
		egui::TopBottomPanel::bottom("terminal").exact_height(TERMINAL_HEIGHT.clone()).show(ctx, |ui| {
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
					ui.label(self.command_history.clone());
				});
			});
		});
		
		//code panel
        egui::CentralPanel::default().show(ctx, |ui| {
            if ui.button("Open file…").clicked() {
                if let Some(path) = rfd::FileDialog::new().pick_file() {
                    self.picked_path = Some(path.display().to_string());
                    let file_path = Path::new(self.picked_path.as_deref().unwrap_or_default());
                    self.code = fs::read_to_string(file_path).expect("Should have been able to read the file");
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
				let mut layout_job =
					egui_extras::syntax_highlighting::highlight(ui.ctx(), &theme, string, language);
				layout_job.wrap.max_width = wrap_width;
				ui.fonts(|f| f.layout_job(layout_job))
			};
			
			egui::ScrollArea::vertical().show(ui, |ui| {
				ui.add(
					egui::TextEdit::multiline(code)
						.font(egui::FontId::monospace(60.0)) // for cursor height
						.code_editor()
						.lock_focus(true)
						.desired_rows(20)
						.lock_focus(true)
						.desired_width(f32::INFINITY)
						.layouter(&mut layouter),
				);
			});
        });
    }
}

