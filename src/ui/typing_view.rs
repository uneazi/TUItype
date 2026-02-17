use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Wrap},
    Frame,
};

use crate::core::typing_session::TypingSession;
use crate::quotes::QuoteMode;
use crate::theme::Theme;
use crate::ui::keyboard::render_keyboard;

pub struct TypingView {
    show_keyboard: bool,
    pressed_keys: Vec<char>,
    quote_mode: QuoteMode,
}

impl TypingView {
    pub fn new(show_keyboard: bool, quote_mode: QuoteMode) -> Self {
        Self {
            show_keyboard,
            pressed_keys: Vec::new(),
            quote_mode,
        }
    }

    pub fn show_keyboard(&self) -> bool {
        self.show_keyboard
    }

    pub fn draw(
        &self,
        frame: &mut Frame,
        session: &TypingSession,
        quote_source: &str,
        theme: &Theme,
        animated_wpm: f64,
    ) {
        let keyboard_height: u16 = if self.show_keyboard { 11 } else { 0 };

        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints(
                [
                    Constraint::Length(5),               // header
                    Constraint::Min(3),                  // quote
                    Constraint::Length(keyboard_height), // keyboard (optional)
                    Constraint::Length(3),               // footer
                ]
                .as_ref(),
            )
            .split(frame.area());

        // Build mode string
        let mode_str = match self.quote_mode {
            QuoteMode::Short => "SHORT",
            QuoteMode::Medium => "MEDIUM",
            QuoteMode::Long => "LONG",
        };

        // First line: Keybinds
        let keybinds_line1 = Line::from(vec![Span::styled(
            " TAB: Mode | Ctrl+H: History | Ctrl+S: Stats | Ctrl+F: Keyboard ",
            Style::default().fg(Color::DarkGray),
        )]);
        // Second line: Keybinds
        let keybinds_line2 = Line::from(vec![Span::styled(
            " Ctrl+T: Theme | Ctrl+N: New Quote | Ctrl+R: Restart | `: Quit ",
            Style::default().fg(Color::DarkGray),
        )]);

        // Third line: Stats
        let stats_line = Line::from(vec![
            Span::styled(
                format!(" [{}] ", mode_str),
                Style::default()
                    .fg(theme.mode_color)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::raw(" | "),
            Span::styled(
                format!(" WPM: {:>5.1} ", animated_wpm),
                Style::default().fg(theme.wpm_color),
            ),
            Span::raw(" | "),
            Span::styled(
                format!(" Acc: {:>5.1}% ", session.accuracy()),
                Style::default().fg(theme.accuracy_color),
            ),
            Span::raw(" | "),
            Span::styled(
                format!(" Errors: {} ", session.mistakes()),
                Style::default().fg(theme.error_color),
            ),
        ]);

        // Combine both lines
        let header_text = vec![keybinds_line1, keybinds_line2, stats_line];

        let header = Paragraph::new(header_text).block(
            Block::default()
                .borders(Borders::BOTTOM)
                .title(" TUItype ")
                .title_style(Style::default().fg(theme.title_color)),
        );
        frame.render_widget(header, chunks[0]);

        let quote_area = chunks[1];
        let horizontal_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(20),
                Constraint::Percentage(60),
                Constraint::Percentage(20),
            ])
            .split(quote_area);

        let vertical_chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Percentage(30),
                Constraint::Min(5),
                Constraint::Percentage(30),
            ])
            .split(horizontal_chunks[1]);

        let quote_spans = render_quote(session, theme);

        // Calculate scroll to keep cursor visible
        let inner_width = vertical_chunks[1].width.saturating_sub(2); // subtract borders
        let cursor_row = calculate_cursor_row(session, inner_width as usize);
        let height = vertical_chunks[1].height.saturating_sub(2); // subtract borders

        // Center the cursor
        let scroll_offset = if cursor_row > height / 2 {
            cursor_row - height / 2
        } else {
            0
        };

        let quote_block = Paragraph::new(quote_spans)
            .scroll((scroll_offset, 0))
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_style(
                        Style::default()
                            .fg(theme.border_color)
                            .add_modifier(Modifier::BOLD),
                    )
                    .title(" ═══ QUOTE ═══ ")
                    .title_style(Style::default().fg(theme.title_color))
                    .title_alignment(Alignment::Center),
            )
            .alignment(Alignment::Center)
            .wrap(Wrap { trim: true })
            .style(Style::default().add_modifier(Modifier::BOLD));

        frame.render_widget(quote_block, vertical_chunks[1]);

        // Footer with quote source
        let footer = Paragraph::new(format!("Source: {}", quote_source))
            .block(
                Block::default()
                    .borders(Borders::TOP)
                    .title("Quote Attribution ")
                    .title_style(Style::default().fg(theme.title_color)),
            )
            .style(Style::default().fg(Color::DarkGray));

        frame.render_widget(footer, chunks[3]);

        if self.show_keyboard {
            let next_char = session.quote().chars().nth(session.typed().len());
            render_keyboard(
                chunks[2],
                frame.buffer_mut(),
                next_char,
                &self.pressed_keys,
                theme,
            );
        }
    }
}

fn render_quote<'a>(session: &'a TypingSession, theme: &'a Theme) -> Line<'a> {
    let mut line = Line::default();

    let quote_chars: Vec<char> = session.quote().chars().collect();
    let typed_chars: Vec<char> = session.typed().chars().collect();
    let len = quote_chars.len();

    for i in 0..len {
        let expected = quote_chars[i];
        let typed = typed_chars.get(i).copied();

        let (ch_to_show, style) = match typed {
            Some(c) => {
                if expected == ' ' && c != ' ' {
                    // SPECIAL CASE: space expected, wrong char typed
                    (
                        c,
                        Style::default()
                            .fg(theme.incorrect_char)
                            .add_modifier(Modifier::BOLD),
                    )
                } else if c == expected {
                    // Correct
                    (expected, Style::default().fg(theme.correct_char))
                } else {
                    // Incorrect (non-space expected, wrong char typed)
                    (
                        expected,
                        Style::default()
                            .fg(theme.incorrect_char)
                            .add_modifier(Modifier::BOLD),
                    )
                }
            }
            None => {
                // Not yet typed
                (expected, Style::default().fg(theme.untyped_char))
            }
        };

        // Cursor highlight on next char to type
        let style = if i == typed_chars.len() && !session.is_complete() {
            style
                .fg(theme.cursor_fg)
                .bg(theme.cursor_bg)
                .add_modifier(Modifier::BOLD | Modifier::UNDERLINED)
        } else {
            style
        };

        line.spans.push(Span::styled(ch_to_show.to_string(), style));
    }

    line
}

fn calculate_cursor_row(session: &TypingSession, width: usize) -> u16 {
    if width < 2 {
        return 0;
    }
    let cursor = session.typed().len();

    let mut row = 0;
    let mut line_len = 0;

    let chars: Vec<char> = session.quote().chars().collect();
    let mut i = 0;

    while i < chars.len() {
        // Find word extent
        let start = i;
        while i < chars.len() && chars[i] != ' ' {
            i += 1;
        }
        let end = i;
        let word_len = end - start;

        // Calculate if word fits
        // Space is needed if not start of line
        let space = if line_len == 0 { 0 } else { 1 };

        if line_len + space + word_len > width {
            row += 1;
            line_len = 0;
        }

        // Add word
        if line_len > 0 {
            line_len += 1;
        }
        line_len += word_len;

        // Check cursor (word)
        if cursor >= start && cursor <= end {
            return row;
        }

        // Handle spaces after word
        while i < chars.len() && chars[i] == ' ' {
            i += 1;
        }

        // Check cursor (spaces)
        // If cursor is in the spaces we just skipped (start was `end`, now `i`)
        // Range (end, i]
        if cursor > end && cursor <= i {
            return row;
        }
    }

    row
}
