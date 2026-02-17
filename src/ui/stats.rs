use crate::models::UserStats;
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

pub struct StatsView {
    stats: UserStats,
}

impl StatsView {
    pub fn new(stats: UserStats) -> Self {
        Self { stats }
    }

    pub fn draw(&self, frame: &mut Frame, area: Rect) {
        // Center the stats box
        let vertical_chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Percentage(20),
                Constraint::Min(20),
                Constraint::Percentage(20),
            ])
            .split(area);

        let horizontal_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(25),
                Constraint::Percentage(50),
                Constraint::Percentage(25),
            ])
            .split(vertical_chunks[1]);

        // Format time
        let hours = self.stats.total_time_seconds / 3600;
        let minutes = (self.stats.total_time_seconds % 3600) / 60;
        let seconds = self.stats.total_time_seconds % 60;

        let time_str = if hours > 0 {
            format!("{}h {}m {}s", hours, minutes, seconds)
        } else if minutes > 0 {
            format!("{}m {}s", minutes, seconds)
        } else {
            format!("{}s", seconds)
        };

        // Build stats content
        let stats_text = vec![
            Line::from(""),
            Line::from(vec![Span::styled(
                "═══════════════════════════",
                Style::default()
                    .fg(Color::Cyan)
                    .add_modifier(Modifier::BOLD),
            )])
            .alignment(Alignment::Center),
            Line::from(vec![Span::styled(
                "   YOUR STATISTICS   ",
                Style::default()
                    .fg(Color::Cyan)
                    .add_modifier(Modifier::BOLD),
            )])
            .alignment(Alignment::Center),
            Line::from(vec![Span::styled(
                "═══════════════════════════",
                Style::default()
                    .fg(Color::Cyan)
                    .add_modifier(Modifier::BOLD),
            )])
            .alignment(Alignment::Center),
            Line::from(""),
            Line::from(""),
            Line::from(vec![
                Span::styled(
                    "Total Tests: ",
                    Style::default()
                        .fg(Color::White)
                        .add_modifier(Modifier::BOLD),
                ),
                Span::styled(
                    format!("{}", self.stats.total_tests),
                    Style::default()
                        .fg(Color::Yellow)
                        .add_modifier(Modifier::BOLD),
                ),
            ])
            .alignment(Alignment::Center),
            Line::from(""),
            Line::from(vec![
                Span::styled(
                    "Best WPM: ",
                    Style::default()
                        .fg(Color::White)
                        .add_modifier(Modifier::BOLD),
                ),
                Span::styled(
                    format!("{:.1}", self.stats.best_wpm),
                    Style::default()
                        .fg(Color::Green)
                        .add_modifier(Modifier::BOLD),
                ),
            ])
            .alignment(Alignment::Center),
            Line::from(""),
            Line::from(vec![
                Span::styled(
                    "Average WPM: ",
                    Style::default()
                        .fg(Color::White)
                        .add_modifier(Modifier::BOLD),
                ),
                Span::styled(
                    format!("{:.1}", self.stats.avg_wpm),
                    Style::default()
                        .fg(Color::Cyan)
                        .add_modifier(Modifier::BOLD),
                ),
            ])
            .alignment(Alignment::Center),
            Line::from(""),
            Line::from(vec![
                Span::styled(
                    "Average Accuracy: ",
                    Style::default()
                        .fg(Color::White)
                        .add_modifier(Modifier::BOLD),
                ),
                Span::styled(
                    format!("{:.1}%", self.stats.avg_accuracy),
                    Style::default()
                        .fg(Color::Magenta)
                        .add_modifier(Modifier::BOLD),
                ),
            ])
            .alignment(Alignment::Center),
            Line::from(""),
            Line::from(vec![
                Span::styled(
                    "Total Practice Time: ",
                    Style::default()
                        .fg(Color::White)
                        .add_modifier(Modifier::BOLD),
                ),
                Span::styled(
                    time_str,
                    Style::default()
                        .fg(Color::Yellow)
                        .add_modifier(Modifier::BOLD),
                ),
            ])
            .alignment(Alignment::Center),
            Line::from(""),
            Line::from(""),
            Line::from(vec![Span::styled(
                "───────────────────────────",
                Style::default().fg(Color::DarkGray),
            )])
            .alignment(Alignment::Center),
            Line::from(""),
            Line::from(vec![
                Span::styled("Press ", Style::default().fg(Color::DarkGray)),
                Span::styled(
                    "ESC",
                    Style::default().fg(Color::Red).add_modifier(Modifier::BOLD),
                ),
                Span::styled(" to go back", Style::default().fg(Color::DarkGray)),
            ])
            .alignment(Alignment::Center),
        ];

        let stats_block = Paragraph::new(stats_text).block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(
                    Style::default()
                        .fg(Color::Cyan)
                        .add_modifier(Modifier::BOLD),
                )
                .title(" ═══ STATISTICS ═══ ")
                .title_alignment(Alignment::Center),
        );

        frame.render_widget(stats_block, horizontal_chunks[1]);
    }
}
