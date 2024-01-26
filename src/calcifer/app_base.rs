use eframe::egui;
use egui::Color32;
use std::{cmp::min, ffi::OsStr, fs, io, path::Path, path::PathBuf};

use crate::tools;
use crate::Calcifer;
use crate::DEFAULT_THEMES;
use crate::MAX_TABS;
use crate::PATH_ROOT;
use crate::SAVE_PATH;
use crate::TIME_LABELS;
use tools::hex_str_to_color;

impl Calcifer {
    pub fn handle_confirm(&mut self) {
        if self.close_tab_confirm.proceed {
            self.close_tab_confirm.close();
            self.delete_tab(self.tab_to_close);
        }

        if self.refresh_confirm.proceed {
            self.refresh_confirm.close();
            self.tabs[self.selected_tab.to_index()].refresh();
        }
    }

    pub fn save_tab(&self) -> Option<PathBuf> {
        if self.tabs[self.selected_tab.to_index()]
            .path
            .file_name()
            .map_or(true, |name| name.to_string_lossy() == "untitled")
        {
            self.save_tab_as()
        } else {
            if let Err(err) = fs::write(
                &self.tabs[self.selected_tab.to_index()].path,
                &self.tabs[self.selected_tab.to_index()].code,
            ) {
                eprintln!("Error writing file: {}", err);
                return None;
            }
            Some(self.tabs[self.selected_tab.to_index()].path.clone())
        }
    }

    pub fn save_tab_as(&self) -> Option<PathBuf> {
        if let Some(path) = rfd::FileDialog::new()
            .set_directory(Path::new(&PATH_ROOT))
            .save_file()
        {
            if let Err(err) = fs::write(&path, &self.tabs[self.selected_tab.to_index()].code) {
                eprintln!("Error writing file: {}", err);
                return None;
            }
            return Some(path);
        }
        None
    }

    pub fn handle_save_file(&mut self, path_option: Option<PathBuf>) {
        if let Some(path) = path_option {
            println!("File saved successfully at: {:?}", path);
            self.tabs[self.selected_tab.to_index()].path = path;
            self.tabs[self.selected_tab.to_index()].saved = true;
        } else {
            println!("File save failed.");
        }
    }

    pub fn from_app_state(app_state: tools::AppState) -> Self {
        let mut new = Self {
            theme: DEFAULT_THEMES[min(app_state.theme, DEFAULT_THEMES.len() - 1)],
            tabs: Vec::new(),
            settings_menu: tools::settings::SettingsWindow::new(DEFAULT_THEMES[app_state.theme]),
            ..Default::default()
        };

        for path in app_state.tabs {
            if !path
                .file_name()
                .map_or(true, |name| name.to_string_lossy() == "untitled")
            {
                new.open_file(Some(&path));
            }
        }

        if new.tabs == vec![] {
            new.open_file(None);
        }

        new
    }

    pub fn save_state(&self) {
        let mut state_theme: usize = 0;
        if let Some(theme) = DEFAULT_THEMES.iter().position(|&r| r == self.theme) {
            state_theme = theme;
        }

        let mut state_tabs = vec![];

        for tab in &self.tabs {
            state_tabs.push(tab.path.clone());
        }
        let app_state = tools::AppState {
            tabs: state_tabs,
            theme: state_theme,
        };

        let _ = tools::save_state(&app_state, SAVE_PATH);
    }

    pub fn move_through_tabs(&mut self, forward: bool) {
        let new_index = if forward {
            (self.selected_tab.to_index() + 1) % self.tabs.len()
        } else {
            self.selected_tab
                .to_index()
                .checked_sub(1)
                .unwrap_or(self.tabs.len() - 1)
        };
        self.selected_tab = tools::TabNumber::from_index(new_index);
    }

    pub fn list_files(&mut self, ui: &mut egui::Ui, path: &Path) -> io::Result<()> {
        if path.file_name().is_none() {
            return Ok(());
        }

        let name = path
            .file_name()
            .unwrap_or_else(|| OsStr::new(""))
            .to_string_lossy()
            .into_owned();

        if !path.is_dir() {
            if ui.button(name).clicked() {
                self.open_file(Some(path));
            }
            return Ok(());
        }

        egui::CollapsingHeader::new(name).show(ui, |ui| match fs::read_dir(path) {
            Err(err) => {
                ui.label(format!("Error reading directory: {}", err));
            }
            Ok(entries) => {
                let mut paths: Vec<Result<fs::DirEntry, io::Error>> = entries
                    .map(|r| r.map_err(|e| io::Error::new(io::ErrorKind::Other, e)))
                    .collect();

                paths.sort_by(|a, b| match (a, b) {
                    (Ok(entry_a), Ok(entry_b)) => tools::sort_directories_first(entry_a, entry_b),
                    (Err(_), Ok(_)) => std::cmp::Ordering::Greater,
                    (Ok(_), Err(_)) => std::cmp::Ordering::Less,
                    (Err(_), Err(_)) => std::cmp::Ordering::Equal,
                });

                for result in paths {
                    match result {
                        Ok(entry) => {
                            let _ = self.list_files(ui, &entry.path());
                        }
                        Err(err) => {
                            ui.label(format!("Error processing directory entry: {}", err));
                        }
                    }
                }
            }
        });
        Ok(())
    }

    pub fn open_file(&mut self, path_option: Option<&Path>) {
        if self.tabs.len() < MAX_TABS {
            if let Some(path) = path_option {
                self.tabs.push(tools::Tab::new(path.to_path_buf()));
            } else {
                self.tabs.push(tools::Tab::default());
            }
            self.selected_tab = tools::TabNumber::from_index(self.tabs.len() - 1);
        }
    }

    pub fn delete_tab(&mut self, index: usize) {
        self.tabs.remove(index);
        self.selected_tab = tools::TabNumber::from_index(min(index, self.tabs.len() - 1));
    }

    pub fn toggle(&self, ui: &mut egui::Ui, display: bool, title: &str) -> bool {
        let bg_color: Color32;
        let text_color: Color32;

        if display {
            bg_color = hex_str_to_color(self.theme.functions);
            text_color = hex_str_to_color(self.theme.bg);
        } else {
            bg_color = hex_str_to_color(self.theme.bg);
            text_color = hex_str_to_color(self.theme.literals);
        };

        ui.style_mut().visuals.override_text_color = Some(text_color);

        if ui.add(egui::Button::new(title).fill(bg_color)).clicked() {
            return !display;
        }
        ui.style_mut().visuals.override_text_color = None;

        display
    }

    pub fn profiler(&self) -> String {
        if !self.profiler_visible {
            return "".to_string();
        }
        let combined_string: Vec<String> = TIME_LABELS
            .into_iter()
            .zip(self.time_watch.clone())
            .map(|(s, v)| format!("{} : {:.1} ms", s, v))
            .collect();

        let mut result = combined_string.join(" ;  ");
        result.push_str(&format!(
            "	total : {:.1} ms",
            self.time_watch.clone().iter().sum::<f32>()
        ));
        result
    }
}
