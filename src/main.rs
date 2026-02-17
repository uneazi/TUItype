use std::io;

use crossterm::{
    event::{self, Event, KeyEventKind},
    terminal::{disable_raw_mode, enable_raw_mode},
};
use ratatui::{backend::CrosstermBackend, Terminal};

mod app;
mod core;
mod input;
mod keyboard;
mod models;
mod quotes;
mod state;
mod storage;
mod theme;
mod ui;

use crate::app::App;
use crate::input::handler::AppAction;
use crate::state::AppState;
use crate::ui::history::HistoryView;
use crate::ui::stats::StatsView;

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
    let mut stats_view: Option<StatsView> = None;

    loop {
        // Draw UI based on state
        terminal.draw(|frame| match app.state() {
            AppState::Testing | AppState::Results => {
                app.draw(frame);
            }
            AppState::History => {
                if let Some(ref view) = history_view {
                    view.draw(frame, frame.area());
                }
            }
            AppState::Stats => {
                if let Some(ref view) = stats_view {
                    view.draw(frame, frame.area());
                }
            }
        })?;

        // Handle input
        if event::poll(std::time::Duration::from_millis(16))? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    if let Some(action) = app.handle_input(key) {
                        match action {
                            AppAction::Quit => break,
                            AppAction::ShowHistory => {
                                let results = app
                                    .db
                                    .get_recent_results(50)
                                    .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
                                history_view = Some(HistoryView::new(results));
                            }
                            AppAction::ShowStats => {
                                let stats = app
                                    .db
                                    .get_stats()
                                    .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
                                stats_view = Some(StatsView::new(stats));
                            }
                            AppAction::BackToTesting => {
                                history_view = None;
                                stats_view = None;
                            }
                            AppAction::NavigateUp => {
                                if let Some(ref mut view) = history_view {
                                    view.previous();
                                }
                            }
                            AppAction::NavigateDown => {
                                if let Some(ref mut view) = history_view {
                                    view.next();
                                }
                            }
                            _ => {}
                        }
                    }
                }
            }
        }

        app.on_tick();
    }

    Ok(())
}
