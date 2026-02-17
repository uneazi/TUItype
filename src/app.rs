use std::time::{Duration, Instant};

use crossterm::event::KeyEvent;
use ratatui::Frame;

use crate::core::metrics;
use crate::core::typing_session::TypingSession;
use crate::input::handler::{AppAction, InputHandler};
use crate::models::{AppConfig, TestResult};
use crate::quotes::{QuoteManager, QuoteMode};
use crate::state::{AppState, StateMachine};
use crate::storage::config::ConfigManager;
use crate::storage::db::Database;
use crate::theme::Theme;
use crate::ui::results_view::ResultsView;
use crate::ui::typing_view::TypingView;

pub struct App {
    // Core state
    state_machine: StateMachine,
    session: TypingSession,
    quote_source: String,
    quote_mode: QuoteMode,
    quote_manager: QuoteManager,

    // Configuration
    pub db: Database,
    pub config: AppConfig,
    theme: Theme,

    // UI state
    typing_view: TypingView,
    animated_wpm: f64,
    last_wpm_for_animation: f64,
    last_tick: Instant,

    // Input handling
    input_handler: InputHandler,
    pressed_keys: Vec<char>,
    pressed_key_timestamp: Option<Instant>,

    // Results
    pub last_result: Option<TestResult>,
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

        let session = TypingSession::new(quote_obj.text.clone());
        let typing_view = TypingView::new(false, quote_mode);

        Ok(Self {
            state_machine: StateMachine::new(AppState::Testing),
            session,
            quote_source: quote_obj.source.clone(),
            quote_mode,
            quote_manager,
            db,
            config,
            theme,
            typing_view,
            animated_wpm: 0.0,
            last_wpm_for_animation: 0.0,
            last_tick: Instant::now(),
            input_handler: InputHandler::new(),
            pressed_keys: Vec::new(),
            pressed_key_timestamp: None,
            last_result: None,
        })
    }

    pub fn handle_input(&mut self, key: KeyEvent) -> Option<AppAction> {
        let action = self
            .input_handler
            .handle(key, self.state(), self.session.is_complete());

        match &action {
            AppAction::TypeChar(c) => {
                let is_complete = self.session.type_char(*c);
                self.pressed_keys.clear();
                self.pressed_keys.push(*c);
                self.pressed_key_timestamp = Some(Instant::now());

                if is_complete {
                    self.finish_test();
                }
            }
            AppAction::Backspace => {
                self.session.backspace();
            }
            AppAction::DeleteWord => {
                self.session.delete_word();
            }
            AppAction::CycleMode => {
                self.quote_mode = match self.quote_mode {
                    QuoteMode::Short => QuoteMode::Medium,
                    QuoteMode::Medium => QuoteMode::Long,
                    QuoteMode::Long => QuoteMode::Short,
                };
                self.reset();
            }
            AppAction::NewQuote => {
                self.reset();
            }
            AppAction::Restart => {
                self.restart();
            }
            AppAction::ToggleKeyboard => {
                let new_show = !self.typing_view.show_keyboard();
                self.typing_view = TypingView::new(new_show, self.quote_mode);
            }
            AppAction::CycleTheme => {
                self.cycle_theme();
            }
            AppAction::ShowHistory => {
                self.state_machine.transition(AppState::History);
            }
            AppAction::ShowStats => {
                self.state_machine.transition(AppState::Stats);
            }
            AppAction::BackToTesting => {
                self.state_machine.transition(AppState::Testing);
            }
            _ => {}
        }

        Some(action)
    }

    pub fn on_tick(&mut self) {
        if self.session.is_complete() {
            return;
        }

        let now = Instant::now();
        if now.duration_since(self.last_tick) >= Duration::from_millis(250) {
            self.last_tick = now;
            self.session.update_metrics();
            self.update_wpm_animation();
        }

        if let Some(timestamp) = self.pressed_key_timestamp {
            if now.duration_since(timestamp) >= Duration::from_millis(120) {
                self.pressed_keys.clear();
                self.pressed_key_timestamp = None;
            }
        }
    }

    fn update_wpm_animation(&mut self) {
        self.animated_wpm = metrics::animate_wpm(
            self.animated_wpm,
            self.session.wpm(),
            &mut self.last_wpm_for_animation,
        );
    }

    pub fn draw(&self, frame: &mut Frame) {
        match self.state() {
            AppState::Testing if self.session.is_complete() => {
                ResultsView::draw(frame, &self.session, &self.quote_source, &self.theme);
            }
            AppState::Results => {
                ResultsView::draw(frame, &self.session, &self.quote_source, &self.theme);
            }
            AppState::Testing => {
                self.typing_view.draw(
                    frame,
                    &self.session,
                    &self.quote_source,
                    &self.theme,
                    self.animated_wpm,
                );
            }
            _ => {} // History and Stats are handled separately
        }
    }

    fn finish_test(&mut self) {
        if let Some(result) = self.session.final_result() {
            self.db.save_result(&result).ok();
            self.last_result = Some(result);
        }
        self.state_machine.transition(AppState::Results);
    }

    pub fn reset(&mut self) {
        if let Some(quote_obj) = self.quote_manager.get_random_quote(self.quote_mode) {
            self.session.reset(quote_obj.text.clone());
            self.quote_source = quote_obj.source.clone();
        }
        self.animated_wpm = 0.0;
        self.last_wpm_for_animation = 0.0;
        self.last_tick = Instant::now();
        self.state_machine = StateMachine::new(AppState::Testing);
        self.typing_view = TypingView::new(self.typing_view.show_keyboard(), self.quote_mode);
    }

    pub fn restart(&mut self) {
        self.session.restart();
        self.animated_wpm = 0.0;
        self.last_wpm_for_animation = 0.0;
        self.last_tick = Instant::now();
    }

    fn cycle_theme(&mut self) {
        let themes = Theme::available_themes();
        let current_index = themes
            .iter()
            .position(|&t| t == self.theme.name)
            .unwrap_or(0);
        let next_index = (current_index + 1) % themes.len();
        self.theme = Theme::from_name(themes[next_index]);
        self.config.theme = self.theme.name.clone();
        self.save_config().ok();
    }

    pub fn save_config(&self) -> anyhow::Result<()> {
        let config_mgr = ConfigManager::new()?;
        config_mgr.save(&self.config)?;
        Ok(())
    }

    // Getters
    pub fn state(&self) -> AppState {
        self.state_machine.current()
    }
}
