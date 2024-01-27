use std::ffi::OsStr;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};
//use eframe::egui;

use crate::tools;
use crate::ALLOWED_FILE_EXTENSIONS;

#[derive(Clone)]
pub struct File {
	pub name: String,
	pub path: PathBuf,
	pub folder_content: Option<Vec<File>>,
	pub folder_open: bool,
}


impl File {
	pub fn new_file(name: String, path: PathBuf) -> Self {
		Self {
			name,
			path,
			folder_content: None,
			folder_open: false,
		}
	}
	
	pub fn empty() -> Self {
		Self {
			name: "No file found".into(),
			path: Path::new("/").to_path_buf(),
			folder_content: None,
			folder_open: false,
		}
	}
}


pub fn generate_file_tree(path: &Path, depth: isize) -> Option<File> {
	if let Some(file_name) = path.file_name() {
		if file_name.to_string_lossy().starts_with('.') {
			return None;
		}
		let extension = path.extension().and_then(|ext| ext.to_str());
		if !ALLOWED_FILE_EXTENSIONS.contains(&extension.unwrap_or_default()) {
			return None;
		}
	} else {
		return None;
	}

	let name = path
		.file_name()
		.unwrap_or_else(|| OsStr::new(""))
		.to_string_lossy()
		.into_owned();

	if !path.is_dir() || depth < 0 {
		return Some(File::new_file(name, path.to_path_buf()));
	}

	match fs::read_dir(path) {
		Err(err) => {
			return Some(File::new_file(format!("Error reading directory: {}", err), path.to_path_buf()));
		}
		Ok(entries) => {
			let mut paths: Vec<Result<fs::DirEntry, io::Error>> = entries
				.map(|r| r.map_err(|e| io::Error::new(io::ErrorKind::Other, e)))
				.collect();

			paths.sort_by(|a, b| match (a, b) {
				(Ok(entry_a), Ok(entry_b)) => tools::sort_directories_first(&entry_a, &entry_b),
				(Err(_), Ok(_)) => std::cmp::Ordering::Greater,
				(Ok(_), Err(_)) => std::cmp::Ordering::Less,
				(Err(_), Err(_)) => std::cmp::Ordering::Equal,
			});

			let mut folder_content = Vec::new();

			for result in paths {
				match result {
					Ok(entry) => {
						if let Some(file) = generate_file_tree(&entry.path(), depth - 1) {
							folder_content.push(file);
						}
					}
					Err(err) => {
						folder_content.push(File::new_file(
							format!("Error reading entry: {}", err),
							path.to_path_buf(),
						));
					}
				}
			}
			
			if folder_content.is_empty() {
				return None;
			}

			return Some(File {
				name,
				path: path.to_path_buf(),
				folder_content: Some(folder_content),
				folder_open: false,
			});
		}
	}
}