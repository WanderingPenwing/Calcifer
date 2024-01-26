use crate::tools::format_path;
use nix::fcntl::fcntl;
use nix::fcntl::FcntlArg;
use nix::fcntl::OFlag;
use std::io::BufRead;
use std::io::BufReader;
use std::io::Read;
use std::os::fd::AsRawFd;
use std::process::Stdio;
use std::{env, path::Path, path::PathBuf, process::Command};

pub struct Buffer {
	pub output_buffer: BufReader<std::process::ChildStdout>,
	pub error_buffer: BufReader<std::process::ChildStderr>,
}

pub struct CommandEntry {
	pub env: String,
	pub command: String,
	pub output: String,
	pub error: String,
	pub buffer: Option<Buffer>,
}

impl CommandEntry {
	pub fn new(command: String) -> Self {
		let (buffer, error) = match execute(command.clone()) {
			Ok(command_buffer) => (Some(command_buffer), String::new()),
			Err(err) => (None, format!("failed to get results: {}", err)),
		};

		CommandEntry {
			env: format_path(&env::current_dir().unwrap_or_else(|_| PathBuf::from("/"))),
			command,
			output: String::new(),
			error,
			buffer,
		}
	}

	pub fn update(&mut self) {
		if let Some(buffer) = &mut self.buffer {
			for line in buffer.output_buffer.by_ref().lines() {
				match line {
					Ok(line) => self.output += &format!("{}\n", line),
					Err(_) => return,
				}
			}

			for line in buffer.error_buffer.by_ref().lines() {
				match line {
					Ok(line) => self.error += &format!("{}\n", line),
					Err(_) => return,
				}
			}
		}
	}
}

pub fn send_command(command: String) -> CommandEntry {
	if command.len() < 2 {
		return CommandEntry::new(command);
	}

	if &command[..2] != "cd" {
		return CommandEntry::new(command);
	}

	if command.len() < 4 {
		let mut entry = CommandEntry::new("echo Invalid cd, should provide path >&2".to_string());
		entry.command = command;
		return entry;
	}

	let path_append = command[3..].replace('~', "/home/penwing");
	let path = Path::new(&path_append);

	if format!("{}", path.display()) == "/" {
		let mut entry = CommandEntry::new("echo Root access denied >&2".to_string());
		entry.command = command;
		return entry;
	}

	if env::set_current_dir(path).is_ok() {
		let mut entry = CommandEntry::new(format!("echo Moved to : {}", path.display()));
		entry.command = command;
		entry
	} else {
		let mut entry =
			CommandEntry::new(format!("echo Could not find path : {} >&2", path.display()));
		entry.command = command;
		entry
	}
}

pub fn execute(command: String) -> Result<Buffer, std::io::Error> {
	let mut child = Command::new("sh")
		.arg("-c")
		.arg(command.clone())
		.stdout(Stdio::piped())
		.stderr(Stdio::piped())
		.spawn()?;

	let stdout = child
		.stdout
		.take()
		.ok_or_else(|| std::io::Error::new(std::io::ErrorKind::Other, "Failed to open stdout"))?;
	let stderr = child
		.stderr
		.take()
		.ok_or_else(|| std::io::Error::new(std::io::ErrorKind::Other, "Failed to open stderr"))?;

	let stdout_fd = stdout.as_raw_fd();
	let stderr_fd = stderr.as_raw_fd();

	fcntl(stdout_fd, FcntlArg::F_SETFL(OFlag::O_NONBLOCK))?;
	fcntl(stderr_fd, FcntlArg::F_SETFL(OFlag::O_NONBLOCK))?;

	let output_buffer = BufReader::new(stdout);
	let error_buffer = BufReader::new(stderr);

	Ok(Buffer {
		output_buffer,
		error_buffer,
	})
}
