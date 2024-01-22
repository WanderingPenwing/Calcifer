
mod tools;
mod calcifer;

use eframe::egui;
use egui_code_editor::ColorTheme;
use std::{path::Path, sync::Arc};
use tools::Demo;

const TERMINAL_HEIGHT : f32 = 200.0;
const RED : egui::Color32 = egui::Color32::from_rgb(235, 108, 99);
const HISTORY_LENGTH : usize = 2;
const SAVE_PATH : &str = "calcifer_save.json";


fn main() -> Result<(), eframe::Error> {
	tools::loaded();
	
	let icon_data = tools::load_icon();
	
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1200.0, 800.0])
			.with_icon(Arc::new(icon_data)),
        ..Default::default()
    };
    
    let app_state: tools::AppState;
    // Attempt to load previous state
    if Path::new(SAVE_PATH).exists() {
        app_state = tools::load_state(SAVE_PATH).expect("Failed to load the save");
    } else {
        app_state = tools::AppState {
            tabs: vec![],
            theme: 0,
        };
    }

    eframe::run_native(
        "Calcifer v1.1",
        options,
        Box::new(move |_cc| Box::from(Calcifer::from_app_state(app_state))),
    )
}


struct Calcifer {
    selected_tab : tools::TabNumber,
    tabs: Vec<tools::Tab>,

    command: String,
    command_history: Vec<tools::CommandEntry>,

    theme: ColorTheme,

	search: tools::search::SearchWindow,
	searching: bool,
}


impl Default for Calcifer {
    fn default() -> Self {
		Self {
			selected_tab: tools::TabNumber::Zero,
			tabs: vec![tools::Tab::default()],

			command: String::new(),
			command_history: Vec::new(),

			theme: tools::themes::CustomColorTheme::fire(),

			search: tools::search::SearchWindow::default(),
			searching: false,
		}
	}
}


impl eframe::App for Calcifer {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
		if ctx.input( |i| i.key_pressed(egui::Key::S) && i.modifiers.ctrl) {
			self.handle_save_file(self.save_tab());
		}
		
		if ctx.input( |i| i.key_pressed(egui::Key::S) && i.modifiers.ctrl && i.modifiers.shift) {
			self.handle_save_file(self.save_tab_as());
		}
		
		if ctx.input( |i| i.key_pressed(egui::Key::Z) && i.modifiers.ctrl) {
			self.undo();
		}

		if ctx.input( |i| i.key_pressed(egui::Key::F) && i.modifiers.ctrl) {
			self.searching = !self.searching.clone();
		}
		
		self.draw_settings(ctx);
        self.draw_tree_panel(ctx);
        self.draw_terminal_panel(ctx);
        self.draw_tab_panel(ctx);
        self.draw_content_panel(ctx);

		if self.searching {
			self.search.show(ctx, &mut self.searching, &mut self.tabs, &mut self.selected_tab);
		}

		if !self.search.tab_selected && self.search.get_tab() != self.selected_tab {
			self.selected_tab = self.search.get_tab();
		}
		self.search.tab_selected = true;
	}
	
	fn on_exit(&mut self, _gl : std::option::Option<&eframe::glow::Context>) {
		self.save_state();
	}
}

