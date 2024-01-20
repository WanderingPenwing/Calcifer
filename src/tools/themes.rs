use egui_code_editor::ColorTheme;

pub struct CustomColorTheme;

impl CustomColorTheme {
    pub fn fire() -> ColorTheme {
        let mut theme = ColorTheme::GRUVBOX; // Or any other theme you want to modify

		theme.name = "Fire";
        theme.dark = true;
        theme.bg = "#101010";
        theme.cursor = "#fafafa";      // foreground
        theme.selection = "#fa8d3e";   // orange
        theme.comments = "#828c9a";    // gray
        theme.functions = "#ffaa33";   // yellow
        theme.keywords = "#fa8d3e";    // orange
        theme.literals = "#5c6166";    // foreground
        theme.numerics = "#aa1010";    // magenta
        theme.punctuation = "#fafafa"; // foreground
        theme.strs = "#fa8d3e";        // green
        theme.types = "#fa8d3e";       // blue
        theme.special = "#f07171";     // red

        theme
    }
}
