use crate::tools::format_path;
use std::{process::Command, env, path::Path};

#[derive(Clone, PartialEq)]
pub struct CommandEntry {
	pub env: String,
	pub command: String,
	pub output: String,
	pub error: String,
}


impl CommandEntry {
	pub fn new(command: String) -> Self {
		CommandEntry {
			env: format_path(&env::current_dir().expect("Could not find Shell Environnment")),
			command,
			output: String::new(),
			error: String::new(),
		}
	}

	pub fn run(&mut self) -> Self {
		let output = Command::new("sh")
			.arg("-c")
			.arg(self.command.clone())
			.output()
			.expect("failed to execute process");
		self.output = (&String::from_utf8_lossy(&output.stdout)).trim_end_matches('\n').to_string();
		self.error = (&String::from_utf8_lossy(&output.stderr)).trim_end_matches('\n').to_string();
		
		self.clone()
	}
}


pub fn run_command(command: String) -> CommandEntry {
	let mut entry = CommandEntry::new(command);

	if entry.command.len() < 2 {
		return entry.run();
	}
		
	if &entry.command[..2] != "cd" {
		return entry.run()
	}
	
	if entry.command.len() < 4 {
		entry.error = "Invalid cd, should provide path".to_string();
		return entry
	}
			
	let path_append = entry.command[3..].replace("~", "/home/penwing");
	let path = Path::new(&path_append);
	
	if format!("{}", path.display()) == "/" {
		entry.error = "Root access denied".to_string();
		return entry
	}
				
	if env::set_current_dir(path).is_ok() {
		entry.output = format!("Moved to : {}", path.display());
	} else {
		entry.error = format!("Could not find path : {}", path.display());
	}
	
	return entry
}