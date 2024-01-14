use eframe::egui;
use egui_extras::syntax_highlighting;

fn main() -> Result<(), eframe::Error> {
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

struct CodeEditor {
    language: String,
    code: String,
}

impl Default for CodeEditor {
    fn default() -> Self {
        Self {
            language: "rs".into(),
            code: "// A very simple example\n\
                   fn main() {\n\
                   \tprintln!(\"Hello world!\");\n\
                   }\n\
                   "
            .into(),
        }
    }
}

impl CodeEditor {
    fn ui(&mut self, ui: &mut egui::Ui) {
        // The contents of the ui method from your reference go here
        // ...

        // You can also replace `self.code` with `&mut self.code` in the method
        // to directly modify the code in the CodeEditor instance.
    }

    fn show(&mut self, ctx: &egui::Context, ui: &mut egui::Ui) {
        // The contents of the show method from your reference go here
        // ...
        self.ui(ui);
    }
}

#[derive(Default)]
struct MyApp {
    picked_path: Option<String>,
    code: String,
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
            
            let mut code_editor = CodeEditor::default();
				code_editor.code = self.code.clone(); // Initialize code editor with MyApp's code
				code_editor.show(ctx, ui);
			});
        
        egui::TopBottomPanel::bottom("terminal").show(ctx, |ui| {
			ui.label("Terminal ?");
		});
    }
}

