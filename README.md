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
Added testing
Added Ctrl+T : refresh current tab
Added Time debug
Added Tree toggle for performance
Added Alt+Arrows to move through tabs
Added Zoom
Added cd
Added terminal color
Max tabs 8 => 20
Max framerate => 30 fps (less cpu usage)

# 1.1.0 :
Added confirm prompt if unsaved 
Async terminal !
Better Ui