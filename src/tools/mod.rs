//use eframe::egui;
//use std::io;
use std::process::Command;
use std::cmp::Ordering;
use std::env;
//use std::path::Path;
//use std::fs;

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
