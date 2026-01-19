use ratatui::{
    Frame,
    layout::Rect,
    widgets::{Block, Borders, List, ListItem},
    style::{Style, Color, Modifier},
    text::{Line, Span},
};
use crate::models::TestResult;

pub struct HistoryView {
    pub results: Vec<TestResult>,
    pub selected: usize,
    pub scroll_offset: usize,
}

impl HistoryView {
    pub fn new(results: Vec<TestResult>) -> Self {
        Self {
            results,
            selected: 0,
            scroll_offset: 0,
        }
    }

    pub fn next(&mut self) {
        if self.selected < self.results.len().saturating_sub(1) {
            self.selected += 1;
            if self.selected >= self.scroll_offset + 10 {
                self.scroll_offset += 1;
            }
        }
    }

    pub fn previous(&mut self) {
        if self.selected > 0 {
            self.selected -= 1;
            if self.selected < self.scroll_offset {
                self.scroll_offset = self.selected;
            }
        }
    }

    pub fn draw(&self, frame: &mut Frame, area: Rect) {
        let items: Vec<ListItem> = self.results
            .iter()
            .enumerate()
            .skip(self.scroll_offset)
            .take(area.height.saturating_sub(2) as usize)
            .map(|(i, result)| {
                let line = Line::from(vec![
                    Span::raw(format!("{:19} ", result.timestamp.format("%Y-%m-%d %H:%M:%S"))),
                    Span::styled(
                        format!("{:>6.1} WPM ", result.wpm),
                        Style::default().fg(Color::Cyan),
                    ),
                    Span::styled(
                        format!("{:>5.1}% ", result.accuracy),
                        Style::default().fg(Color::Yellow),
                    ),
                    Span::raw(format!("[{}]", result.mode)),
                ]);

                let style = if i == self.selected {
                    Style::default().bg(Color::DarkGray).add_modifier(Modifier::BOLD)
                } else {
                    Style::default()
                };

                ListItem::new(line).style(style)
            })
            .collect();

        let list = List::new(items)
            .block(Block::default().borders(Borders::ALL).title(" Test History "));

        frame.render_widget(list, area);
    }
}

