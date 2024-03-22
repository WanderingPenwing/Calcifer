use eframe::egui;
use image::GenericImageView;
use std::{
	error::Error,
	fs,
	fs::{read_to_string, OpenOptions},
	io::Write,
	path::{Path, PathBuf},
};
use serde::Serialize;
use serde::Deserialize;

#[derive(Serialize, Deserialize, Debug, PartialEq, Default)]
pub struct AppState {
	pub tabs: Vec<PathBuf>,
	pub theme: usize,
	pub zoom: f32,
}

pub fn save_state(state: &AppState, file_path: &Path) -> Result<(), std::io::Error> {
	let serialized_state = serde_json::to_string(state)?;

	if let Some(parent_dir) = file_path.parent() {
		fs::create_dir_all(parent_dir)?;
	}

	let mut file = OpenOptions::new()
		.write(true)
		.create(true)
		.truncate(true)
		.open(file_path)?;

	file.write_all(serialized_state.as_bytes())?;

	println!("Saved state at {}", file_path.display());

	Ok(())
}

pub fn load_state(file_path: &Path) -> Result<AppState, std::io::Error> {
	let serialized_state = read_to_string(file_path)?;

	Ok(serde_json::from_str(&serialized_state)?)
}

pub fn load_icon() -> Result<egui::IconData, Box<dyn Error>> {
	let (icon_rgba, icon_width, icon_height) = {
		let icon = include_bytes!("../../assets/icon.png");
		let image = image::load_from_memory(icon)?;
		let rgba = image.clone().into_rgba8().to_vec();
		let (width, height) = image.dimensions();
		(rgba, width, height)
	};

	Ok(egui::IconData {
		rgba: icon_rgba,
		width: icon_width,
		height: icon_height,
	})
}
