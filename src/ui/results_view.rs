use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

use crate::core::typing_session::TypingSession;
use crate::theme::Theme;

pub struct ResultsView;

impl ResultsView {
    pub fn draw(frame: &mut Frame, session: &TypingSession, quote_source: &str, theme: &Theme) {
        // Create centered vertical layout
        let vertical_chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Percentage(20),
                Constraint::Min(15),
                Constraint::Percentage(20),
                Constraint::Length(3),
            ])
            .split(frame.area());

        // Create centered horizontal layout
        let horizontal_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(25),
                Constraint::Percentage(50),
                Constraint::Percentage(25),
            ])
            .split(vertical_chunks[1]);

        // Build results content
        let duration_secs = session.duration().as_secs_f64();
        let final_wpm = session.wpm();
        let final_accuracy = session.accuracy();

        let results_text = vec![
            Line::from(""),
            Line::from(vec![Span::styled(
                "╔══════════════════════════╗",
                Style::default()
                    .fg(theme.success_color)
                    .add_modifier(Modifier::BOLD),
            )])
            .alignment(Alignment::Center),
            Line::from(vec![Span::styled(
                "║      TEST COMPLETE!      ║",
                Style::default()
                    .fg(theme.success_color)
                    .add_modifier(Modifier::BOLD),
            )])
            .alignment(Alignment::Center),
            Line::from(vec![Span::styled(
                "╚══════════════════════════╝",
                Style::default()
                    .fg(theme.success_color)
                    .add_modifier(Modifier::BOLD),
            )])
            .alignment(Alignment::Center),
            Line::from(""),
            Line::from(""),
            Line::from(vec![
                Span::styled(
                    "WPM: ",
                    Style::default()
                        .fg(Color::White)
                        .add_modifier(Modifier::BOLD),
                ),
                Span::styled(
                    format!("{:.1}", final_wpm),
                    Style::default()
                        .fg(theme.wpm_color)
                        .add_modifier(Modifier::BOLD),
                ),
            ])
            .alignment(Alignment::Center),
            Line::from(""),
            Line::from(vec![
                Span::styled(
                    "Accuracy: ",
                    Style::default()
                        .fg(Color::White)
                        .add_modifier(Modifier::BOLD),
                ),
                Span::styled(
                    format!("{:.1}%", final_accuracy),
                    Style::default()
                        .fg(theme.accuracy_color)
                        .add_modifier(Modifier::BOLD),
                ),
            ])
            .alignment(Alignment::Center),
            Line::from(""),
            Line::from(vec![
                Span::styled(
                    "Time: ",
                    Style::default()
                        .fg(Color::White)
                        .add_modifier(Modifier::BOLD),
                ),
                Span::styled(
                    format!("{:.2}s", duration_secs),
                    Style::default()
                        .fg(Color::Magenta)
                        .add_modifier(Modifier::BOLD),
                ),
            ])
            .alignment(Alignment::Center),
            Line::from(""),
            Line::from(""),
            Line::from(vec![Span::styled(
                "─────────────────────────────",
                Style::default().fg(Color::DarkGray),
            )])
            .alignment(Alignment::Center),
            Line::from(""),
            Line::from(vec![
                Span::styled("Press ", Style::default().fg(Color::DarkGray)),
                Span::styled(
                    "SPACE",
                    Style::default()
                        .fg(theme.success_color)
                        .add_modifier(Modifier::BOLD),
                ),
                Span::styled(" to restart", Style::default().fg(Color::DarkGray)),
            ])
            .alignment(Alignment::Center),
            Line::from(vec![
                Span::styled("Press ", Style::default().fg(Color::DarkGray)),
                Span::styled(
                    "`",
                    Style::default().fg(Color::Red).add_modifier(Modifier::BOLD),
                ),
                Span::styled(" to quit", Style::default().fg(Color::DarkGray)),
            ])
            .alignment(Alignment::Center),
        ];

        let results_block = Paragraph::new(results_text).block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(
                    Style::default()
                        .fg(theme.success_color)
                        .add_modifier(Modifier::BOLD),
                )
                .title(" ═══ RESULTS ═══ ")
                .title_style(Style::default().fg(theme.title_color))
                .title_alignment(Alignment::Center),
        );

        frame.render_widget(results_block, horizontal_chunks[1]);

        // Footer with quote source
        let footer = Paragraph::new(format!("Source: {}", quote_source))
            .block(
                Block::default()
                    .borders(Borders::TOP)
                    .title("Quote Attribution ")
                    .title_style(Style::default().fg(theme.title_color)),
            )
            .style(Style::default().fg(Color::DarkGray));

        frame.render_widget(footer, vertical_chunks[3]);
    }
}
