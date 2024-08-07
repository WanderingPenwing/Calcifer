use std::hash::DefaultHasher;
use std::hash::Hash;
use std::hash::Hasher;
use std::time::SystemTime;
use std::time::UNIX_EPOCH;
use std::{
    cmp::Ordering,
    ffi::OsStr,
    fs, io,
    path::{Path, PathBuf},
};

//use crate::ALLOWED_FILE_EXTENSIONS;

#[derive(Clone)]
pub struct FileEntry {
    pub name: String,
    pub path: PathBuf,
    pub folder_content: Option<Vec<FileEntry>>,
    pub content_checked: bool,
    pub id: String,
}

impl FileEntry {
    pub fn new_entry(name: String, path: PathBuf) -> Self {
        Self {
            name,
            path: path.clone(),
            folder_content: None,
            content_checked: true,
            id: Self::generate_unique_id(path),
        }
    }
    pub fn end_of_branch(name: String, path: PathBuf) -> Self {
        Self {
            name,
            path: path.clone(),
            folder_content: Some(vec![]),
            content_checked: false,
            id: Self::generate_unique_id(path),
        }
    }

    fn generate_unique_id(path: PathBuf) -> String {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        let mut hasher = DefaultHasher::new();
        now.hash(&mut hasher);

        let hash = hasher.finish();

        format!("{}#{}", path.display(), hash)
    }
}

pub fn update_file_tree(file: FileEntry, opened_dirs: Vec<String>) -> FileEntry {
    if !opened_dirs.contains(&get_file_path_id(&file.path)) {
        return file;
    }

    if !file.content_checked {
        let folder = generate_folder_entry(&file.path);
        return update_file_tree(folder, opened_dirs);
    }

    if file.folder_content.is_none() {
        return file;
    }

    let folder_content = file.folder_content.expect("should have checked if none");

    let updated_content: Vec<FileEntry> = folder_content
        .iter()
        .map(|entry| update_file_tree(entry.clone(), opened_dirs.clone()))
        .collect();
    FileEntry {
        name: file.name,
        path: file.path.clone(),
        folder_content: Some(updated_content),
        content_checked: true,
        id: FileEntry::generate_unique_id(file.path),
    }
}

pub fn get_file_path_id(path: &PathBuf) -> String {
    format!("#{}", path.clone().display())
}

pub fn generate_folder_entry(path: &Path) -> FileEntry {
    if let Some(file_name) = path.file_name() {
        let name = file_name.to_string_lossy().into_owned();

        match fs::read_dir(path) {
            Err(err) => FileEntry::new_entry(
                format!("Error reading directory: {}", err),
                path.to_path_buf(),
            ),
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
                            if let Some(file) = generate_entry(&entry.path()) {
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

                FileEntry {
                    name,
                    path: path.to_path_buf(),
                    folder_content: Some(folder_content),
                    content_checked: true,
                    id: FileEntry::generate_unique_id(path.to_path_buf()),
                }
            }
        }
    } else {
        FileEntry::new_entry(
            "Error reading directory name".to_string(),
            path.to_path_buf(),
        )
    }
}

fn generate_entry(path: &Path) -> Option<FileEntry> {
    if let Some(file_name) = path.file_name() {
        if file_name.to_string_lossy().starts_with('.') {
            return None;
        }
    //		let extension = path.extension().and_then(|ext| ext.to_str());
    //		if !ALLOWED_FILE_EXTENSIONS.contains(&extension.unwrap_or_default()) {
    //			return None;
    //		}
    } else {
        return None;
    }

    let name = path
        .file_name()
        .unwrap_or_else(|| OsStr::new(""))
        .to_string_lossy()
        .into_owned();

    if !path.is_dir() {
        return Some(FileEntry::new_entry(name, path.to_path_buf()));
    }
    Some(FileEntry::end_of_branch(name, path.to_path_buf()))
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
