use eframe::egui;
use std::{
    cmp::min,
    sync::atomic::{AtomicUsize, Ordering},
};

use crate::core::hex_str_to_color;
use crate::editor::ColorTheme;
use crate::sub_windows;
use crate::MAX_PROJECT_COLUMNS;

pub struct Project {
    pub categories: Vec<Category>,
    pub selected_item: Location,
    pub item_window: sub_windows::ProjectItemWindow,
    was_moving: bool,
}

impl Project {
    pub fn new() -> Self {
        Self {
            categories: vec![Category::create()],
            selected_item: Location::zero(),
            was_moving: false,
            item_window: sub_windows::ProjectItemWindow::new(),
        }
    }

    fn add_category(&mut self) {
        let last = self.categories.len() - 1;
        self.categories[last].initialize();
        if self.categories.len() < MAX_PROJECT_COLUMNS {
            self.categories.push(Category::create());
        }
    }

    fn delete_category(&mut self, index: usize) {
        self.categories.remove(index);
        let last = self.categories.len() - 1;
        if self.categories[last].name != "+" {
            self.categories.push(Category::create());
        }
    }
}

#[derive(Clone)]
pub struct Category {
    name: String,
    pub content: Vec<Item>,
}

impl Category {
    fn create() -> Self {
        Self {
            name: "+".into(),
            content: vec![],
        }
    }

    fn initialize(&mut self) {
        self.name = "untitled".into();
    }
}

#[derive(Clone, Hash)]
pub struct Item {
    pub name: String,
    pub description: String,
    id: usize,
}

impl Item {
    fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            description: "// Hello there".to_string(),
            id: get_id(),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Location {
    pub category: usize,
    pub row: usize,
}

impl Location {
    fn zero() -> Self {
        Self {
            category: 0,
            row: 0,
        }
    }
}

fn get_id() -> usize {
    static COUNTER: AtomicUsize = AtomicUsize::new(1);
    COUNTER.fetch_add(1, Ordering::Relaxed)
}

pub fn draw_project(ui: &mut egui::Ui, theme: ColorTheme, project: &mut Project) {
    ui.columns(MAX_PROJECT_COLUMNS, |uis| {
        for (category_index, category) in project.categories.clone().into_iter().enumerate() {
            let ui = &mut uis[category_index];

            if category.name == "+" {
                if ui.add(egui::Button::new("+")).clicked() {
                    project.add_category();
                }
                continue;
            } else {
                let response = ui.add(
                    egui::TextEdit::singleline(&mut project.categories[category_index].name)
                        .desired_width(f32::INFINITY),
                );
                if response.lost_focus() && project.categories[category_index].name.is_empty() {
                    project.delete_category(category_index);
                }
            }

            ui.separator();

            for (item_index, item) in category.content.iter().enumerate() {
                if project.selected_item
                    == (Location {
                        category: category_index,
                        row: item_index,
                    })
                {
                    ui.style_mut().visuals.override_text_color = Some(hex_str_to_color(theme.bg));
                    ui.add(
                        egui::Button::new(item.name.clone())
                            .fill(hex_str_to_color(theme.functions)),
                    );
                } else {
                    ui.style_mut().visuals.override_text_color =
                        Some(hex_str_to_color(theme.literals));
                    if ui
                        .add(egui::Button::new(item.name.clone()).fill(hex_str_to_color(theme.bg)))
                        .clicked()
                    {
                        project.selected_item = Location {
                            category: category_index,
                            row: item_index,
                        };
                    }
                }
            }

            if category.name != "+" {
                if ui.add(egui::Button::new("+")).clicked() {
                    project.categories[category_index]
                        .content
                        .push(Item::new("item"));
                }
            }
        }
    });

    let mut moved = false;
    let category = project.selected_item.category.clone();
    let row = project.selected_item.row.clone();

    if ui.input(|i| i.key_pressed(egui::Key::ArrowLeft) && i.modifiers.shift)
        && project.selected_item.category > 0
    {
        moved = true;
        if !project.was_moving {
            let temp = project.categories[category].content[row].clone();
            project.categories[category - 1].content.push(temp);
            project.categories[category].content.remove(row);
            project.selected_item.category -= 1;
            project.selected_item.row = project.categories[category - 1].content.len() - 1;
        }
    } else if ui.input(|i| i.key_pressed(egui::Key::ArrowRight) && i.modifiers.shift)
        && project.selected_item.category < project.categories.len() - 2
    {
        moved = true;
        if !project.was_moving {
            let temp = project.categories[category].content[row].clone();
            project.categories[category + 1].content.push(temp);
            project.categories[category].content.remove(row);
            project.selected_item.category += 1;
            project.selected_item.row = project.categories[category + 1].content.len() - 1;
        }
    } else if ui.input(|i| i.key_pressed(egui::Key::ArrowUp) && i.modifiers.shift)
        && project.selected_item.row > 0
    {
        moved = true;
        if !project.was_moving {
            let temp = project.categories[category].content[row].clone();
            project.categories[category].content[row] =
                project.categories[category].content[row - 1].clone();
            project.categories[category].content[row - 1] = temp.clone();
            project.selected_item.row -= 1;
        }
    } else if ui.input(|i| i.key_pressed(egui::Key::ArrowDown) && i.modifiers.shift) {
        moved = true;
        if !project.was_moving {
            let temp = project.categories[category].content[row].clone();
            project.categories[category].content[row] =
                project.categories[category].content[row + 1].clone();
            project.categories[category].content[row + 1] = temp.clone();
            project.selected_item.row += 1;
        }
    } else if ui.input(|i| i.key_pressed(egui::Key::ArrowLeft))
        && project.selected_item.category > 0
    {
        moved = true;
        if !project.was_moving {
            project.selected_item.category -= 1;
            project.selected_item.row = min(
                project.categories[category].content.len() - 1,
                project.selected_item.row,
            );
        }
    } else if ui.input(|i| i.key_pressed(egui::Key::ArrowRight))
        && project.selected_item.category < project.categories.len() - 2
    {
        moved = true;
        if !project.was_moving {
            project.selected_item.category += 1;
            project.selected_item.row = min(
                project.categories[category].content.len() - 1,
                project.selected_item.row,
            );
        }
    } else if ui.input(|i| i.key_pressed(egui::Key::ArrowUp)) && project.selected_item.row > 0 {
        moved = true;
        if !project.was_moving {
            project.selected_item.row -= 1;
        }
    } else if ui.input(|i| i.key_pressed(egui::Key::ArrowDown))
        && project.selected_item.row < project.categories[category].content.len() - 1
    {
        moved = true;
        if !project.was_moving {
            project.selected_item.row += 1;
        }
    }

    project.was_moving = moved;
}
