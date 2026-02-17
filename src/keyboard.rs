#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Finger {
    Pinky,
    Ring,
    Middle,
    IndexLeft,
    IndexRight,
    Thumb,
}

#[derive(Clone)]
pub struct KeyDef {
    pub label: &'static str,
    pub width: u8,
    pub finger: Finger,
    pub visual_width: Option<u8>,
}

pub struct KeyboardLayout {
    rows: Vec<Vec<KeyDef>>,
    home_row: Vec<char>,
}

impl KeyboardLayout {
    pub fn new() -> Self {
        let rows: Vec<Vec<KeyDef>> = vec![
            // Row 0: Number row
            vec![
                KeyDef {
                    label: "`",
                    width: 3,
                    finger: Finger::Pinky,
                    visual_width: None,
                },
                KeyDef {
                    label: "1",
                    width: 3,
                    finger: Finger::Pinky,
                    visual_width: None,
                },
                KeyDef {
                    label: "2",
                    width: 3,
                    finger: Finger::Pinky,
                    visual_width: None,
                },
                KeyDef {
                    label: "3",
                    width: 3,
                    finger: Finger::Ring,
                    visual_width: None,
                },
                KeyDef {
                    label: "4",
                    width: 3,
                    finger: Finger::Ring,
                    visual_width: None,
                },
                KeyDef {
                    label: "5",
                    width: 3,
                    finger: Finger::Ring,
                    visual_width: None,
                },
                KeyDef {
                    label: "6",
                    width: 3,
                    finger: Finger::Ring,
                    visual_width: None,
                },
                KeyDef {
                    label: "7",
                    width: 3,
                    finger: Finger::Ring,
                    visual_width: None,
                },
                KeyDef {
                    label: "8",
                    width: 3,
                    finger: Finger::Ring,
                    visual_width: None,
                },
                KeyDef {
                    label: "9",
                    width: 3,
                    finger: Finger::Ring,
                    visual_width: None,
                },
                KeyDef {
                    label: "0",
                    width: 3,
                    finger: Finger::Ring,
                    visual_width: None,
                },
                KeyDef {
                    label: "-",
                    width: 3,
                    finger: Finger::Pinky,
                    visual_width: None,
                },
                KeyDef {
                    label: "=",
                    width: 3,
                    finger: Finger::Pinky,
                    visual_width: None,
                },
                KeyDef {
                    label: "←",
                    width: 4,
                    finger: Finger::Pinky,
                    visual_width: Some(1),
                },
            ],
            // Row 1: QWERTY row
            vec![
                KeyDef {
                    label: "⇥",
                    width: 4,
                    finger: Finger::Pinky,
                    visual_width: Some(1),
                },
                KeyDef {
                    label: "q",
                    width: 3,
                    finger: Finger::Pinky,
                    visual_width: None,
                },
                KeyDef {
                    label: "w",
                    width: 3,
                    finger: Finger::Ring,
                    visual_width: None,
                },
                KeyDef {
                    label: "e",
                    width: 3,
                    finger: Finger::Middle,
                    visual_width: None,
                },
                KeyDef {
                    label: "r",
                    width: 3,
                    finger: Finger::IndexLeft,
                    visual_width: None,
                },
                KeyDef {
                    label: "t",
                    width: 3,
                    finger: Finger::IndexLeft,
                    visual_width: None,
                },
                KeyDef {
                    label: "y",
                    width: 3,
                    finger: Finger::IndexRight,
                    visual_width: None,
                },
                KeyDef {
                    label: "u",
                    width: 3,
                    finger: Finger::IndexRight,
                    visual_width: None,
                },
                KeyDef {
                    label: "i",
                    width: 3,
                    finger: Finger::Middle,
                    visual_width: None,
                },
                KeyDef {
                    label: "o",
                    width: 3,
                    finger: Finger::Ring,
                    visual_width: None,
                },
                KeyDef {
                    label: "p",
                    width: 3,
                    finger: Finger::Pinky,
                    visual_width: None,
                },
                KeyDef {
                    label: "[",
                    width: 3,
                    finger: Finger::Pinky,
                    visual_width: None,
                },
                KeyDef {
                    label: "]",
                    width: 3,
                    finger: Finger::Pinky,
                    visual_width: None,
                },
                KeyDef {
                    label: "\\",
                    width: 3,
                    finger: Finger::Pinky,
                    visual_width: None,
                },
            ],
            // Row 2: Home row (ASDF)
            vec![
                KeyDef {
                    label: "⇪",
                    width: 6,
                    finger: Finger::Pinky,
                    visual_width: Some(1),
                },
                KeyDef {
                    label: "a",
                    width: 3,
                    finger: Finger::Pinky,
                    visual_width: None,
                },
                KeyDef {
                    label: "s",
                    width: 3,
                    finger: Finger::Ring,
                    visual_width: None,
                },
                KeyDef {
                    label: "d",
                    width: 3,
                    finger: Finger::Middle,
                    visual_width: None,
                },
                KeyDef {
                    label: "f",
                    width: 3,
                    finger: Finger::IndexLeft,
                    visual_width: None,
                },
                KeyDef {
                    label: "g",
                    width: 3,
                    finger: Finger::IndexLeft,
                    visual_width: None,
                },
                KeyDef {
                    label: "h",
                    width: 3,
                    finger: Finger::IndexRight,
                    visual_width: None,
                },
                KeyDef {
                    label: "j",
                    width: 3,
                    finger: Finger::IndexRight,
                    visual_width: None,
                },
                KeyDef {
                    label: "k",
                    width: 3,
                    finger: Finger::Middle,
                    visual_width: None,
                },
                KeyDef {
                    label: "l",
                    width: 3,
                    finger: Finger::Ring,
                    visual_width: None,
                },
                KeyDef {
                    label: ";",
                    width: 3,
                    finger: Finger::Pinky,
                    visual_width: None,
                },
                KeyDef {
                    label: "'",
                    width: 3,
                    finger: Finger::Pinky,
                    visual_width: None,
                },
                KeyDef {
                    label: "↵",
                    width: 5,
                    finger: Finger::Pinky,
                    visual_width: Some(1),
                },
            ],
            // Row 3: Bottom row (ZXCV)
            vec![
                KeyDef {
                    label: "⇧",
                    width: 7,
                    finger: Finger::Pinky,
                    visual_width: Some(1),
                },
                KeyDef {
                    label: "z",
                    width: 3,
                    finger: Finger::Pinky,
                    visual_width: None,
                },
                KeyDef {
                    label: "x",
                    width: 3,
                    finger: Finger::Ring,
                    visual_width: None,
                },
                KeyDef {
                    label: "c",
                    width: 3,
                    finger: Finger::Middle,
                    visual_width: None,
                },
                KeyDef {
                    label: "v",
                    width: 3,
                    finger: Finger::IndexLeft,
                    visual_width: None,
                },
                KeyDef {
                    label: "b",
                    width: 3,
                    finger: Finger::IndexLeft,
                    visual_width: None,
                },
                KeyDef {
                    label: "n",
                    width: 3,
                    finger: Finger::IndexRight,
                    visual_width: None,
                },
                KeyDef {
                    label: "m",
                    width: 3,
                    finger: Finger::IndexRight,
                    visual_width: None,
                },
                KeyDef {
                    label: ",",
                    width: 3,
                    finger: Finger::Middle,
                    visual_width: None,
                },
                KeyDef {
                    label: ".",
                    width: 3,
                    finger: Finger::Ring,
                    visual_width: None,
                },
                KeyDef {
                    label: "/",
                    width: 3,
                    finger: Finger::Pinky,
                    visual_width: None,
                },
                KeyDef {
                    label: "⇧",
                    width: 8,
                    finger: Finger::Pinky,
                    visual_width: Some(1),
                },
            ],
            // Row 4: Spacebar
            vec![KeyDef {
                label: " ",
                width: 15,
                finger: Finger::Thumb,
                visual_width: None,
            }],
        ];

        let home_row = vec!['a', 's', 'd', 'f', 'j', 'k', 'l', ';'];

        Self { rows, home_row }
    }
// TODO: Add functionality to shift keys
    #[allow(dead_code)]
    pub fn get_finger(&self, key: char) -> Option<Finger> {
        let key_lower = key.to_ascii_lowercase();
        for row in &self.rows {
            for key_def in row {
                if key_def.label.chars().next().map(|c| c.to_ascii_lowercase()) == Some(key_lower) {
                    return Some(key_def.finger);
                }
            }
        }
        None
    }

    pub fn is_home_row(&self, key: char) -> bool {
        self.home_row.contains(&key.to_ascii_lowercase())
    }

    pub fn get_rows(&self) -> &Vec<Vec<KeyDef>> {
        &self.rows
    }
}

impl Default for KeyboardLayout {
    fn default() -> Self {
        Self::new()
    }
}
