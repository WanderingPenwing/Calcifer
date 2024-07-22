use eframe::egui;
use egui::{
	FontFamily, FontId,
	TextStyle::{Body, Button, Heading, Monospace, Small},
};
use homedir::get_my_home;
use std::{ops::Range, path::PathBuf, sync::Arc, thread, time};
use std::env;
use std::time::Duration;

mod core;
mod editor;
mod panels;
mod sub_windows;

#[cfg(debug_assertions)]
const TITLE: &str = " debug";

#[cfg(not(debug_assertions))]
const TITLE: &str = "";

//const ALLOWED_FILE_EXTENSIONS: [&str; 14] = ["", "rs", "toml", "txt", "project", "sh", "md", "html", "js", "css", "php", "py", "kv", "nix"];
const PROJECT_EXTENSION: &str = "project";
const TERMINAL_HEIGHT: f32 = 200.0;
const TERMINAL_RANGE: Range<f32> = 100.0..600.0;
const RED: egui::Color32 = egui::Color32::from_rgb(235, 108, 99);
const TIME_LABELS: [&str; 7] = [
	"input", "settings", "tree", "terminal", "tabs", "content", "windows",
];
const ZOOM_FACTOR: f32 = 1.1;
const MAX_FPS: f32 = 30.0;
const DISPLAY_PATH_DEPTH: usize = 3;
const MAX_PROJECT_COLUMNS: usize = 8;
const RUNNING_COMMAND_REFRESH_DELAY: f32 = 0.2;

fn main() -> Result<(), eframe::Error> {
	let icon_data = core::load_icon().unwrap_or_default();

	let options = eframe::NativeOptions {
		viewport: egui::ViewportBuilder::default()
			.with_inner_size([1200.0, 800.0])
			.with_icon(Arc::new(icon_data)),
		..Default::default()
	};

	// Attempt to load previous state
	let app_state: core::AppState = if save_path().exists() {
		match core::load_state(save_path().as_path()) {
			Ok(app_state) => app_state,
			Err(_) => core::AppState::default(),
		}
	} else {
		core::AppState::default()
	};
	
	let args: Vec<String> = env::args().collect();
	let file_to_open = if args.len() > 1 {
		println!("Opening file: {}", args[1].clone());
		let mut path = env::current_dir().unwrap_or_default();
		path.push(args[1].clone());
		Some(path)
	} else {
		None
	};

	eframe::run_native(
		&format!("Calcifer{}", TITLE),
		options,
		Box::new(move |_cc| Box::from(Calcifer::from_app_state(app_state, file_to_open))),
	)
}

struct Calcifer {
	focused: bool,
	got_focus: bool,
	
	selected_tab: usize,
	tabs: Vec<panels::Tab>,

	command: String,
	command_history: Vec<panels::CommandEntry>,
	running_command: bool,

	theme: editor::ColorTheme,
	font_size: f32,
	zoom: f32,

	project_content: panels::Project,

	home: PathBuf,
	tree_dir_opened: Vec<String>,
	file_tree: Option<panels::FileEntry>,
	n_file_displayed: usize,

	tree_visible: bool,
	profiler_visible: bool,
	terminal_visible: bool,

	close_tab_confirm: sub_windows::ConfirmWindow,
	tab_to_close: usize,
	refresh_confirm: sub_windows::ConfirmWindow,
	exit_confirm: sub_windows::ConfirmWindow,

	search_menu: sub_windows::SearchWindow,
	settings_menu: sub_windows::SettingsWindow,
	shortcuts_menu: sub_windows::ShortcutsWindow,

	time_watch: Vec<f32>,
	next_frame: time::Instant,
}

impl Default for Calcifer {
	fn default() -> Self {
		Self {
			focused: true,
			got_focus: false,
			
			selected_tab: 0,
			tabs: vec![panels::Tab::default()],

			command: String::new(),
			command_history: Vec::new(),
			running_command: false,

			theme: editor::themes::DEFAULT_THEMES[0],
			font_size: 14.0,
			zoom: 1.0,

			project_content: panels::Project::new(),

			home: get_my_home().unwrap().unwrap(),
			tree_dir_opened: vec![],
			file_tree: None,
			n_file_displayed: 0,

			tree_visible: false,
			profiler_visible: false,
			terminal_visible: false,

			close_tab_confirm: sub_windows::ConfirmWindow::new(
				"You have some unsaved changes, Do you still want to close this document ?",
				"Confirm Close",
			),
			tab_to_close: 0,
			refresh_confirm: sub_windows::ConfirmWindow::new(
				"You have some unsaved changes, Do you still want to refresh this document ?",
				"Confirm Refresh",
			),
			exit_confirm: sub_windows::ConfirmWindow::new("", "Confirm Exit"),

			search_menu: sub_windows::SearchWindow::default(),
			settings_menu: sub_windows::SettingsWindow::new(editor::themes::DEFAULT_THEMES[0]),
			shortcuts_menu: sub_windows::ShortcutsWindow::new(),

			time_watch: vec![0.0; TIME_LABELS.len()],
			next_frame: time::Instant::now(),
		}
	}
}

impl eframe::App for Calcifer {
	fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
		thread::sleep(time::Duration::from_secs_f32(
			((1.0 / MAX_FPS) - self.next_frame.elapsed().as_secs_f32()).max(0.0),
		));
		self.next_frame = time::Instant::now();

		let mut watch = time::Instant::now();

		let mut style = (*ctx.style()).clone();
		style.text_styles = [
			(
				Heading,
				FontId::new(self.font_size * 1.6, FontFamily::Proportional),
			),
			(Body, FontId::new(self.font_size, FontFamily::Proportional)),
			(
				Monospace,
				FontId::new(self.font_size, FontFamily::Monospace),
			),
			(
				Button,
				FontId::new(self.font_size, FontFamily::Proportional),
			),
			(Small, FontId::new(self.font_size, FontFamily::Proportional)),
		]
		.into();
		ctx.set_style(style);

		if ctx.zoom_factor() != self.zoom {
			ctx.set_zoom_factor(self.zoom);
		}

		if ctx.input(|i| i.key_pressed(egui::Key::R) && i.modifiers.ctrl)
			&& !self.refresh_confirm.visible
		{
			if self.tabs[self.selected_tab].saved {
				self.tabs[self.selected_tab].refresh();
			} else {
				self.refresh_confirm.ask();
			}
		}

		if ctx.input(|i| i.key_pressed(egui::Key::Enter)) && ctx.memory(|m| m.focus() == None)
			&& self.tabs[self.selected_tab].language == PROJECT_EXTENSION
		{
			self.project_content.item_window.visible = true;
		}
		
		if ctx.input(|i| i.key_pressed(egui::Key::S) && i.modifiers.ctrl) {
			self.handle_save_file(self.save_tab());
		}

		if ctx.input(|i| i.key_pressed(egui::Key::T) && i.modifiers.ctrl) {
			self.file_tree = None;
			self.tree_dir_opened = vec![];
		}

		if ctx.input(|i| i.key_pressed(egui::Key::S) && i.modifiers.ctrl && i.modifiers.shift) {
			self.handle_save_file(self.save_tab_as());
		}

		if ctx.input(|i| i.key_pressed(egui::Key::ArrowLeft) && i.modifiers.alt) {
			self.move_through_tabs(false);
		}

		if ctx.input(|i| i.key_pressed(egui::Key::ArrowRight) && i.modifiers.alt) {
			self.move_through_tabs(true);
		}

		if ctx.input(|i| i.zoom_delta() > 1.0) {
			self.zoom = (self.zoom*ZOOM_FACTOR).min(10.0);
		}

		if ctx.input(|i| i.zoom_delta() < 1.0) {
			self.zoom = (self.zoom/ZOOM_FACTOR).max(0.1);
		}

		if ctx.input(|i| i.key_pressed(egui::Key::F) && i.modifiers.ctrl) {
			self.search_menu.visible = !self.search_menu.visible;
			self.search_menu.initialized = !self.search_menu.visible;
		}
		
		self.got_focus = false;
		if ctx.input(|i| !i.viewport().focused.unwrap_or_default()) {
			self.focused = false;
		} else {
			if !self.focused {
				self.got_focus = true;
			}
			self.focused = true;
		}

		if ctx.input(|i| i.viewport().close_requested()) {
			let mut unsaved_tabs: Vec<usize> = vec![];
			for (index, tab) in self.tabs.iter().enumerate() {
				if !tab.saved {
					unsaved_tabs.push(index);
				}
			}
			if !unsaved_tabs.is_empty() {
				let mut unsaved_tabs_names: String = "".to_string();
				for index in unsaved_tabs.iter() {
					unsaved_tabs_names.push_str(&self.tabs[*index].get_name());
				}
				egui::Context::send_viewport_cmd(ctx, egui::ViewportCommand::CancelClose);
				self.exit_confirm.prompt = format!(
					"You have some unsaved changes :\n{}\nDo you still want to exit ?",
					unsaved_tabs_names
				);
				self.exit_confirm.ask();
			}
		}

		self.time_watch[0] = watch.elapsed().as_micros() as f32 / 1000.0;
		watch = time::Instant::now();

		self.draw_settings(ctx);

		self.time_watch[1] = watch.elapsed().as_micros() as f32 / 1000.0;
		watch = time::Instant::now();

		self.draw_tree_panel(ctx);

		self.time_watch[2] = watch.elapsed().as_micros() as f32 / 1000.0;
		watch = time::Instant::now();

		self.draw_bottom_tray(ctx);
		self.draw_terminal_panel(ctx);

		self.time_watch[3] = watch.elapsed().as_micros() as f32 / 1000.0;
		watch = time::Instant::now();

		self.draw_tab_panel(ctx);

		self.time_watch[4] = watch.elapsed().as_micros() as f32 / 1000.0;
		watch = time::Instant::now();

		self.draw_content_panel(ctx);

		self.time_watch[5] = watch.elapsed().as_micros() as f32 / 1000.0;
		watch = time::Instant::now();

		self.draw_windows(ctx);

		self.time_watch[6] = watch.elapsed().as_micros() as f32 / 1000.0;

		if self.running_command {
			egui::Context::request_repaint_after(ctx, Duration::from_secs_f32(RUNNING_COMMAND_REFRESH_DELAY));
		}
	}

	fn on_exit(&mut self, _gl: std::option::Option<&eframe::glow::Context>) {
		self.save_state();
	}
}

//save path
fn save_path() -> PathBuf {
	if TITLE.is_empty() {
		get_my_home()
			.unwrap()
			.unwrap()
			.as_path()
			.join(".config")
			.join("calcifer")
			.join("save.json")
			.to_path_buf()
	} else {
		get_my_home()
			.unwrap()
			.unwrap()
			.as_path()
			.join(".config")
			.join("calcifer")
			.join("debug_save.json")
			.to_path_buf()
	}
}
