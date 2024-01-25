use eframe::egui::text_edit::CCursorRange;
use std::{fs::read_to_string, path::PathBuf};

use crate::MAX_TABS;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum TabNumber {
    Open,
    Number(u8), // Using a range for numeric values
}

impl TabNumber {
    pub fn from_index(n: usize) -> TabNumber {
        match n {
            0..=MAX_TABS => TabNumber::Number(n as u8),
            _ => TabNumber::Number(0),
        }
    }

    pub fn to_index(&self) -> usize {
        match self {
            TabNumber::Number(n) => *n as usize,
            _ => 0,
        }
    }
}

#[derive(Clone, PartialEq)]
pub struct Tab {
    pub path: PathBuf,
    pub code: String,
    pub language: String,
    pub saved: bool,
    pub scroll_offset: f32,
    pub last_cursor: Option<CCursorRange>,
}

impl Default for Tab {
    fn default() -> Self {
        Self {
            path: "untitled".into(),
            code: "// Hello there, Master".into(),
            language: "rs".into(),
            saved: false,
            scroll_offset: 0.0,
            last_cursor: None,
        }
    }
}

impl Tab {
    pub fn new(path: PathBuf) -> Self {
        Self {
            path: path.clone().into(),
            code: read_to_string(path.clone())
                .expect("Not able to read the file")
                .replace(&" ".repeat(4), "\t"),
            language: path.to_str().unwrap().split('.').last().unwrap().into(),
            saved: true,
            scroll_offset: 0.0,
            last_cursor: None,
        }
    }
    pub fn get_name(&self) -> String {
        self.path
            .file_name()
            .expect("Could not get Tab Name")
            .to_string_lossy()
            .to_string()
    }

    pub fn refresh(&mut self) {
        self.code = read_to_string(self.path.clone())
            .expect("Not able to read the file")
            .replace(&" ".repeat(4), "\t");
        self.saved = true;
        println!("refreshed {}", self.path.display());
    }
}
