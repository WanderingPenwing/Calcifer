use std::{env, process::Command, cmp::Ordering, path::PathBuf, path::Path, fs::read_to_string, fs::write, path::Component, ffi::OsStr};
use crate::calcifer::code_editor::Syntax;
use eframe::egui;
use egui::text_edit::CCursorRange;
use serde::{Serialize, Deserialize};

//pub mod themes;
pub mod search;


pub trait View {
	fn ui(&mut self, ui: &mut egui::Ui, tabs: &mut Vec<Tab>, selected_tab: &mut TabNumber);
}

/// Something to view
pub trait Demo {
	/// Is the demo enabled for this integraton?
	fn is_enabled(&self, _ctx: &egui::Context) -> bool {
		true
	}

	/// `&'static` so we can also use it as a key to store open/close state.
	fn name(&self) -> &str; //'static 

	/// Show windows, etc
	fn show(&mut self, ctx: &egui::Context, open: &mut bool, tabs: &mut Vec<Tab>, selected_tab: &mut TabNumber);
}


#[derive(Debug, PartialEq, Eq, Clone)]
pub enum TabNumber {
	None,
	Open,
	Zero,
	One,
	Two,
	Three,
	Four,
	Five,
	Six,
	Seven,
}


impl TabNumber {
	pub fn from_index(n : usize) -> TabNumber {
		match n {
			0 => TabNumber::Zero,
			1 => TabNumber::One,
			2 => TabNumber::Two,
			3 => TabNumber::Three,
			4 => TabNumber::Four,
			5 => TabNumber::Five,
			6 => TabNumber::Six,
			7 => TabNumber::Seven,
			_ => TabNumber::None,
		}
	}
	pub fn to_index(&self) -> usize {
		match self {
			TabNumber::Zero => 0,
			TabNumber::One => 1,
			TabNumber::Two => 2,
			TabNumber::Three => 3,
			TabNumber::Four => 4,
			TabNumber::Five => 5,
			TabNumber::Six => 6,
			TabNumber::Seven => 7,
			_ => 0,
		}
	}
}

#[derive(Clone, PartialEq)]
pub struct Tab {
	pub path : PathBuf,
	pub code : String,
	pub language : String,
	pub saved : bool,
	pub scroll_offset : f32,
	pub last_cursor : Option<CCursorRange>,
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
	pub fn get_name(&self) -> String {
		self.path.file_name().expect("Could not get Tab Name").to_string_lossy().to_string()
	}
}


pub struct CommandEntry {
	pub env : String,
	pub command : String,
	pub output : String,
	pub error : String,
}


impl Default for CommandEntry {
	fn default() -> Self {
		Self {
			env: env::current_dir().expect("Could not find Shell Environnment").file_name().expect("Could not get Shell Environnment Name").to_string_lossy().to_string(),
			command : "".into(),
			output : "".into(),
			error : "".into(),
		}
	}
}


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


pub fn loaded() {
	println!("Tools loaded");
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


pub fn run_command(cmd : String) -> CommandEntry {
	let mut entry = CommandEntry::default();
	if &cmd[..2] == "cd" {
		let path_append = cmd[3..].replace("~", "/home/penwing");
		let path = Path::new(&path_append);
		entry.command = cmd;
		
		if format!("{}", path.display()) != "/" {
			if !env::set_current_dir(path).is_ok() {
				entry.error = format!("Could not find path : {}", path.display());
			}
		}
	} else {
		let output = Command::new("sh")
			.arg("-c")
			.arg(cmd.clone())
			.output()
			.expect("failed to execute process");
		
		entry.command = cmd;
		entry.output = (&String::from_utf8_lossy(&output.stdout)).to_string();
		entry.error = (&String::from_utf8_lossy(&output.stderr)).to_string();
	}
	
	entry
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


pub fn format_path(path: &Path, n_parents: usize) -> String {
    let components: Vec<&OsStr> = path
        .components()
		.rev()
        .take(n_parents + 1) // Take up to three components (root, parent, file/folder)
        .filter_map(|component| match component {
            Component::RootDir | Component::CurDir => None,
            _ => Some(component.as_os_str()),
        })
        .collect();

    format!("{}>", components.iter().rev().map(|&c| c.to_string_lossy()).collect::<Vec<_>>().join("/"))
}


#[cfg(test)]
mod tests;