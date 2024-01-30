use crate::editor::{themes::DEFAULT_THEMES, ColorTheme};
use eframe::egui;

pub struct SettingsWindow {
    pub visible: bool,
    pub updated: bool,
    pub theme: ColorTheme,
}

impl SettingsWindow {
    pub fn new(theme: ColorTheme) -> Self {
        Self {
            visible: false,
            updated: false,
            theme,
        }
    }

    pub fn show(&mut self, ctx: &egui::Context) {
        let mut visible = self.visible;
        egui::Window::new("Settings")
            .open(&mut visible) //I want it to be able to change its visibility (if user close manually)
            .vscroll(true)
            .hscroll(true)
            .show(ctx, |ui| self.ui(ui)); //but I want to edit the rest of the parameters and maybe close automatically
        self.visible = self.visible && visible;
    }

    fn ui(&mut self, ui: &mut egui::Ui) {
        ui.set_min_width(250.0);
        ui.horizontal(|ui| {
            ui.label("Theme ");

            let previous_theme = self.theme;
            egui::ComboBox::from_label("")
                .selected_text(self.theme.name.to_string())
                .show_ui(ui, |ui| {
                    ui.style_mut().wrap = Some(false);
                    ui.set_min_width(60.0);
                    for theme in DEFAULT_THEMES {
                        ui.selectable_value(&mut self.theme, theme, theme.name);
                    }
                });
            if self.theme != previous_theme {
                self.updated = true;
            }
        });
    }
}
