use crate::core::format_path;

use arboard::Clipboard;
use nix::fcntl::{fcntl, FcntlArg, OFlag};
use std::{
	env,
	io::{BufRead, BufReader},
	os::fd::AsRawFd,
	path::{Path, PathBuf},
	process::{Child, Command, Stdio},
};

pub struct Buffer {
	pub output_buffer: BufReader<std::process::ChildStdout>,
	pub error_buffer: BufReader<std::process::ChildStderr>,
	pub child: Child,
}

#[derive(Clone)]
pub struct Line {
	pub text: String,
	pub error: bool,
}

impl Line {
	fn output(text: String) -> Self {
		Self {
			text: remove_line_break(text),
			error: false,
		}
	}
	fn error(text: String) -> Self {
		Self {
			text: remove_line_break(text),
			error: true,
		}
	}
}

pub struct CommandEntry {
	pub env: String,
	pub command: String,
	pub result: Vec<Line>,
	pub buffer: Option<Buffer>,
	pub finished: bool,
}

impl CommandEntry {
	pub fn new(env: String, command: String) -> Self {
		let (buffer, result) = match execute(command.clone()) {
			Ok(command_buffer) => (Some(command_buffer), vec![]),
			Err(err) => (
				None,
				vec![Line::error(format!("failed to get results: {}", err))],
			),
		};

		CommandEntry {
			env,
			command,
			result,
			buffer,
			finished: false,
		}
	}

	pub fn update(&mut self) {
		if let Some(buffer) = &mut self.buffer {
			let mut output = String::new();
			loop {
				let _ = buffer.output_buffer.read_line(&mut output);
				if !remove_line_break(output.to_string()).is_empty() {
					self.result.push(Line::output(format!("{}\n", output)));
					output = "".to_string()
				} else {
					break;
				}
			}

			let mut error = String::new();
			loop {
				let _ = buffer.error_buffer.read_line(&mut error);
				if !remove_line_break(error.to_string()).is_empty() {
					self.result.push(Line::error(format!("{}\n", error)));
					error = "".to_string()
				} else {
					break;
				}
			}

			if let Ok(Some(_exit_status)) = buffer.child.try_wait() {
				//self.result.push(Line::output(format!("Command finished with status: {:?}\n", exit_status)));
				self.buffer_dump();
				self.finished = true;
			}
		}
	}
	
	fn buffer_dump(&mut self) {
//		if self.buffer.is_none() {
//			return
//		}
//		
//		let output_buffer = &self.buffer.as_ref().unwrap().output_buffer;
//		for line in output_buffer.lines() {
//            match line {
//                Ok(line) => self.result.push(Line::output(format!("{}\n", line))),
//                Err(_) => return,
//            }
//        }
//		let error_buffer = &self.buffer.as_ref().unwrap().error_buffer;
//		for line in error_buffer.lines() {
//            match line {
//                Ok(line) => self.result.push(Line::error(format!("{}\n", line))),
//                Err(_) => return,
//            }
//        }
	}

	pub fn copy_error_code(&self) {
		let mut txt: String = "".to_string();
		for line in self.result.iter() {
			if line.error {
				txt.push_str(&format!("{}\n", line.text));
			}
		}
		let mut _clipboard = Clipboard::new().expect("Failed to initialize clipboard");
		_clipboard.set_text(txt).unwrap();
	}
}

pub fn send_command(command: String) -> CommandEntry {
	let env = format_path(&env::current_dir().unwrap_or_else(|_| PathBuf::from("/")));

	if command.len() < 2 {
		return CommandEntry::new(env, command);
	}

	if &command[..2] != "cd" {
		return CommandEntry::new(env, command);
	}

	if command.len() < 4 {
		let mut entry =
			CommandEntry::new(env, "echo Invalid cd, should provide path >&2".to_string());
		entry.command = command;
		return entry;
	}

	let path_append = command[3..].replace('~', "/home/penwing");
	let path = Path::new(&path_append);

	if format!("{}", path.display()) == "/" {
		let mut entry = CommandEntry::new(env, "echo Root access denied >&2".to_string());
		entry.command = command;
		return entry;
	}

	if env::set_current_dir(path).is_ok() {
		let mut entry = CommandEntry::new(env, format!("echo Moved to : {}", path.display()));
		entry.command = command;
		entry
	} else {
		let mut entry = CommandEntry::new(
			env,
			format!("echo Could not find path : {} >&2", path.display()),
		);
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
		child,
	})
}

fn remove_line_break(input: String) -> String {
	let mut text = input.clone();
	while text.ends_with('\n') {
		text.pop();
		if text.ends_with('\r') {
			text.pop();
		}
	}
	text
}
