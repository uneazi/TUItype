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
            (KeyCode::Char('t'), mods, _) if mods.contains(KeyModifiers::CONTROL) => {
                AppAction::CycleTheme
            }

            // Toggle keyboard
            (KeyCode::Char('f'), mods, _) if mods.contains(KeyModifiers::CONTROL) => {
                AppAction::ToggleKeyboard
            }

            // New quote / restart
            (KeyCode::Char('n'), mods, _) if mods.contains(KeyModifiers::CONTROL) => {
                AppAction::NewQuote
            }
            (KeyCode::Char('r'), mods, _) if mods.contains(KeyModifiers::CONTROL) => {
                AppAction::Restart
            }

            // History view
            (KeyCode::Char('h'), mods, _) if mods.contains(KeyModifiers::CONTROL) => {
                AppAction::ShowHistory
            }

            // Stats view
            (KeyCode::Char('s'), mods, _) if mods.contains(KeyModifiers::CONTROL) => {
                AppAction::ShowStats
            }

            // Escape to go back
            (KeyCode::Esc, _, AppState::History | AppState::Stats) => AppAction::BackToTesting,

            // Navigation in history/stats
            (KeyCode::Up, _, AppState::History | AppState::Stats) => AppAction::NavigateUp,
            (KeyCode::Down, _, AppState::History | AppState::Stats) => AppAction::NavigateDown,

            // Select/Enter
            (KeyCode::Enter, _, _) => {
                if is_complete && (state == AppState::Testing || state == AppState::Results) {
                    AppAction::NewQuote
                } else {
                    AppAction::Select
                }
            }

            // Space handling - NewQuote for Results, Restart for Testing
            (KeyCode::Char(' '), _, _) => {
                if is_complete && state == AppState::Testing {
                    AppAction::Restart
                } else if is_complete && state == AppState::Results {
                    AppAction::NewQuote
                } else {
                    AppAction::TypeChar(' ')
                }
            }

            // Character input during testing
            (KeyCode::Char(c), mods, AppState::Testing) if mods.contains(KeyModifiers::SHIFT) => {
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

            // Ctrl+Backspace / Ctrl+H shows history (must be before general backspace arms)
            (KeyCode::Backspace, mods, _) if mods.contains(KeyModifiers::CONTROL) => {
                AppAction::ShowHistory
            }

            // Backspace handling
            (KeyCode::Backspace, mods, AppState::Testing) if mods.contains(KeyModifiers::ALT) => {
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
