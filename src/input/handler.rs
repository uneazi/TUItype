use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use crate::state::AppState;

#[derive(Debug, Clone)]
pub enum AppAction {
    Quit,
    ShowHistory,
    ShowStats,
    BackToTesting,
    CycleTheme,
    CycleMode,
    NewQuote,
    Restart,
    ToggleKeyboard,
    TypeChar(char),
    Backspace,
    DeleteWord,
    NavigateUp,
    NavigateDown,
    Select,
    None,
}

pub struct InputHandler;

impl InputHandler {
    pub fn new() -> Self {
        Self
    }

    pub fn handle(&self, key: KeyEvent, state: AppState, is_complete: bool) -> AppAction {
        match (key.code, key.modifiers, state) {
            // Global quit
            (KeyCode::Char('`'), _, _) => AppAction::Quit,

            // Mode switching - always available
            (KeyCode::Tab, _, _) => AppAction::CycleMode,

            // Theme cycling
            (KeyCode::Char('t'), KeyModifiers::CONTROL, _) => AppAction::CycleTheme,

            // Toggle keyboard
            (KeyCode::Char('f'), KeyModifiers::CONTROL, _) => AppAction::ToggleKeyboard,

            // New quote / restart
            (KeyCode::Char('n'), KeyModifiers::CONTROL, _) => AppAction::NewQuote,
            (KeyCode::Char('r'), KeyModifiers::CONTROL, _) => AppAction::Restart,

            // History view
            (KeyCode::Char('h'), KeyModifiers::CONTROL, _) => AppAction::ShowHistory,

            // Stats view
            (KeyCode::Char('s'), KeyModifiers::CONTROL, _) => AppAction::ShowStats,

            // Escape to go back
            (KeyCode::Esc, _, AppState::History | AppState::Stats) => AppAction::BackToTesting,

            // Navigation in history/stats
            (KeyCode::Up, _, AppState::History | AppState::Stats) => AppAction::NavigateUp,
            (KeyCode::Down, _, AppState::History | AppState::Stats) => AppAction::NavigateDown,

            // Select/Enter
            (KeyCode::Enter, _, _) => {
                if is_complete && state == AppState::Testing {
                    AppAction::NewQuote
                } else {
                    AppAction::Select
                }
            }

            // Space handling
            (KeyCode::Char(' '), _, _) => {
                if is_complete && state == AppState::Testing {
                    AppAction::Restart
                } else {
                    AppAction::TypeChar(' ')
                }
            }

            // Character input during testing
            (KeyCode::Char(c), KeyModifiers::SHIFT, AppState::Testing) => {
                if !is_complete {
                    AppAction::TypeChar(c.to_ascii_uppercase())
                } else {
                    AppAction::None
                }
            }

            (KeyCode::Char(c), _, AppState::Testing) => {
                if !is_complete {
                    AppAction::TypeChar(c)
                } else {
                    AppAction::None
                }
            }

            // Backspace handling
            (KeyCode::Backspace, KeyModifiers::ALT, AppState::Testing) => {
                if !is_complete {
                    AppAction::DeleteWord
                } else {
                    AppAction::None
                }
            }

            (KeyCode::Backspace, _, AppState::Testing) => {
                if !is_complete {
                    AppAction::Backspace
                } else {
                    AppAction::None
                }
            }

            _ => AppAction::None,
        }
    }
}
