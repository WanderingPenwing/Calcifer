use egui_code_editor::ColorTheme;

pub struct CustomColorTheme;

impl CustomColorTheme {
    pub fn fire() -> ColorTheme {
        let mut theme = ColorTheme::GRUVBOX; // Or any other theme you want to modify

		theme.name = "Fire";
        theme.dark = true;
        theme.bg = "#242424";
        theme.cursor = "#dadada";      // foreground
        theme.selection = "#444852";   // dunno
        theme.comments = "#656565";    // dark_gray
        theme.functions = "#ffad69";   // light orange
        theme.keywords = "#48b1a7";    // mid green
        theme.literals = "#d2d2d3";    // 
        theme.numerics = "#ff7b4f";    // orange
        theme.punctuation = "#989898"; // gray
        theme.strs = "#cbd5a1";        // light_green
        theme.types = "#038e83";       // dark_green
        theme.special = "#48b1a7";     // mid green

        theme
    }
    
    pub fn ash() -> ColorTheme {
        let mut theme = ColorTheme::GRUVBOX; // Or any other theme you want to modify

		theme.name = "Ash";
        theme.dark = true;
        theme.bg = "#101010";
        theme.cursor = "#eaeaea";      // foreground
        theme.selection = "#505050";   // bg5
        theme.comments = "#656565";    // gray
        theme.functions = "#a0a0a0";   // green
        theme.keywords = "#848484";    // orange
        theme.literals = "#d2d2d2";    // foreground
        theme.numerics = "#d2d2d2";    // magenta
        theme.punctuation = "#848484"; // foreground
        theme.strs = "#a0a0a0";        // green
        theme.types = "#c6c6c6";       
        theme.special = "#848484";     

        theme
    }
    
    pub fn from_index(n : usize) -> ColorTheme {
		match n {
			0 => ColorTheme::SONOKAI,
			1 => ColorTheme::GRUVBOX,
			2 => ColorTheme::GITHUB_DARK,
			3 => ColorTheme::AYU_MIRAGE,
			4 => ColorTheme::AYU_DARK,
			5 => CustomColorTheme::ash(),
			6 => CustomColorTheme::fire(),
			_ => CustomColorTheme::ash(),
		}
	}
	
	pub fn max() -> usize {
		7
	}
	
	pub fn to_index(theme : ColorTheme) -> usize {
		match theme.name {
			"Sonokai" => 0,
			"Gruvbox" => 1,
			"Github Dark" => 2,
			"Ayu Mirage" => 3,
			"Ayu Dark" => 4,
			"Ash" => 5,
			"Fire" => 6,
			_ => 0,
		}
	}
}
