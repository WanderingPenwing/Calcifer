//use eframe::egui;
//use std::io;
use std::{process::Command, cmp::Ordering, env, path::PathBuf};
use egui_code_editor::Syntax;
//use std::fs;

pub mod themes;

#[derive(Debug, PartialEq, Eq)]
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
    pub fn from_n(n : usize) -> TabNumber {
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
    pub fn to_n(&self) -> usize {
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

#[derive(Clone)]
pub struct Tab {
	pub path : PathBuf,
	pub code : String,
	pub language : String,
	pub saved : bool,
	pub history : Vec<String>,
}

impl Default for Tab {
	fn default() -> Self {
		Self {
			path: "untitled".into(),
			code: "// Hello there, Master".into(),
			language: "rs".into(),
			saved: false,
			history: vec![],
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


pub fn loaded() {
	println!("Tools loaded");
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
	let output = Command::new("sh")
        .arg("-c")
        .arg(cmd.clone())
        .output()
        .expect("failed to execute process");
	
	entry.command = cmd;
	entry.output = (&String::from_utf8_lossy(&output.stdout)).to_string();
	entry.error = (&String::from_utf8_lossy(&output.stdout)).to_string();
	
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
