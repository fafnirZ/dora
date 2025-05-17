// thanks gemini.

use ratatui::style::Color;

struct Colour {
    name: &'static str,
    hex: &'static str,
    rgb: (u8, u8, u8),
}


impl Colour {
    pub fn to_ratatui_color_rgb(&self) -> Color {
        let (r,g,b) = self.rgb;
        return Color::Rgb(r,g,b);
    }
}


const DARK_BLUE: Colour = Colour {
    name: "Dark Blue",
    hex: "#1e40af",
    rgb: (30, 64, 175),
};

const WHITE: Colour = Colour {
    name: "White",
    hex: "#f5f5f5",
    rgb: (245, 245, 245),
};

const YELLOW: Colour = Colour {
    name: "Yellow",
    hex: "#facc15",
    rgb: (250, 204, 21),
};

const LIGHT_BLUE: Colour = Colour {
    name: "Light Blue",
    hex: "#60a5fa",
    rgb: (96, 165, 250),
};

const LIGHT_YELLOW: Colour = Colour {
    name: "Light Yellow",
    hex: "#fef08a",
    rgb: (254, 240, 138),
};

const DARK_TEAL: Colour = Colour {
    name: "Dark Teal",
    hex: "#0f766e",
    rgb: (15, 118, 110),
};

const ORANGE: Colour = Colour {
    name: "Orange",
    hex: "#ea580c",
    rgb: (234, 88, 12),
};

const LIGHT_TEAL: Colour = Colour {
    name: "Light Teal",
    hex: "#6ee7b7",
    rgb: (110, 231, 183),
};

const LIGHT_ORANGE: Colour = Colour {
    name: "Light Orange",
    hex: "#fdb974",
    rgb: (253, 185, 116),
};

const DARK_PURPLE: Colour = Colour {
    name: "Dark Purple",
    hex: "#5b21b6",
    rgb: (91, 33, 182),
};

const GREEN: Colour = Colour {
    name: "Green",
    hex: "#16a34a",
    rgb: (22, 163, 74),
};

const LIGHT_PURPLE: Colour = Colour {
    name: "Light Purple",
    hex: "#d946ef",
    rgb: (217, 70, 239),
};

const LIGHT_GREEN: Colour = Colour {
    name: "Light Green",
    hex: "#86ef7d",
    rgb: (134, 239, 125),
};

const DARK_BLUE_GRAY: Colour = Colour {
    name: "Dark Blue Gray",
    hex: "#282850", // Approximate hex
    rgb: (40, 40, 80),
};