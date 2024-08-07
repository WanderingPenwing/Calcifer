use eframe::egui;

pub struct ShortcutsWindow {
    pub visible: bool,
}

impl ShortcutsWindow {
    pub fn new() -> Self {
        Self { visible: false }
    }

    pub fn show(&mut self, ctx: &egui::Context) {
        let mut visible = self.visible;
        egui::Window::new("Shortcuts")
            .open(&mut visible)
            .vscroll(true)
            .hscroll(true)
            .show(ctx, |ui| self.ui(ui));
        self.visible = self.visible && visible;
    }

    fn ui(&mut self, ui: &mut egui::Ui) {
        ui.set_min_width(250.0);
        ui.label("Ctrl+S : save file");
        ui.label("Ctrl+Shift+S : save file as");
        ui.label("Ctrl+R : reload file");
        ui.separator();
        ui.label("Ctrl+F : open search window");
        ui.separator();
        ui.label("Ctrl+T : reload tree");
        ui.separator();
        ui.label("Ctrl+Z : undo");
        ui.label("Ctrl+Y : redo");
        ui.label("Tab on selection : add indent of selection");
        ui.label("Shift+Tab on selection : remove indent of selection");
        ui.label("Ctrl+E : comment selection");
        ui.separator();
        ui.label("Alt+Arrows : move between tabs");
        ui.separator();
        ui.label("Enter (project_mode) : edit item");
        ui.label("Arrows (project_mode) : change selected item");
        ui.label("Shift+Arrows (project_mode) : move selected item");
    }
}
