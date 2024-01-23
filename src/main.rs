mod tools;
mod calcifer;

use eframe::egui;
use calcifer::code_editor::ColorTheme;
use std::{path::Path, sync::Arc, time::Instant};

use tools::Demo;
use calcifer::code_editor::themes::DEFAULT_THEMES;

const TERMINAL_HEIGHT : f32 = 200.0;
const RED : egui::Color32 = egui::Color32::from_rgb(235, 108, 99);
const SAVE_PATH : &str = "calcifer_save.json";
const TIME_LABELS : [&str; 5] = ["settings", "tree", "terminal", "tabs", "content"];


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
		"Calcifer v1.0.3",
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
	
	debug_display: bool,
	time_watch: Vec<f32>,
}


impl Default for Calcifer {
	fn default() -> Self {
		Self {
			selected_tab: tools::TabNumber::Zero,
			tabs: vec![tools::Tab::default()],

			command: String::new(),
			command_history: Vec::new(),

			theme: DEFAULT_THEMES[0],

			search: tools::search::SearchWindow::default(),
			searching: false,
			
			debug_display: false,
			time_watch: vec![0.0; TIME_LABELS.len()],
		}
	}
}


impl eframe::App for Calcifer {
	fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
		let mut watch = Instant::now();
		
		if ctx.input( |i| i.key_pressed(egui::Key::S) && i.modifiers.ctrl) {
			self.handle_save_file(self.save_tab());
		}
		
		if ctx.input( |i| i.key_pressed(egui::Key::T) && i.modifiers.ctrl) {
			self.indent_with_tabs();
		}
		
		if ctx.input( |i| i.key_pressed(egui::Key::S) && i.modifiers.ctrl && i.modifiers.shift) {
			self.handle_save_file(self.save_tab_as());
		}

		if ctx.input( |i| i.key_pressed(egui::Key::F) && i.modifiers.ctrl) {
			self.searching = !self.searching.clone();
			if self.searching {
				self.search.initialized = false;
			}
		}
		
		self.draw_settings(ctx);
		
		self.time_watch[0] = watch.elapsed().as_micros() as f32 / 1000.0;
		watch = Instant::now();
		
		self.draw_tree_panel(ctx);
		
		self.time_watch[1] = watch.elapsed().as_micros() as f32 / 1000.0;
		watch = Instant::now();
		
		self.draw_terminal_panel(ctx);
		
		self.time_watch[2] = watch.elapsed().as_micros() as f32 / 1000.0;
		watch = Instant::now();
		
		self.draw_tab_panel(ctx);
		
		self.time_watch[3] = watch.elapsed().as_micros() as f32 / 1000.0;
		watch = Instant::now();
		
		self.draw_content_panel(ctx);
		
		self.time_watch[4] = watch.elapsed().as_micros() as f32 / 1000.0;

		if self.searching {
			self.search.show(ctx, &mut self.searching, &mut self.tabs, &mut self.selected_tab);
		}

		if !self.search.tab_selected && self.search.get_tab() != self.selected_tab {
			self.selected_tab = self.search.get_tab();
			println!("changed tab to {}", self.selected_tab.to_index());
		}
		self.search.tab_selected = true;
	}
	
	fn on_exit(&mut self, _gl : std::option::Option<&eframe::glow::Context>) {
		self.save_state();
	}
}

