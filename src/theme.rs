use ratatui::style::Color;

#[derive(Debug, Clone)]
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
    // Keyboard colors
    pub keyboard_key: Color,
    pub keyboard_key_text: Color,
    pub current_key_highlight: Color,
    pub finger_pinky: Color,
    pub finger_ring: Color,
    pub finger_middle: Color,
    pub finger_index: Color,
    pub finger_thumb: Color,
}

impl Theme {
    pub fn from_name(name: &str) -> Self {
        match name.to_lowercase().as_str() {
            "dark" => Self::dark(),
            "light" => Self::light(),
            "nord" => Self::nord(),
            "dracula" => Self::dracula(),
            "solarized" => Self::solarized(),
            "catppuccin-mocha" | "catppuccin" | "mocha" => Self::catppuccin_mocha(),
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
            keyboard_key: Color::Rgb(45, 45, 45),
            keyboard_key_text: Color::White,
            current_key_highlight: Color::Yellow,
            finger_pinky: Color::Rgb(255, 100, 100),
            finger_ring: Color::Rgb(255, 180, 100),
            finger_middle: Color::Rgb(100, 255, 100),
            finger_index: Color::Rgb(100, 180, 255),
            finger_thumb: Color::Rgb(200, 100, 255),
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
            keyboard_key: Color::Rgb(220, 220, 220),
            keyboard_key_text: Color::Black,
            current_key_highlight: Color::Rgb(255, 100, 0),
            finger_pinky: Color::Rgb(200, 80, 80),
            finger_ring: Color::Rgb(200, 140, 60),
            finger_middle: Color::Rgb(60, 160, 60),
            finger_index: Color::Rgb(60, 120, 200),
            finger_thumb: Color::Rgb(140, 60, 180),
        }
    }

    pub fn nord() -> Self {
        Self {
            name: "nord".to_string(),
            correct_char: Color::Rgb(163, 190, 140), // Nord14 - green
            incorrect_char: Color::Rgb(191, 97, 106), // Nord11 - red
            untyped_char: Color::Rgb(76, 86, 106),   // Nord3 - dark gray
            cursor_fg: Color::Rgb(236, 239, 244),    // Nord6 - white
            cursor_bg: Color::Rgb(76, 86, 106),      // Nord3
            wpm_color: Color::Rgb(136, 192, 208),    // Nord8 - cyan
            accuracy_color: Color::Rgb(235, 203, 139), // Nord13 - yellow
            error_color: Color::Rgb(191, 97, 106),   // Nord11 - red
            mode_color: Color::Rgb(180, 142, 173),   // Nord15 - purple
            border_color: Color::Rgb(136, 192, 208), // Nord8 - cyan
            title_color: Color::Rgb(136, 192, 208),  // Nord8
            success_color: Color::Rgb(163, 190, 140), // Nord14
            keyboard_key: Color::Rgb(67, 76, 94),    // Nord2
            keyboard_key_text: Color::Rgb(216, 222, 233), // Nord6
            current_key_highlight: Color::Rgb(136, 192, 208), // Nord8
            finger_pinky: Color::Rgb(191, 97, 106),  // Nord11
            finger_ring: Color::Rgb(235, 203, 139),  // Nord13
            finger_middle: Color::Rgb(163, 190, 140), // Nord14
            finger_index: Color::Rgb(136, 192, 208), // Nord8
            finger_thumb: Color::Rgb(180, 142, 173), // Nord15
        }
    }

    pub fn dracula() -> Self {
        Self {
            name: "dracula".to_string(),
            correct_char: Color::Rgb(80, 250, 123),    // Green
            incorrect_char: Color::Rgb(255, 85, 85),   // Red
            untyped_char: Color::Rgb(98, 114, 164),    // Comment gray
            cursor_fg: Color::Rgb(248, 248, 242),      // Foreground
            cursor_bg: Color::Rgb(68, 71, 90),         // Current line
            wpm_color: Color::Rgb(139, 233, 253),      // Cyan
            accuracy_color: Color::Rgb(241, 250, 140), // Yellow
            error_color: Color::Rgb(255, 85, 85),      // Red
            mode_color: Color::Rgb(255, 121, 198),     // Pink
            border_color: Color::Rgb(189, 147, 249),   // Purple
            title_color: Color::Rgb(189, 147, 249),    // Purple
            success_color: Color::Rgb(80, 250, 123),   // Green
            keyboard_key: Color::Rgb(68, 71, 90),      // Current line
            keyboard_key_text: Color::Rgb(248, 248, 242), // Foreground
            current_key_highlight: Color::Rgb(255, 121, 198), // Pink
            finger_pinky: Color::Rgb(255, 85, 85),     // Red
            finger_ring: Color::Rgb(241, 250, 140),    // Yellow
            finger_middle: Color::Rgb(80, 250, 123),   // Green
            finger_index: Color::Rgb(139, 233, 253),   // Cyan
            finger_thumb: Color::Rgb(255, 121, 198),   // Pink
        }
    }

    pub fn solarized() -> Self {
        Self {
            name: "solarized".to_string(),
            correct_char: Color::Rgb(133, 153, 0),   // Green
            incorrect_char: Color::Rgb(220, 50, 47), // Red
            untyped_char: Color::Rgb(88, 110, 117),  // Base01
            cursor_fg: Color::Rgb(253, 246, 227),    // Base3
            cursor_bg: Color::Rgb(88, 110, 117),     // Base01
            wpm_color: Color::Rgb(42, 161, 152),     // Cyan
            accuracy_color: Color::Rgb(181, 137, 0), // Yellow
            error_color: Color::Rgb(220, 50, 47),    // Red
            mode_color: Color::Rgb(211, 54, 130),    // Magenta
            border_color: Color::Rgb(38, 139, 210),  // Blue
            title_color: Color::Rgb(38, 139, 210),   // Blue
            success_color: Color::Rgb(133, 153, 0),  // Green
            keyboard_key: Color::Rgb(88, 110, 117),  // Base01
            keyboard_key_text: Color::Rgb(253, 246, 227), // Base3
            current_key_highlight: Color::Rgb(181, 137, 0), // Yellow
            finger_pinky: Color::Rgb(220, 50, 47),   // Red
            finger_ring: Color::Rgb(181, 137, 0),    // Yellow
            finger_middle: Color::Rgb(133, 153, 0),  // Green
            finger_index: Color::Rgb(38, 139, 210),  // Blue
            finger_thumb: Color::Rgb(211, 54, 130),  // Magenta
        }
    }

    pub fn catppuccin_mocha() -> Self {
        Self {
            name: "catppuccin-mocha".to_string(),
            correct_char: Color::Rgb(166, 227, 161), // green  #a6e3a1
            incorrect_char: Color::Rgb(243, 139, 168), // red    #f38ba8
            untyped_char: Color::Rgb(88, 91, 112),   // surface2 #585b70
            cursor_fg: Color::Rgb(205, 214, 244),    // text   #cdd6f4
            cursor_bg: Color::Rgb(49, 50, 68),       // surface0 #313244
            wpm_color: Color::Rgb(148, 226, 213),    // teal   #94e2d5
            accuracy_color: Color::Rgb(249, 226, 175), // yellow #f9e2af
            error_color: Color::Rgb(243, 139, 168),  // red    #f38ba8
            mode_color: Color::Rgb(203, 166, 247),   // mauve  #cba6f7
            border_color: Color::Rgb(116, 199, 236), // sapphire-ish #74c7ec[web:180]
            title_color: Color::Rgb(180, 190, 254),  // lavender #b4befe
            success_color: Color::Rgb(166, 227, 161), // green  #a6e3a1
            keyboard_key: Color::Rgb(49, 50, 68),    // surface0
            keyboard_key_text: Color::Rgb(205, 214, 244), // text
            current_key_highlight: Color::Rgb(249, 226, 175), // yellow
            finger_pinky: Color::Rgb(243, 139, 168), // red
            finger_ring: Color::Rgb(249, 226, 175),  // yellow
            finger_middle: Color::Rgb(166, 227, 161), // green
            finger_index: Color::Rgb(137, 180, 250), // blue
            finger_thumb: Color::Rgb(203, 166, 247), // mauve
        }
    }

    pub fn available_themes() -> Vec<&'static str> {
        vec![
            "dark",
            "light",
            "nord",
            "dracula",
            "solarized",
            "catppuccin-mocha",
        ]
    }
}
