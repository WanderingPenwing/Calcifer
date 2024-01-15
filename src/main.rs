
mod tools;

use eframe::egui;
use tools::Demo;


fn main() -> Result<(), eframe::Error> {
	tools::code_editor::linked();
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1200.0, 800.0]) // wide enough for the drag-drop overlay text
            .with_drag_and_drop(true),
        ..Default::default()
    };
    eframe::run_native(
        "Calcifer",
        options,
        Box::new(|_cc| Box::<MyApp>::default()),
    )
}

//#[derive(Default)]
struct MyApp {
    picked_path: Option<String>,
    code_editor: tools::code_editor::CodeEditor,
    code_open: bool,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            picked_path: None,
            code_editor: tools::code_editor::CodeEditor::default(), // Initialize CodeEditor
            code_open: true,
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
		egui::SidePanel::left("my_left_panel").show(ctx, |ui| {
			ui.label("Tree ?");
		});
		
        egui::CentralPanel::default().show(ctx, |ui| {
            if ui.button("Open file…").clicked() {
                if let Some(path) = rfd::FileDialog::new().pick_file() {
                    self.picked_path = Some(path.display().to_string());
                }
            }
            
            ui.label("Code");

            if let Some(picked_path) = &self.picked_path {
                ui.horizontal(|ui| {
                    ui.label("Picked file:");
                    ui.monospace(picked_path);
                });
            }
            
            //~ let mut code_editor = CodeEditor::default();
				//~ code_editor.code = self.code.clone(); // Initialize code editor with MyApp's code
				//~ code_editor.language = self.language.clone();
				//~ code_editor.show(ctx, ui);
			//~ 
			self.code_editor.show(ctx, &mut self.code_open);
        });
        egui::TopBottomPanel::bottom("terminal").show(ctx, |ui| {
			ui.label("Terminal ?");
		});
    }
}

