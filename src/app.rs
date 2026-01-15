use std::time::{Duration, Instant};

use crossterm::event::KeyEvent;
use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

const QUOTE: &str = "The quick brown fox jumps over the lazy dog.";

pub struct App {
    quote: String,
    typed: String,
    started_at: Option<Instant>,
    last_tick: Instant,
    wpm: f64,
    accuracy: f64,
}

impl App {
    pub fn new() -> Self {
        Self {
            quote: QUOTE.to_string(),
            typed: String::new(),
            started_at: None,
            last_tick: Instant::now(),
            wpm: 0.0,
            accuracy: 100.0,
        }
    }

    pub fn on_key(&mut self, key: KeyEvent) {
        use crossterm::event::KeyCode;

        // Start timer on first key
        if self.started_at.is_none() {
            self.started_at = Some(Instant::now());
        }

        match key.code {
            KeyCode::Char(c) => {
                self.typed.push(c);
            }
            KeyCode::Backspace => {
                self.typed.pop();
            }
            _ => {}
        }

        self.recalc_metrics();
    }

    pub fn on_tick(&mut self) {
        let now = Instant::now();
        if now.duration_since(self.last_tick) >= Duration::from_millis(250) {
            self.last_tick = now;
            self.recalc_metrics();
        }
    }

    fn recalc_metrics(&mut self) {
        // Accuracy
        let mut correct = 0usize;
        let attempted = self.typed.len().max(1); // avoid div by zero

        for (i, ch) in self.typed.chars().enumerate() {
            if self.quote.chars().nth(i) == Some(ch) {
                correct += 1;
            }
        }

        self.accuracy = (correct as f64 / attempted as f64) * 100.0;

        // WPM
        if let Some(start) = self.started_at {
            let elapsed = start.elapsed().as_secs_f64().max(1.0 / 60.0);
            let chars_typed = self.typed.len() as f64;
            let words = chars_typed / 5.0;
            self.wpm = words / (elapsed / 60.0);
        } else {
            self.wpm = 0.0;
        }
    }

    pub fn draw(&self, frame: &mut Frame) {
       let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints(
                [
                    Constraint::Length(3), // header
                    Constraint::Min(3),    // quote
                    Constraint::Length(3), // footer
                ]
                .as_ref(),
            )
            .split(frame.area());

        // Header: stats
        let header_text = Line::from(vec![
            Span::styled(
                format!(" WPM: {:>5.1}  ", self.wpm),
                Style::default().fg(Color::Cyan),
            ),
            Span::raw(" | "),
            Span::styled(
                format!(" Acc: {:>5.1}% ", self.accuracy),
                Style::default().fg(Color::Yellow),
            ),
            Span::raw(" | "),
            Span::styled(
                " Press 'q' to quit ",
                Style::default().fg(Color::DarkGray),
            ),
        ]);

        let header = Paragraph::new(header_text).block(
            Block::default()
                .borders(Borders::BOTTOM)
                .title(" MonkeyType TUI (prototype) "),
        );
        frame.render_widget(header, chunks[0]);

        // Middle: quote + typed overlay
        let quote_spans = self.render_quote();
        let quote_block = Paragraph::new(quote_spans).block(
            Block::default()
                .borders(Borders::ALL)
                .title(" Quote "),
        );
        frame.render_widget(quote_block, chunks[1]);

        // Footer: raw input
        let footer = Paragraph::new(self.typed.as_str()).block(
            Block::default()
                .borders(Borders::TOP)
                .title(" Your input "),
        );
        frame.render_widget(footer, chunks[2]);
    }

    
    fn render_quote(&self) -> Line<'_> {
        let mut line = Line::default();

        for (i, ch) in self.quote.chars().enumerate() {
            let style = match self.typed.chars().nth(i) {
                Some(typed_ch) if typed_ch == ch => {
                    Style::default().fg(Color::Green)
                }
                Some(_) => Style::default()
                    .fg(Color::Red)
                    .add_modifier(Modifier::BOLD),
                None => Style::default().fg(Color::DarkGray),
            };

            line.spans.push(Span::styled(ch.to_string(), style));
        }

        line
    }
}
