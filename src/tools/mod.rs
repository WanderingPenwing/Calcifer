use std::{cmp::Ordering, path::PathBuf, path::Path, fs, fs::read_to_string, io::Write, path::Component, ffi::OsStr, fs::OpenOptions};
use crate::calcifer::code_editor::Syntax;
use eframe::egui;
use serde::{Serialize, Deserialize};
use crate::DISPLAY_PATH_DEPTH;
use toml::Value;

//my tools;
pub mod search;
pub mod confirm;
pub mod settings;
pub mod shortcuts;

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
	
	if let Some(parent_dir) = Path::new(file_path).parent() {
		fs::create_dir_all(parent_dir)?;
	}

	let mut file = OpenOptions::new()
		.write(true)
		.create(true)
		.truncate(true)
		.open(file_path)?;

	file.write_all(serialized_state.as_bytes())?;
	
	println!("Saved state at {}", file_path);

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

pub fn version() -> String {
	// Read the contents of the Cargo.toml file
	if !Path::new("Cargo.toml").exists() {
		return "".to_string()
	}
	let toml_content = fs::read_to_string("Cargo.toml").expect("Failed to read Cargo.toml");

	// Parse the TOML content
	let toml: Value = toml::from_str(&toml_content).expect("Failed to parse TOML");

	// Extract version information
	if let Some(package) = toml.get("package") {
		if let Some(version) = package.get("version") {
			if let Some(version_string) = version.as_str() {
				println!("Version: {}", version_string);
				return format!(" v{}", version_string)
			}
		}
	}
	return "".to_string()
}


#[cfg(test)]
mod tests;
