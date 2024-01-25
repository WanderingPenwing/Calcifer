use super::ColorTheme;

impl ColorTheme {
    ///  Original Author: sainnhe <https://github.com/sainnhe/sonokai>
    ///  Modified by p4ymak <https://github.com/p4ymak>
    pub const FIRE: ColorTheme = ColorTheme {
        name: "Fire",
        dark: true,
        bg: "#242424",
        cursor: "#dadada",      // foreground
        selection: "#444852",   // dunno
        comments: "#656565",    // dark_gray
        functions: "#ffad69",   // light orange
        keywords: "#48b1a7",    // mid green
        literals: "#d2d2d3",    //
        numerics: "#ff7b4f",    // orange
        punctuation: "#989898", // gray
        strs: "#cbd5a1",        // light_green
        types: "#038e83",       // dark_green
        special: "#48b1a7",     // mid green
    };

    pub const ASH: ColorTheme = ColorTheme {
        name: "Ash",
        dark: true,
        bg: "#101010",
        cursor: "#eaeaea",
        selection: "#505050",
        comments: "#656565",
        functions: "#a0a0a0",
        keywords: "#848484",
        literals: "#d2d2d2",
        numerics: "#d2d2d2",
        punctuation: "#848484",
        strs: "#a0a0a0",
        types: "#c6c6c6",
        special: "#848484",
    };
}
