#[cfg(test)]

mod tests {

    use crate::tools::*;

    #[test]
    fn test_tab_number_conversions() {
        let tab_num = TabNumber::from_index(3);
        assert_eq!(tab_num, TabNumber::Three);
        assert_eq!(tab_num.to_index(), 3);
    }

    #[test]
    fn test_default_tab() {
        let default_tab = Tab::default();
        assert_eq!(default_tab.path, PathBuf::from("untitled"));
        assert_eq!(default_tab.code, "// Hello there, Master");
        assert_eq!(default_tab.language, "rs");
        assert!(!default_tab.saved);
        assert_eq!(default_tab.scroll_offset, 0.0);
        assert_eq!(default_tab.last_cursor, None);
    }

    #[test]
    fn test_get_tab_name() {
        let tab = Tab {
            path: PathBuf::from("/path/to/file.rs"),
            code: String::from(""),
            language: String::from("rs"),
            saved: true,
            scroll_offset: 0.0,
            last_cursor: None,
        };
        assert_eq!(tab.get_name(), "file.rs");
    }

    #[test]
    fn test_default_command_entry() {
        let default_entry = CommandEntry::default();
        assert_eq!(
            default_entry.env,
            env::current_dir()
                .expect("Could not find Shell Environnment")
                .file_name()
                .expect("Could not get Shell Environnment Name")
                .to_string_lossy()
                .to_string()
        );
        assert_eq!(default_entry.command, "");
        assert_eq!(default_entry.output, "");
        assert_eq!(default_entry.error, "");
    }

    #[test]
    fn test_save_and_load_state() {
        let tabs = vec![
            PathBuf::from("/path/to/file1.rs"),
            PathBuf::from("/path/to/file2.py"),
        ];
        let theme = 42;
        let original_state = AppState { tabs, theme };

        // Save state to a temporary file
        let temp_file_path = "/tmp/test_state.json";
        save_state(&original_state, temp_file_path).expect("Failed to save state");

        // Load state from the temporary file
        let loaded_state = load_state(temp_file_path).expect("Failed to load state");

        assert_eq!(original_state, loaded_state);
    }

    #[test]
    fn test_run_command() {
        let cmd = "echo hello".to_string();
        let entry = run_command(cmd);
        assert_eq!(entry.command, "echo hello");
        assert_eq!(entry.output.trim(), "hello");
        assert_eq!(entry.error, "");
    }

    // Add more tests as needed for other functions
}
