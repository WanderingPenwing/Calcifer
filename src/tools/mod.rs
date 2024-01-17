use eframe::egui;
use std::io;
use std::process::Command;
use std::cmp::Ordering;
use std::path::Path;
use std::fs;


pub fn loaded() {
	println!("Tools loaded");
}


pub fn run_command(cmd : String) -> String {
	let command = "> ".to_owned() + &cmd.clone() + "\n";
	let output = Command::new("sh")
        .arg("-c")
        .arg(cmd)
        .output()
        .expect("failed to execute process");
	(command + &String::from_utf8_lossy(&output.stdout)).to_string()
}


pub fn list_files(ui: &mut egui::Ui, path: &Path) -> io::Result<()> {
	if let Some(name) = path.file_name() {
		if path.is_dir() {
			egui::CollapsingHeader::new(name.to_string_lossy()).show(ui, |ui| {
                let mut paths: Vec<_> = fs::read_dir(&path).expect("Failed to read dir").map(|r| r.unwrap()).collect();
                                              
                // Sort the vector using the custom sorting function
				paths.sort_by(|a, b| sort_directories_first(a, b));

				for result in paths {
					//let result = path_result.expect("Failed to get path");
					//let full_path = result.path();
					let _ = list_files(ui, &result.path());
				}
            });
		} else {
			ui.label(name.to_string_lossy());
        }
    }
    Ok(())
}


fn sort_directories_first(a: &std::fs::DirEntry, b: &std::fs::DirEntry) -> Ordering {
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
