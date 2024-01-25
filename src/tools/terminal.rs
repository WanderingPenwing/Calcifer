use crate::tools::format_path;
use std::{process::Command, env, path::Path};
use std::io::BufReader;
use std::io::BufRead;
use std::process::Stdio;
use nix::fcntl::{fcntl, FcntlArg, OFlag};
use std::os::unix::io::AsRawFd;


pub struct CommandEntry {
	pub env: String,
	pub command: String,
	pub output: String,
	pub error: String,
	pub output_buffer: BufReader<std::process::ChildStdout>,
	pub error_buffer: BufReader<std::process::ChildStderr>,
}


impl CommandEntry {
	pub fn new(command: String) -> Self {
		let (stdout_reader, stderr_reader) = execute(command.clone());

		CommandEntry {
			env: format_path(&env::current_dir().expect("Could not find Shell Environnment")),
			command,
			output: String::new(),
			error: String::new(),
			output_buffer: stdout_reader,
			error_buffer: stderr_reader,
		}
	}

	pub fn update(&mut self) {
		let mut output = String::new();
		let _ = self.output_buffer.read_line(&mut output);
		if !output.is_empty() {
			self.output += &output;
		}
		
		let mut error = String::new();
		let _ = self.error_buffer.read_line(&mut error);
		if !error.is_empty() {
			self.error += &error;
		}
	}
}


pub fn send_command(command: String) -> CommandEntry {
	if command.len() < 2 {
		return CommandEntry::new(command);
	}
	
	if &command[..2] != "cd" {
		return CommandEntry::new(command)
	}
	
	if command.len() < 4 {
		let mut entry = CommandEntry::new("echo Invalid cd, should provide path >&2".to_string());
		entry.command = command;
		return entry
	}
			
	let path_append = command[3..].replace("~", "/home/penwing");
	let path = Path::new(&path_append);
	
	if format!("{}", path.display()) == "/" {
		let mut entry = CommandEntry::new("echo Root access denied >&2".to_string());
		entry.command = command;
		return entry
	}
				
	if env::set_current_dir(path).is_ok() {
		let mut entry = CommandEntry::new(format!("echo Moved to : {}", path.display()));
		entry.command = command;
		return entry
	} else {
		let mut entry = CommandEntry::new(format!("echo Could not find path : {} >&2", path.display()));
		entry.command = command;
		return entry
	}
}


pub fn execute(command: String) -> (BufReader<std::process::ChildStdout>, BufReader<std::process::ChildStderr>) {
	let mut child = Command::new("sh")
		.arg("-c")
		.arg(command.clone())
		.stdout(Stdio::piped())
		.stderr(Stdio::piped())
		.spawn()
		.expect("failed to execute process");
	
	let stdout = child.stdout.take().unwrap();
	let stdout_fd = stdout.as_raw_fd();

	fcntl(stdout_fd, FcntlArg::F_SETFL(OFlag::O_NONBLOCK)).expect("Failed to set non-blocking mode");
	
	let stderr = child.stderr.take().unwrap();
	let stderr_fd = stderr.as_raw_fd();

	fcntl(stderr_fd, FcntlArg::F_SETFL(OFlag::O_NONBLOCK)).expect("Failed to set non-blocking mode");

	return (BufReader::new(stdout), BufReader::new(stderr));
}