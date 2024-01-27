// Placeholder

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
}

impl CommandEntry {
	pub fn new(env: String, command: String) -> Self {
		CommandEntry {
			env,
			command,
			result: vec![Line::error("General Kenobi")],
		}
	}

	pub fn update(&mut self) {
		return
	}
}

pub fn send_command(command: String) -> CommandEntry {
	return CommandEntry::new("windows>", "hello there");
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
