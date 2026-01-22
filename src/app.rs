use std::time::{Duration, Instant};

use crossterm::event::KeyEvent;
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Wrap},
    Frame,
};

use crate::storage::db::Database;
use crate::storage::config::ConfigManager;
use crate::models::{AppConfig, TestResult};
use chrono::Utc;
use crate::quotes::{QuoteManager, QuoteMode};
use crate::theme::Theme;

pub enum AppState {
    Testing,
    Results,
    History,
    Stats,
}

pub struct App {
    quote: String,
    quote_source: String,
    pub quote_mode: QuoteMode,
    quote_manager: QuoteManager,
    typed: String,
    started_at: Option<Instant>,
    last_tick: Instant,
    wpm: f64,
    wpm_history: Vec<(Instant, f64)>,
    mistakes: usize,
    accuracy: f64,
    is_complete: bool,
    completed_at: Option<Instant>,
    final_wpm: f64,
    final_accuracy: f64,
    final_duration: Duration,
    pub state: AppState,
    pub db: Database,
    pub config: AppConfig,
    pub last_result: Option<TestResult>,
    theme: Theme,
}

impl App {
    pub fn new() -> anyhow::Result<Self> {
        let proj_dirs = directories::ProjectDirs::from("", "", "TypingTUI")
            .ok_or_else(|| anyhow::anyhow!("No home dir"))?;
        let data_dir = proj_dirs.data_dir();
        std::fs::create_dir_all(data_dir)?;
        let db_path = data_dir.join("typing.db");
        let db = Database::open(db_path.to_str().unwrap())?;
        let config_mgr = ConfigManager::new()?;
        let config = config_mgr.load()?;

        // Initialize quote manager
        let quote_manager = QuoteManager::new()?;
        let quote_mode = QuoteMode::Medium;

        // Get initial quote
        let quote_obj = quote_manager
            .get_random_quote(quote_mode)
            .ok_or_else(|| anyhow::anyhow!("No quotes available"))?;

        // Load theme from config
        let theme = Theme::from_name(&config.theme);

        Ok(Self {
            quote: quote_obj.text.clone(),
            quote_source: quote_obj.source.clone(),
            quote_mode,
            quote_manager,
            typed: String::new(),
            started_at: None,
            last_tick: Instant::now(),
            wpm: 0.0,
            wpm_history: Vec::new(),
            mistakes: 0,
            accuracy: 0.0,
            is_complete: false,
            completed_at: None,
            final_wpm: 0.0,
            final_accuracy: 0.0,
            final_duration: Duration::from_secs(0),
            state: AppState::Testing,
            db,
            config,
            last_result: None,
            theme,
        })
    }

    pub fn on_key(&mut self, key: KeyEvent) {
        use crossterm::event::{KeyCode, KeyModifiers};

        if self.is_complete {
            return;
        }

        if self.started_at.is_none() {
            self.started_at = Some(Instant::now());
        }

        match (key.code, key.modifiers) {
            (KeyCode::Char(c), _) => {
                let expected = self.quote.chars().nth(self.typed.len());
                if expected != Some(c) {
                    self.mistakes += 1;
                }
                self.typed.push(c);
            }
 
            (KeyCode::Backspace, KeyModifiers::ALT) => {
                // Alt+Backspace: delete whole word
                self.delete_word();
            }
 
            (KeyCode::Backspace, _) => {
                // Regular backspace
                self.typed.pop();
            }
 
            _ => {}
        }

        self.recalc_metrics();
        self.check_completion();
    }

    fn delete_word(&mut self) {
        // Find the start of the current word (from right)
        let mut start = self.typed.len();

        // Move left until we hit a non-word character or beginning
        while start > 0 {
            let ch = self.typed.as_bytes()[start - 1];
            if ch.is_ascii_whitespace() || !ch.is_ascii_alphanumeric() {
                break;
            }
            start -= 1;
        }

        // Remove characters from start to end
        self.typed.drain(start..);
    }

    pub fn on_tick(&mut self) {
        if self.is_complete {
            return;
        }

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

            // Record WPM samples for consistency calculation
            if self.wpm > 0.0 {
                self.wpm_history.push((Instant::now(), self.wpm));
            }
        } else {
            self.wpm = 0.0;
        }
    }

    fn calculate_raw_wpm(&self) -> f64 {
        if let Some(start) = self.started_at {
            let elapsed = start.elapsed().as_secs_f64().max(1.0 / 60.0);
            let total_chars = self.typed.len() as f64;  // All chars, including mistakes
            let words = total_chars / 5.0;
            words / (elapsed / 60.0)
        } else {
            0.0
        }
    }

    // Calculate WPM consistency
    fn calculate_consistency(&self) -> f64 {
        if self.wpm_history.len() < 2 {
            return 100.0;
        }

        let wpms: Vec<f64> = self. wpm_history.iter().map(|(_, wpm)|*wpm).collect();
        let mean = wpms.iter().sum::<f64>() / wpms.len() as f64;
        let variance = wpms
            .iter()
            .map(|x| (x - mean)
                .powi(2))
            .sum::<f64>() / wpms.len() as f64;
        let std_dev = variance.sqrt();

        // Convert to percentage (lower std_dev = higher consistency)
        ((mean - std_dev) / mean * 100.0).max(0.0).min(100.0)
    }

    fn check_completion(&mut self) {
        // Completion conditions:
        // 1. Typed length matches quote length
        // 2. Last character is correct
        if self.typed.len() == self.quote.len() {
            // Check if last character matches
            let last_typed = self.typed.chars().last();
            let last_quote = self.quote.chars().last();

            if last_typed == last_quote {
                // Mark as complete and freeze metrics
                self.is_complete = true;
                self.completed_at = Some(Instant::now());
                self.final_wpm = self.wpm;
                self.final_accuracy = self.accuracy;

                if let Some(start) = self.started_at {
                    self.final_duration = start.elapsed();
                }

            // Save to database
            self.finish_test();
            }
        }
    }

    pub fn reset(&mut self) {
        // Get a new random quote
        if let Some(quote_obj) = self.quote_manager.get_random_quote(self.quote_mode) {
            self.quote = quote_obj.text.clone();
            self.quote_source = quote_obj.source.clone();
        }
        self.typed.clear();
        self.started_at = None;
        self.wpm = 0.0;
        self.accuracy = 100.0;
        self.is_complete = false;
        self.completed_at = None;
        self.final_wpm = 0.0;
        self.final_accuracy = 0.0;
        self.final_duration = Duration::from_secs(0);
        self.last_tick = Instant::now();
        self.wpm_history.clear();
        self.mistakes = 0;
    }

    pub fn restart(&mut self) {
        self.typed.clear();
        self.started_at = None;
        self.wpm = 0.0;
        self.accuracy = 100.0;
        self.is_complete = false;
        self.completed_at = None;
        self.final_wpm = 0.0;
        self.final_accuracy = 0.0;
        self.final_duration = Duration::from_secs(0);
        self.last_tick = Instant::now();
        self.wpm_history.clear();
        self.mistakes = 0;
    }


    pub fn is_complete(&self) -> bool {
        self.is_complete
    }

    pub fn draw(&self, frame: &mut Frame) {
        if self.is_complete {
            self.draw_results(frame);
        } else {
            self.draw_typing_screen(frame);
        }
    }

    // Footer with quote source
    fn quote_footer<'a>(&'a self) -> Paragraph<'a> {
        Paragraph::new(format!("Source: {}", self.quote_source))
            .block(
                Block::default()
                    .borders(Borders::TOP)
                    .title(" Quote Attribution ")
                    .title_style(Style::default().fg(self.theme.title_color)),
            )
            .style(Style::default().fg(Color::DarkGray))
    }


    fn draw_typing_screen(&self, frame: &mut Frame) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints(
                [
                    Constraint::Length(5), // header 
                    Constraint::Min(3),    // quote
                    Constraint::Length(3), // footer
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
        let keybinds_line1 = Line::from(vec![
            Span::styled(
                " TAB: Mode | Ctrl+H: History | Ctrl+S: Stats ",
                Style::default().fg(Color::DarkGray),
            ),
        ]);
        // Second line: Keybinds
        let keybinds_line2 = Line::from(vec![
            Span::styled(
                " Ctrl+T: Theme | Ctrl+N: New Quote | Ctrl+R: Restart | `: Quit ",
                Style::default().fg(Color::DarkGray),
            ),
        ]);


        // Third line: Stats
        let stats_line = Line::from(vec![
            Span::styled(
                format!(" [{}] ", mode_str),
                Style::default().fg(self.theme.mode_color).add_modifier(Modifier::BOLD),
            ),
            Span::raw(" | "),
            Span::styled(
                format!(" WPM: {:>5.1} ", self.wpm),
                Style::default().fg(self.theme.wpm_color),
            ),
            Span::raw(" | "),
            Span::styled(
                format!(" Acc: {:>5.1}% ", self.accuracy),
                Style::default().fg(self.theme.accuracy_color),
            ),
            Span::raw(" | "),
            Span::styled(
                format!(" Errors: {} ", self.mistakes),
                Style::default().fg(self.theme.error_color),
            ),
        ]);


        // Combine both lines
        let header_text = vec![
            keybinds_line1,
            keybinds_line2,
            stats_line,
        ];

        let header = Paragraph::new(header_text).block(
            Block::default()
                .borders(Borders::BOTTOM)
                .title(" TUItype ")
                .title_style(Style::default().fg(self.theme.title_color)),
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

        let quote_spans = self.render_quote();

        let quote_block = Paragraph::new(quote_spans)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_style(Style::default()
                        .fg(self.theme.border_color)
                        .add_modifier(Modifier::BOLD))
                    .title(" ═══ QUOTE ═══ ")
                    .title_style(Style::default().fg(self.theme.title_color))
                    .title_alignment(Alignment::Center)
            )
            .alignment(Alignment::Center)
            .wrap(Wrap { trim: true })
            .style(Style::default().add_modifier(Modifier::BOLD));

        frame.render_widget(quote_block, vertical_chunks[1]);

        let footer = self.quote_footer();
        frame.render_widget(footer, chunks[2]);
    }

    fn draw_results(&self, frame: &mut Frame) {
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
        let duration_secs = self.final_duration.as_secs_f64();

        let results_text = vec![
            Line::from(""),
            Line::from(vec![Span::styled(
                "╔══════════════════════════╗",
                Style::default()
                    .fg(self.theme.success_color)
                    .add_modifier(Modifier::BOLD),
            )])
            .alignment(Alignment::Center),
            Line::from(vec![Span::styled(
                "║      TEST COMPLETE!      ║",
                Style::default()
                    .fg(self.theme.success_color)
                    .add_modifier(Modifier::BOLD),
            )])
            .alignment(Alignment::Center),
            Line::from(vec![Span::styled(
                "╚══════════════════════════╝",
                Style::default()
                    .fg(self.theme.success_color)
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
                    format!("{:.1}", self.final_wpm),
                    Style::default()
                        .fg(self.theme.wpm_color)
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
                    format!("{:.1}%", self.final_accuracy),
                    Style::default()
                        .fg(self.theme.accuracy_color)
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
                        .fg(self.theme.success_color)
                        .add_modifier(Modifier::BOLD),
                ),
                Span::styled(" to restart", Style::default().fg(Color::DarkGray)),
            ])
            .alignment(Alignment::Center),
            Line::from(vec![
                Span::styled("Press ", Style::default().fg(Color::DarkGray)),
                Span::styled(
                    "`",
                    Style::default()
                        .fg(Color::Red)
                        .add_modifier(Modifier::BOLD),
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
                        .fg(self.theme.success_color)
                        .add_modifier(Modifier::BOLD),
                )
                .title(" ═══ RESULTS ═══ ")
                .title_style(Style::default().fg(self.theme.title_color))
                .title_alignment(Alignment::Center),
        );

        frame.render_widget(results_block, horizontal_chunks[1]);

        let footer = self.quote_footer();
        frame.render_widget(footer, vertical_chunks[3]);
    }

    fn render_quote(&self) -> Line<'_> {
        let mut line = Line::default();

        let quote_chars: Vec<char> = self.quote.chars().collect();
        let typed_chars: Vec<char> = self.typed.chars().collect();
        let len = quote_chars.len();

        for i in 0..len {
            let expected = quote_chars[i];
            let typed = typed_chars.get(i).copied();

            let (ch_to_show, style) = match typed {
                Some(c) => {
                    if expected == ' ' && c != ' ' {
                        // SPECIAL CASE: space expected, wrong char typed
                        (c, Style::default()
                            .fg(self.theme.incorrect_char)
                            .add_modifier(Modifier::BOLD))
                    } else if c == expected {
                        // Correct
                        (expected, Style::default().fg(self.theme.correct_char))
                    } else {
                        // Incorrect (non-space expected, wrong char typed)
                        (expected, Style::default()
                            .fg(self.theme.incorrect_char)
                            .add_modifier(Modifier::BOLD))
                    }
                }
                None => {
                    // Not yet typed
                    (expected, Style::default().fg(self.theme.untyped_char))
                }
            };

            // Cursor highlight on next char to type
            let style = if i == typed_chars.len() && !self.is_complete {
                style
                    .fg(self.theme.cursor_fg)
                    .bg(self.theme.cursor_bg)
                    .add_modifier(Modifier::BOLD | Modifier::UNDERLINED)
            } else {
                style
            };

            line.spans.push(Span::styled(ch_to_show.to_string(), style));
        }

        line
    }

    pub fn finish_test(&mut self) {
        let result = TestResult {
            id: None,
            timestamp: Utc::now(),
            mode: "medium".to_string(),
            wpm: self.wpm,
            raw_wpm: self.calculate_raw_wpm(), // calculate separately
            accuracy: self.accuracy,
            consistency: self.calculate_consistency(),  // calculate from WPM samples
            quote_length: self.quote.len() as i64,
            duration_seconds: self.started_at.unwrap().elapsed().as_secs() as i64,
        };

        self.db.save_result(&result).ok();
        self.last_result = Some(result);
        self.state = AppState::Results;
    }

    pub fn change_mode(&mut self, mode: QuoteMode) {
        self.quote_mode = mode;
        self.reset(); // This will get a new quote in the new mode
    }

    pub fn show_history(&mut self) -> anyhow::Result<()> {
        self.state = AppState::History;
        Ok(())
    }

    pub fn show_stats(&mut self) -> anyhow::Result<()> {
        self.state = AppState::Stats;
        Ok(())
    }

    pub fn back_to_testing(&mut self) {
        self.state = AppState::Testing;
    }

    pub fn cycle_theme(&mut self) {
        let themes = Theme::available_themes();
        let current_index = themes
            .iter()
            .position(|&t| t == self.theme.name)
            .unwrap_or(0);
        let next_index = (current_index + 1) % themes.len();
        self.theme = Theme::from_name(themes[next_index]);
        // Update config
        self.config.theme = self.theme.name.clone();
        self.save_config().ok();
    }

    pub fn save_config(&self) -> anyhow::Result<()> {
        let config_mgr = ConfigManager::new()?;
        config_mgr.save(&self.config)?;
        Ok(())
    }
}
