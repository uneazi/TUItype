use std::io;

use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind, KeyModifiers},
    terminal::{disable_raw_mode, enable_raw_mode},
};
use ratatui::{backend::CrosstermBackend, Terminal};

mod app;
mod models;
mod storage;
mod quotes;
mod ui;

use crate::app::{App, AppState};
use crate::ui:: history::HistoryView;

fn main() -> io::Result<()> {
    // 1. Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    crossterm::execute!(
        stdout,
        crossterm::terminal::EnterAlternateScreen,
        crossterm::event::EnableMouseCapture
    )?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // 2. Run app
    let res = run_app(&mut terminal);

    // 3. Restore terminal
    disable_raw_mode()?;
    crossterm::execute!(
        terminal.backend_mut(),
        crossterm::event::DisableMouseCapture,
        crossterm::terminal::LeaveAlternateScreen
    )?;
    terminal.show_cursor()?;

    // 4. Propagate any error after restoring terminal
    res
}

fn run_app(terminal: &mut Terminal<CrosstermBackend<io::Stdout>>) -> io::Result<()> {
    let mut app = App::new().map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
    let mut history_view: Option<HistoryView> = None;

    loop {
        // Draw UI based on state
        terminal.draw(|frame| {
            match &app.state {
                AppState::Testing | AppState::Results => {
                    app.draw(frame);
                }
                AppState::History => {
                    if let Some(ref view) = history_view {
                        view.draw(frame, frame.area());
                    }
                }
            }
        })?;

        // Handle input
        if event::poll(std::time::Duration::from_millis(16))? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    match (key.code, key.modifiers) {
                        (KeyCode::Char('`'), _) => {
                            break;
                        }

                        (KeyCode::Char('h'), KeyModifiers::CONTROL) => {
                            let results = app.db.get_recent_results(50)
                                .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
                            history_view = Some(HistoryView::new(results));
                            app.show_history()
                                .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
                        }

                        (KeyCode::Tab, _) => {
                            use crate::quotes::QuoteMode;
                            let next_mode = match app.quote_mode {
                                QuoteMode::Short => QuoteMode::Medium,
                                QuoteMode::Medium => QuoteMode::Long,
                                QuoteMode::Long => QuoteMode::Short,
                            };
                            app.change_mode(next_mode);
                        }

                        (KeyCode::Esc, _) => {
                            if matches!(app.state, AppState::History) {
                                app.back_to_testing();
                                history_view = None;
                            }
                        }

                        (KeyCode::Up, _) => {
                            if let Some(ref mut view) = history_view {
                                view.previous();
                            }
                        }
                        (KeyCode::Down, _) => {
                            if let Some(ref mut view) = history_view {
                                view.next();
                            }
                        }

                        (KeyCode::Char(' '), _) | (KeyCode::Enter, _) => {
                            if app.is_complete() {
                                app.reset();
                            } else {
                                app.on_key(key);
                            }
                        }

                        _ => {
                            app.on_key(key);
                        }
                    }
                }
            }
        }

        app.on_tick();
    }

    Ok(())
}
