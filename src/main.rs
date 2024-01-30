use eframe::egui;
use egui::{
    FontFamily::Proportional,
    FontId,
    TextStyle::{Body, Button, Heading, Monospace, Small},
};
use homedir::get_my_home;
use std::{ops::Range, path::PathBuf, sync::Arc, thread, time};

mod core;
mod editor;
mod panels;
mod sub_windows;

#[cfg(debug_assertions)]
const TITLE: &str = " debug";

#[cfg(not(debug_assertions))]
const TITLE: &str = "";

const ALLOWED_FILE_EXTENSIONS: [&str; 6] = ["", "rs", "toml", "txt", "project", "sh"];
const PROJECT_EXTENSION: &str = "project";
const TERMINAL_HEIGHT: f32 = 200.0;
const TERMINAL_RANGE: Range<f32> = 100.0..500.0;
const RED: egui::Color32 = egui::Color32::from_rgb(235, 108, 99);
const TIME_LABELS: [&str; 7] = [
    "input", "settings", "tree", "terminal", "tabs", "content", "windows",
];
const MAX_FPS: f32 = 30.0;
const DISPLAY_PATH_DEPTH: usize = 3;
const MAX_TABS: usize = 20;

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

    eframe::run_native(
        &format!("Calcifer{}", TITLE),
        options,
        Box::new(move |_cc| Box::from(Calcifer::from_app_state(app_state))),
    )
}

struct Calcifer {
    selected_tab: panels::TabNumber,
    tabs: Vec<panels::Tab>,

    command: String,
    command_history: Vec<panels::CommandEntry>,

    theme: editor::ColorTheme,
    font_size: f32,

    project_mode: bool,

    home: PathBuf,
    tree_dir_opened: Vec<String>,
    file_tree: Option<panels::FileEntry>,

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
            selected_tab: panels::TabNumber::from_index(0),
            tabs: vec![panels::Tab::default()],

            command: String::new(),
            command_history: Vec::new(),

            theme: editor::themes::DEFAULT_THEMES[0],
            font_size: 14.0,

            project_mode: true,

            home: get_my_home().unwrap().unwrap(),
            tree_dir_opened: vec![],
            file_tree: None,

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
            (Heading, FontId::new(self.font_size * 1.6, Proportional)),
            (Body, FontId::new(self.font_size, Proportional)),
            (Monospace, FontId::new(self.font_size, Proportional)),
            (Button, FontId::new(self.font_size, Proportional)),
            (Small, FontId::new(self.font_size, Proportional)),
        ]
        .into();
        ctx.set_style(style);

        if ctx.input(|i| i.key_pressed(egui::Key::R) && i.modifiers.ctrl)
            && !self.refresh_confirm.visible
        {
            if self.tabs[self.selected_tab.to_index()].saved {
                self.tabs[self.selected_tab.to_index()].refresh();
            } else {
                self.refresh_confirm.ask();
            }
        }

        if ctx.input(|i| i.key_pressed(egui::Key::S) && i.modifiers.ctrl) {
            self.handle_save_file(self.save_tab());
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
            self.font_size = (self.font_size * 1.1).min(30.0);
        }

        if ctx.input(|i| i.zoom_delta() < 1.0) {
            self.font_size = (self.font_size / 1.1).max(10.0);
        }

        if ctx.input(|i| i.key_pressed(egui::Key::F) && i.modifiers.ctrl) {
            self.search_menu.visible = !self.search_menu.visible;
            self.search_menu.initialized = !self.search_menu.visible;
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
    }

    fn on_exit(&mut self, _gl: std::option::Option<&eframe::glow::Context>) {
        self.save_state();
    }
}

fn save_path() -> PathBuf {
    if TITLE.is_empty() {
        get_my_home()
            .unwrap()
            .unwrap()
            .as_path()
            .join(".calcifer")
            .join("save.json")
            .to_path_buf()
    } else {
        get_my_home()
            .unwrap()
            .unwrap()
            .as_path()
            .join(".calcifer")
            .join("debug")
            .join("save.json")
            .to_path_buf()
    }
}
