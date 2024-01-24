// Hello there, Master

impl super::Calcifer {
	pub fn handle_confirm(&mut self) {
		if self.close_tab_confirm.proceed {
			self.close_tab_confirm.close();
			self.delete_tab(self.tab_to_close);
		}
	}
	pub fn save_tab(&self) -> Option<PathBuf> {
		if self.tabs[self.selected_tab.to_index()].path.file_name().expect("Could not get Tab Name").to_string_lossy().to_string() == "untitled" {
			return self.save_tab_as();
		} else {
			if let Err(err) = fs::write(&self.tabs[self.selected_tab.to_index()].path, &self.tabs[self.selected_tab.to_index()].code) {
				eprintln!("Error writing file: {}", err);
				return None;
			}
			return Some(self.tabs[self.selected_tab.to_index()].path.clone())
		}
	}
	
	pub fn save_tab_as(&self) -> Option<PathBuf> {
		if let Some(path) = rfd::FileDialog::new().set_directory(Path::new(&PATH_ROOT)).save_file() {
			if let Err(err) = fs::write(&path, &self.tabs[self.selected_tab.to_index()].code) {
				eprintln!("Error writing file: {}", err);
				return None;
			}
			return Some(path);
		}
		return None
	}
	
	pub fn handle_save_file(&mut self, path_option : Option<PathBuf>) {
		if let Some(path) = path_option {
			println!("File saved successfully at: {:?}", path);
			self.tabs[self.selected_tab.to_index()].path = path;
			self.tabs[self.selected_tab.to_index()].saved = true;
		} else {
			println!("File save failed.");
		}
	}
	
	pub fn from_app_state(app_state: tools::AppState) -> Self {
		let mut new = Self {
			theme: DEFAULT_THEMES[min(app_state.theme, DEFAULT_THEMES.len() - 1)],
			tabs: Vec::new(),
			..Default::default()
		};
		
		for path in app_state.tabs {
			if path.file_name().expect("Could not get Tab Name").to_string_lossy().to_string() != "untitled" {
				new.open_file(Some(&path));
			}
		}
		
		if new.tabs == vec![] {
			new.open_file(None);
		}
		
		new
	}
	
	pub fn save_state(&self) {
		let mut state_theme : usize = 0;
		if let Some(theme) = DEFAULT_THEMES.iter().position(|&r| r == self.theme) {
			state_theme = theme;
		}
		
		let mut state_tabs = vec![];
		
		for tab in &self.tabs {
			state_tabs.push(tab.path.clone());
		}
		let app_state = tools::AppState {
			tabs: state_tabs,
			theme: state_theme,
		};
		
		let _ = tools::save_state(&app_state, super::SAVE_PATH);
	}
	
	pub fn move_through_tabs(&mut self, forward : bool) {
		let new_index = if forward {
			(self.selected_tab.to_index() + 1) % self.tabs.len()
		} else {
			self.selected_tab.to_index().checked_sub(1).unwrap_or(self.tabs.len() - 1)
		};
		self.selected_tab = tools::TabNumber::from_index(new_index);
	}
	
	fn list_files(&mut self, ui: &mut egui::Ui, path: &Path) -> io::Result<()> {
		if let Some(name) = path.file_name() {
			if path.is_dir() {
				egui::CollapsingHeader::new(name.to_string_lossy()).show(ui, |ui| {
					let mut paths: Vec<_> = fs::read_dir(&path).expect("Failed to read dir").map(|r| r.unwrap()).collect();
												  
					// Sort the vector using the custom sorting function
					paths.sort_by(|a, b| tools::sort_directories_first(a, b));

					for result in paths {
						//let result = path_result.expect("Failed to get path");
						//let full_path = result.path();
						let _ = self.list_files(ui, &result.path());
					}
				});
			} else {
				//ui.label(name.to_string_lossy());
				if ui.button(name.to_string_lossy()).clicked() {
					self.open_file(Some(path));
				}
			}
		}
		Ok(())
	}
	
	fn open_file(&mut self, path_option: Option<&Path>) {
		if self.tabs.len() < MAX_TABS {
			if let Some(path) = path_option {
				self.tabs.push(tools::Tab::new(path.to_path_buf()));
			} else {
				self.tabs.push(tools::Tab::default());
			}
			self.selected_tab = tools::TabNumber::from_index(self.tabs.len() - 1);
		}
	}
	
	fn delete_tab(&mut self, index : usize) {
		self.tabs.remove(index);
		self.selected_tab = tools::TabNumber::from_index(min(index, self.tabs.len() - 1));
	}
	
	fn toggle(&self, ui: &mut egui::Ui, display : bool, title : &str) -> bool {
		let text = if display.clone() {
			format!("hide {}", title)
		} else {
			format!("show {}", title)
		};
					
		if ui.add(egui::Button::new(text)).clicked() {
			return !display
		}
		return display
	}
}