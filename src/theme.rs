use ratatui::style::Color;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Theme {
    pub name: String,
    pub correct_char: Color,
    pub incorrect_char: Color,
    pub untyped_char: Color,
    pub cursor_fg: Color,
    pub cursor_bg: Color,
    pub wpm_color: Color,
    pub accuracy_color: Color,
    pub error_color: Color,
    pub mode_color: Color,
    pub border_color: Color,
    pub title_color: Color,
    pub success_color: Color,
}

impl Theme {
    pub fn from_name(name: &str) -> Self {
        match name.to_lowercase().as_str() {
            "dark" => Self::dark(),
            "light" => Self::light(),
            "nord" => Self::nord(),
            "dracula" => Self::dracula(),
            "solarized" => Self::solarized(),
            _ => Self::dark(), // Default fallback
        }
    }

    pub fn dark() -> Self {
        Self {
            name: "dark".to_string(),
            correct_char: Color::Green,
            incorrect_char: Color::Red,
            untyped_char: Color::DarkGray,
            cursor_fg: Color::White,
            cursor_bg: Color::DarkGray,
            wpm_color: Color::Cyan,
            accuracy_color: Color::Yellow,
            error_color: Color::Red,
            mode_color: Color::Magenta,
            border_color: Color::Cyan,
            title_color: Color::Cyan,
            success_color: Color::Green,
        }
    }

    pub fn light() -> Self {
        Self {
            name: "light".to_string(),
            correct_char: Color::Green,
            incorrect_char: Color::Red,
            untyped_char: Color::Gray,
            cursor_fg: Color::Black,
            cursor_bg: Color::Gray,
            wpm_color: Color::Blue,
            accuracy_color: Color::Magenta,
            error_color: Color::Red,
            mode_color: Color::Magenta,
            border_color: Color::Blue,
            title_color: Color::Blue,
            success_color: Color::Green,
        }
    }

    pub fn nord() -> Self {
        Self {
            name: "nord".to_string(),
            correct_char: Color::Rgb(163, 190, 140),  // Nord14 - green
            incorrect_char: Color::Rgb(191, 97, 106), // Nord11 - red
            untyped_char: Color::Rgb(76, 86, 106),    // Nord3 - dark gray
            cursor_fg: Color::Rgb(236, 239, 244),     // Nord6 - white
            cursor_bg: Color::Rgb(76, 86, 106),       // Nord3
            wpm_color: Color::Rgb(136, 192, 208),     // Nord8 - cyan
            accuracy_color: Color::Rgb(235, 203, 139), // Nord13 - yellow
            error_color: Color::Rgb(191, 97, 106),    // Nord11 - red
            mode_color: Color::Rgb(180, 142, 173),    // Nord15 - purple
            border_color: Color::Rgb(136, 192, 208),  // Nord8 - cyan
            title_color: Color::Rgb(136, 192, 208),   // Nord8
            success_color: Color::Rgb(163, 190, 140), // Nord14
        }
    }

    pub fn dracula() -> Self {
        Self {
            name: "dracula".to_string(),
            correct_char: Color::Rgb(80, 250, 123),   // Green
            incorrect_char: Color::Rgb(255, 85, 85),  // Red
            untyped_char: Color::Rgb(98, 114, 164),   // Comment gray
            cursor_fg: Color::Rgb(248, 248, 242),     // Foreground
            cursor_bg: Color::Rgb(68, 71, 90),        // Current line
            wpm_color: Color::Rgb(139, 233, 253),     // Cyan
            accuracy_color: Color::Rgb(241, 250, 140), // Yellow
            error_color: Color::Rgb(255, 85, 85),     // Red
            mode_color: Color::Rgb(255, 121, 198),    // Pink
            border_color: Color::Rgb(189, 147, 249),  // Purple
            title_color: Color::Rgb(189, 147, 249),   // Purple
            success_color: Color::Rgb(80, 250, 123),  // Green
        }
    }

    pub fn solarized() -> Self {
        Self {
            name: "solarized".to_string(),
            correct_char: Color::Rgb(133, 153, 0),    // Green
            incorrect_char: Color::Rgb(220, 50, 47),  // Red
            untyped_char: Color::Rgb(88, 110, 117),   // Base01
            cursor_fg: Color::Rgb(253, 246, 227),     // Base3
            cursor_bg: Color::Rgb(88, 110, 117),      // Base01
            wpm_color: Color::Rgb(42, 161, 152),      // Cyan
            accuracy_color: Color::Rgb(181, 137, 0),  // Yellow
            error_color: Color::Rgb(220, 50, 47),     // Red
            mode_color: Color::Rgb(211, 54, 130),     // Magenta
            border_color: Color::Rgb(38, 139, 210),   // Blue
            title_color: Color::Rgb(38, 139, 210),    // Blue
            success_color: Color::Rgb(133, 153, 0),   // Green
        }
    }

    pub fn available_themes() -> Vec<&'static str> {
        vec!["dark", "light", "nord", "dracula", "solarized"]
    }
}
