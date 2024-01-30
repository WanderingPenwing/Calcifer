use std::{
    cmp::Ordering,
    ffi::OsStr,
    fs, io,
    path::{Path, PathBuf},
};

use crate::ALLOWED_FILE_EXTENSIONS;

#[derive(Clone)]
pub struct FileEntry {
    pub name: String,
    pub path: PathBuf,
    pub folder_content: Option<Vec<FileEntry>>,
    pub folder_open: bool,
}

impl FileEntry {
    pub fn new_entry(name: String, path: PathBuf) -> Self {
        Self {
            name,
            path,
            folder_content: None,
            folder_open: false,
        }
    }
}

pub fn generate_file_tree(path: &Path, depth: isize) -> Option<FileEntry> {
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
        return Some(FileEntry::new_entry(name, path.to_path_buf()));
    }

    match fs::read_dir(path) {
        Err(err) => Some(FileEntry::new_entry(
            format!("Error reading directory: {}", err),
            path.to_path_buf(),
        )),
        Ok(entries) => {
            let mut paths: Vec<Result<fs::DirEntry, io::Error>> = entries
                .map(|r| r.map_err(|e| io::Error::new(io::ErrorKind::Other, e)))
                .collect();

            paths.sort_by(|a, b| match (a, b) {
                (Ok(entry_a), Ok(entry_b)) => sort_directories_first(entry_a, entry_b),
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
                        folder_content.push(FileEntry::new_entry(
                            format!("Error reading entry: {}", err),
                            path.to_path_buf(),
                        ));
                    }
                }
            }

            if folder_content.is_empty() {
                return None;
            }

            Some(FileEntry {
                name,
                path: path.to_path_buf(),
                folder_content: Some(folder_content),
                folder_open: false,
            })
        }
    }
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
