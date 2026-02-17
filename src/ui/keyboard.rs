use ratatui::{buffer::Buffer, layout::Rect, style::Style};

use crate::keyboard::KeyboardLayout;
use crate::theme::Theme;

pub fn render_keyboard(
    area: Rect,
    buf: &mut Buffer,
    current_key: Option<char>,
    pressed_keys: &[char],
    theme: &Theme,
) {
    if area.width < 50 || area.height < 11 {
        return;
    }

    let layout = KeyboardLayout::new();
    let rows = layout.get_rows();
    let key_height = 1u16;
    let h_gap = 1u16;
    let v_gap = 1u16;

    // Calculate widths for each row based on actual key widths
    let row_widths: Vec<i32> = rows
        .iter()
        .map(|row| {
            row.iter().map(|k| k.width as i32).sum::<i32>()
                + ((row.len() as i32) - 1).max(0) * h_gap as i32
        })
        .collect();

    // Find maximum row width (row 0 is widest due to backspace)
    let max_row_width = *row_widths.iter().max().unwrap_or(&55);

    // Center the widest row in the full area
    let start_x = area.x as i32 + (area.width as i32 - max_row_width) / 2;
    let start_y = area.y + 1;

    // Standard ANSI keyboard stagger (in character positions)
    // Keys are now 3 chars wide + 1 gap = 4 per key unit
    // Row 0 (numbers): reference
    // Row 1 (QWERTY): +2 chars (half a key right - Tab key)
    // Row 2 (home):    +6 chars (one key right - Caps Lock key)
    // Row 3 (bottom): +8 chars (one and a half keys right - Left Shift key)
    let row_stagger = [0i32, 0, 0, 0, 0];

    // Calculate spacebar offset: centered under the home row
    let home_row_stagger = row_stagger[2];
    let home_row_width = row_widths[2];
    let spacebar_width = row_widths[4];
    let _spacebar_offset = home_row_stagger + (home_row_width - spacebar_width) / 2;

    for (row_idx, row) in rows.iter().enumerate() {
        let y = start_y + (row_idx as u16) * (key_height + v_gap);

        if y >= area.y + area.height {
            break;
        }

        // Use stagger for rows 0-3, calculated offset for spacebar row 4
        let x_offset = if row_idx == 4 {
            home_row_stagger + (home_row_width - spacebar_width) / 2
        } else {
            row_stagger[row_idx]
        };

        let mut col_pos = 0i32;
        for key_def in row {
            let key_width = key_def.width as i32;
            let x = start_x + x_offset + col_pos;

            let area_right = (area.x + area.width) as i32;
            if x + key_width > area_right || x < area.x as i32 {
                col_pos += key_width + h_gap as i32;
                continue;
            }

            let key_char = key_def.label.chars().next().unwrap_or(' ');

            let is_current = current_key
                .map(|c| c.to_ascii_lowercase() == key_char.to_ascii_lowercase())
                .unwrap_or(false);

            let is_pressed = !is_current
                && pressed_keys
                    .iter()
                    .any(|&c| c.to_ascii_lowercase() == key_char.to_ascii_lowercase());

            let is_home = layout.is_home_row(key_char);

            let finger_fg = match key_def.finger {
                crate::keyboard::Finger::Pinky => theme.finger_pinky,
                crate::keyboard::Finger::Ring => theme.finger_ring,
                crate::keyboard::Finger::Middle => theme.finger_middle,
                crate::keyboard::Finger::IndexLeft | crate::keyboard::Finger::IndexRight => {
                    theme.finger_index
                }
                crate::keyboard::Finger::Thumb => theme.finger_thumb,
            };

            let bg = if is_current {
                theme.current_key_highlight
            } else if is_pressed {
                finger_fg
            } else {
                theme.keyboard_key
            };

            // Render key background
            for dy in 0..key_height as i32 {
                for dx in 0..key_width {
                    let px = x + dx;
                    let py = y as i32 + dy;
                    if py < (area.y + area.height) as i32 && px < area_right && px >= area.x as i32
                    {
                        if let Some(cell) = buf.cell_mut((px as u16, py as u16)) {
                            let is_left_edge = dx == 0;
                            let is_right_edge = dx == key_width - 1;

                            let (char_to_render, bg_color, fg_color) = if is_current {
                                if is_left_edge {
                                    ('|', theme.keyboard_key, finger_fg)
                                } else if is_right_edge {
                                    ('|', theme.keyboard_key, finger_fg)
                                } else {
                                    (' ', theme.keyboard_key, theme.keyboard_key_text)
                                }
                            } else {
                                (' ', bg, theme.keyboard_key_text)
                            };

                            cell.set_char(char_to_render);
                            cell.set_style(Style::default().bg(bg_color).fg(fg_color));
                        }
                    }
                }
            }

            // Render key label (centered within the key)
            let label_len = key_def.visual_width.unwrap_or(key_def.label.len() as u8) as i32;
            let key_width = key_def.width as i32;
            if label_len > 0 && key_width >= label_len {
                let label_x = x + (key_width - label_len + 1) / 2;
                if label_x < area_right {
                    for (i, ch) in key_def.label.chars().enumerate() {
                        let px = label_x + (i as i32);
                        if px < area_right && px >= area.x as i32 {
                            if let Some(cell) = buf.cell_mut((px as u16, y)) {
                                cell.set_char(ch);
                                let mut modifiers = ratatui::style::Modifier::BOLD;
                                if is_home {
                                    modifiers |= ratatui::style::Modifier::UNDERLINED;
                                }
                                let label_bg = if is_current { theme.keyboard_key } else { bg };
                                cell.set_style(
                                    Style::default()
                                        .bg(label_bg)
                                        .fg(if is_current { finger_fg } else { finger_fg })
                                        .add_modifier(modifiers),
                                );
                            }
                        }
                    }
                }
            }

            col_pos += key_width + h_gap as i32;
        }
    }
}
