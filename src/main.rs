use std::io;

use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind},
    terminal::{disable_raw_mode, enable_raw_mode},
};
use ratatui::{backend::CrosstermBackend, Terminal};

mod app;
mod models;
mod storage;

use crate::app::App;

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

    loop {
        // Draw UI
        terminal.draw(|frame| {
            app.draw(frame);
        })?;

        // Handle input
        if event::poll(std::time::Duration::from_millis(16))? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    match key.code {
                        KeyCode::Char('`') => {
                            // Quit (works in both states)
                            break;
                        }
                        KeyCode::Char(' ') | KeyCode::Enter => {
                            // Restart if test is complete
                            if app.is_complete() {
                                app.reset();
                            } else {
                                // Normal typing: pass space to app
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

        // Tick (for timers etc.)
        app.on_tick();
    }

    Ok(())
}
