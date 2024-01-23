# Calcifer

My custom code editor (only the features I want inside) using egui and my own fork of egui_code_editor https://lib.rs/crates/egui_code_editor

# 1.0.0 :
Added a File Tree
Added Tabs
Added an Embedded Terminal
Added Syntax Highlighting
Added Themes

# 1.0.1 :
Fixed Terminal sterr output
Fixed scroll between tabs
Library subjugation (got the raw files of the egui_code_editor for some internal modifications)

# 1.0.2 :
Added find and replace function
Added multi line tab and shift+tab
Added Ctrl+E : comment multiline
Fixed Ctr+Z (was already in library, tried to make my own, and then found the better one)
Added indent recognition (when there is a line break, the indentation level is kept)


# 1.0.3 :
Added Ctrl+T : turn 4 spaces into tab across the whole document