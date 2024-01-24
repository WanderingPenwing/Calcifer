mod tools;
mod calcifer;

use eframe::egui;
use calcifer::code_editor::ColorTheme;
use std::{path::Path, sync::Arc, time, thread};

use calcifer::code_editor::themes::DEFAULT_THEMES;

const TERMINAL_HEIGHT : f32 = 200.0;
const RED : egui::Color32 = egui::Color32::from_rgb(235, 108, 99);
const SAVE_PATH : &str = "/home/penwing/Documents/.saves/calcifer_save.json";
const TIME_LABELS : [&str; 5] = ["settings", "tree", "terminal", "tabs", "content"];
const MAX_FPS : f32 = 30.0;
const PATH_ROOT : &str = "/home/penwing/Documents/";
const DISPLAY_PATH_DEPTH : usize = 3;
const MAX_TABS : usize = 20;


fn main() -> Result<(), eframe::Error> {
	let icon_data = tools::load_icon();
	
	//env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).
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
		"Calcifer v1.0.4",
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
	font_size: f32,

	search: tools::search::SearchWindow,
	
	debug_display: bool,
	time_watch: Vec<f32>,
	next_frame: time::Instant,
	
	tree_display: bool,
	
	close_tab_confirm: tools::confirm::ConfirmWindow,
	tab_to_close: usize,
	refresh_confirm: tools::confirm::ConfirmWindow,
}


impl Default for Calcifer {
	fn default() -> Self {
		Self {
			selected_tab: tools::TabNumber::from_index(0),
			tabs: vec![tools::Tab::default()],

			command: String::new(),
			command_history: Vec::new(),

			theme: DEFAULT_THEMES[0],
			font_size: 14.0,

			search: tools::search::SearchWindow::default(),
			
			debug_display: false,
			time_watch: vec![0.0; TIME_LABELS.len()],
			next_frame: time::Instant::now(),
			
			tree_display: false,
			
			close_tab_confirm: tools::confirm::ConfirmWindow::new("You have some unsaved changes, Do you still want to close this document ?", "Confirm Close"),
			tab_to_close: 0,
			refresh_confirm: tools::confirm::ConfirmWindow::new("You have some unsaved changes, Do you still want to refresh this document ?", "Confirm Refresh"),
		}
	}
}


impl eframe::App for Calcifer {
	fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
		thread::sleep(time::Duration::from_secs_f32(((1.0/MAX_FPS) - self.next_frame.elapsed().as_secs_f32()).max(0.0)));
		self.next_frame = time::Instant::now();
		
		let mut watch = time::Instant::now();
		
		if ctx.input( |i| i.key_pressed(egui::Key::T) && i.modifiers.ctrl) && !self.refresh_confirm.visible {
			if self.tabs[self.selected_tab.to_index()].saved {
				self.tabs[self.selected_tab.to_index()].refresh();
			} else {
				self.refresh_confirm.ask();
			}
		}
		
		if ctx.input( |i| i.key_pressed(egui::Key::S) && i.modifiers.ctrl) {
			self.handle_save_file(self.save_tab());
		}
		
		if ctx.input( |i| i.key_pressed(egui::Key::S) && i.modifiers.ctrl && i.modifiers.shift) {
			self.handle_save_file(self.save_tab_as());
		}
		
		if ctx.input( |i| i.key_pressed(egui::Key::ArrowLeft) && i.modifiers.alt) {
			self.move_through_tabs(false);
		}
		
		if ctx.input( |i| i.key_pressed(egui::Key::ArrowRight) && i.modifiers.alt) {
			self.move_through_tabs(true);
		}
		
		if ctx.input( |i| i.zoom_delta() > 1.0) {
			self.font_size = (self.font_size * 1.1).min(30.0);
		}
		
		if ctx.input( |i| i.zoom_delta() < 1.0) {
			self.font_size = (self.font_size / 1.1).max(10.0);
		}

		if ctx.input( |i| i.key_pressed(egui::Key::F) && i.modifiers.ctrl) {
			self.search.visible = !self.search.visible.clone();
			self.search.initialized = !self.search.visible.clone();
		}
		
		self.draw_settings(ctx);
		
		self.time_watch[0] = watch.elapsed().as_micros() as f32 / 1000.0;
		watch = time::Instant::now();
		
		self.draw_tree_panel(ctx);
		
		self.time_watch[1] = watch.elapsed().as_micros() as f32 / 1000.0;
		watch = time::Instant::now();
		
		self.draw_terminal_panel(ctx);
		
		self.time_watch[2] = watch.elapsed().as_micros() as f32 / 1000.0;
		watch = time::Instant::now();
		
		self.draw_tab_panel(ctx);
		
		self.time_watch[3] = watch.elapsed().as_micros() as f32 / 1000.0;
		watch = time::Instant::now();
		
		self.draw_content_panel(ctx);
		
		self.time_watch[4] = watch.elapsed().as_micros() as f32 / 1000.0;

		self.search.show(ctx, &mut self.tabs, &mut self.selected_tab);
		self.close_tab_confirm.show(ctx, &mut self.tabs, &mut self.selected_tab);
		self.refresh_confirm.show(ctx, &mut self.tabs, &mut self.selected_tab);
		
		self.handle_confirm();
	}
	
	fn on_exit(&mut self, _gl : std::option::Option<&eframe::glow::Context>) {
		self.save_state();
	}
}

