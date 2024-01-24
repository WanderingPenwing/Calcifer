use std::{cmp::Ordering, path::PathBuf, path::Path, fs::read_to_string, fs::write, path::Component, ffi::OsStr};
use crate::calcifer::code_editor::Syntax;
use eframe::egui;
use serde::{Serialize, Deserialize};
use crate::DISPLAY_PATH_DEPTH;

//my tools;
pub mod search;
pub mod confirm;

pub mod terminal;
pub use terminal::*;

pub mod tabs;
pub use tabs::*;


#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct AppState {
	pub tabs: Vec<PathBuf>,
	pub theme: usize,
}


pub fn save_state(state: &AppState, file_path: &str) -> Result<(), std::io::Error> {
	let serialized_state = serde_json::to_string(state)?;

	write(file_path, serialized_state)?;

	Ok(())
}


pub fn load_state(file_path: &str) -> Result<AppState, std::io::Error> {
	let serialized_state = read_to_string(file_path)?;

	Ok(serde_json::from_str(&serialized_state)?)
}


pub fn load_icon() -> egui::IconData {
	let (icon_rgba, icon_width, icon_height) = {
		let icon = include_bytes!("../../assets/icon.png");
		let image = image::load_from_memory(icon)
			.expect("Failed to open icon path")
			.into_rgba8();
		let (width, height) = image.dimensions();
		let rgba = image.into_raw();
		(rgba, width, height)
	};
	
	egui::IconData {
		rgba: icon_rgba,
		width: icon_width,
		height: icon_height,
	}
}


pub fn to_syntax(language : &str) -> Syntax {
	match language {
		"py" => Syntax::python(),
		"rs" => Syntax::rust(),
		_ => Syntax::shell(),
	}
}


pub fn sort_directories_first(a: &std::fs::DirEntry, b: &std::fs::DirEntry) -> Ordering {
	let a_is_dir = a.path().is_dir();
	let b_is_dir = b.path().is_dir();

	// Directories come first, then files
	if a_is_dir && !b_is_dir {
		Ordering::Less
	} else if !a_is_dir && b_is_dir {
		Ordering::Greater
	} else {
		// Both are either directories or files, sort alphabetically
		a.path().cmp(&b.path())
	}
}


pub fn format_path(path: &Path) -> String {
	let components: Vec<&OsStr> = path
		.components()
		.rev()
		.take(DISPLAY_PATH_DEPTH)
		.filter_map(|component| match component {
			Component::RootDir | Component::CurDir => None,
			_ => Some(component.as_os_str()),
		})
		.collect();

	format!("{}>", components.iter().rev().map(|&c| c.to_string_lossy()).collect::<Vec<_>>().join("/"))
}


#[cfg(test)]
mod tests;