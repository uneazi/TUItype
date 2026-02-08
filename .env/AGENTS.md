# AGENTS.md

This file provides guidelines and instructions for AI agents working on the TUItype codebase.

## Project Overview

TUItype is a fast, keyboard-focused typing test built in Rust for the terminal. It uses:
- **ratatui** (v0.30) for TUI rendering
- **crossterm** (v0.29) for terminal abstraction
- **rusqlite** with bundled SQLite for persistence
- **chrono** for date/time handling
- **anyhow** for error handling
- Edition: Rust 2024

## Build Commands

### Development
```bash
cargo run                    # Run with debug build (fastest compile)
cargo run --release          # Run with optimizations
```

### Production Build
```bash
cargo build --release        # Build optimized binary
# Output: target/release/TUItype
```

### Running Tests
```bash
cargo test                   # Run all tests
cargo test --release        # Run tests with release optimizations
cargo test -- <pattern>     # Run specific test by name
cargo test --lib            # Run library tests only
cargo test --doc            # Run documentation tests
```

### Linting and Formatting
```bash
cargo fmt                   # Format code (uses rustfmt)
cargo fmt -- --check        # Check formatting without modifying
cargo clippy                # Run clippy lints
cargo clippy --fix          # Auto-fix clippy warnings
```

### Documentation
```bash
cargo doc --no-deps         # Generate documentation
cargo doc --open            # Generate and open docs
```

### Full CI Pipeline (what GitHub Actions runs)
```bash
cargo build --release --locked
cargo test
cargo fmt -- --check
cargo clippy
cargo doc --no-deps
```

## Code Style Guidelines

### Formatting
- Use default rustfmt settings (4-space indent, hard tabs not used)
- Maximum line length: 100 characters (default)
- Use trailing commas in multi-line expressions
- Place opening braces on same line as function/trait definitions

### Imports
Group imports in this order with blank lines between groups:
1. Standard library imports (`std::` or `core::`)
2. External crate imports
3. Local module imports (`crate::` or `super::`)

```rust
use std::io;

use crossterm::{
    event::{self, Event, KeyCode},
    terminal::{disable_raw_mode, enable_raw_mode},
};
use ratatui::{backend::CrosstermBackend, Terminal};

mod app;
mod models;

use crate::app::App;
```

### Naming Conventions
- **Modules**: snake_case (e.g., `storage/db`, `ui/mod`)
- **Structs**: PascalCase (e.g., `App`, `TestResult`, `Theme`)
- **Enums**: PascalCase (e.g., `AppState`, `QuoteMode`)
- **Fields/Variables**: snake_case (e.g., `quote_mode`, `started_at`)
- **Functions**: snake_case (e.g., `on_key`, `recalc_metrics`)
- **Constants**: SCREAMING_SNAKE_CASE for const values, snake_case for static
- **Type Parameters**: Single uppercase letter (e.g., `T`, `E`)

### Error Handling
- Use `anyhow::Result<T>` for functions that may fail at the application level
- Use `thiserror` for library-level errors with specific error types
- Use `?` operator for early returns
- Wrap errors with context using `.context()` or `with_context()`
- For I/O errors that need specific handling, use `io::ErrorKind`

```rust
pub fn new() -> anyhow::Result<Self> {
    let proj_dirs = ProjectDirs::from("", "", "TUItype")
        .ok_or_else(|| anyhow::anyhow!("No home dir"))?;
    // ...
}

fn read_config(path: &Path) -> Result<Config> {
    let content = std::fs::read_to_string(path)
        .with_context(|| format!("Failed to read {}", path.display()))?;
    // ...
}
```

### Enums
Prefer enums over boolean flags. Use `#[derive(Debug, Clone)]` for enums used across modules.

```rust
#[derive(Debug, Clone)]
pub enum AppState {
    Testing,
    Results,
    History,
    Stats,
}
```

### Structs and Data Types
- Use `pub` visibility for fields that need external access
- Use `Option<T>` for nullable values
- Use `#[derive(Debug, Clone, Serialize, Deserialize)]` for models
- Use explicit `usize` for collection indices and counts

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestResult {
    pub id: Option<i64>,
    pub timestamp: DateTime<Utc>,
    pub wpm: f64,
    pub accuracy: f64,
}
```

### Match Expressions
Use match for exhaustive enum handling. Avoid `if let` when full exhaustiveness matters.

```rust
match app.state {
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
}
```

### UI/Ratatui Patterns
- Create separate view structs for each screen (e.g., `StatsView`, `HistoryView`)
- Views should implement `new()` and `draw()` methods
- Use `Layout::default()` with `Constraint` for positioning
- Use `Style::default()` with chained methods for styling
- Use `Span::styled()` for colored text segments

```rust
pub struct StatsView {
    stats: UserStats,
}

impl StatsView {
    pub fn new(stats: UserStats) -> Self {
        Self { stats }
    }

    pub fn draw(&self, frame: &mut Frame, area: Rect) {
        // Layout and rendering logic
    }
}
```

### Database Operations
- Use `rusqlite` with prepared statements
- Use `params![]` macro for parameterized queries
- Wrap SQLite errors with `anyhow` context

```rust
pub fn save_result(&self, result: &TestResult) -> Result<i64> {
    self.conn.execute(
        "INSERT INTO test_results ...",
        params![
            result.timestamp,
            result.wpm,
            // ...
        ],
    )?;
    Ok(self.conn.last_insert_rowid())
}
```

### Testing
- Tests live in `src/ui/test.rs` (currently empty) or as `#[cfg(test)]` modules
- Use `#[test]` attribute for test functions
- Use `assert!`, `assert_eq!`, `assert_ne!` for assertions
- Group related tests with `mod tests { ... }`

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_wpm_calculation() {
        let app = App::new().unwrap();
        // Test logic
    }
}
```

### Documentation
- Use `///` for public API docs (Rustdoc)
- Use `//!` for module-level docs
- Document all `pub` functions and types
- Include examples in doc comments where helpful

```rust
/// Creates a new theme with the specified color scheme.
///
/// # Arguments
///
/// * `name` - The name of the theme (case-insensitive)
///
/// # Returns
///
/// A new `Theme` instance with the specified colors.
pub fn from_name(name: &str) -> Self {
    // ...
}
```

### Git Conventions
- Feature branches: `feature/<name>`
- Bug fixes: `fix/<description>`
- Commit messages: Imperative mood, short summary line (50 chars max)

```bash
git checkout -b feature/new-theme
git commit -m "Add Dracula theme with custom RGB colors"
```

### File Organization
```
src/
├── main.rs           # Entry point, terminal setup, event loop
├── app.rs            # Core App struct and typing logic
├── models.rs         # Data structures (TestResult, UserStats, AppConfig)
├── quotes.rs         # Quote loading and QuoteMode enum
├── theme.rs          # Theme definitions and colors
├── storage/
│   ├── mod.rs
│   ├── db.rs         # Database operations
│   ├── config.rs     # Config file management
│   └── history.rs
└── ui/
    ├── mod.rs
    ├── history.rs    # History view rendering
    ├── stats.rs      # Statistics view rendering
    └── test.rs       # Tests (currently empty)
```

### Performance Considerations
- Use `cargo build --release` for production performance
- Minimize allocations in hot paths (typing feedback loop)
- Use `&str` instead of `String` where possible
- Reuse buffers when drawing frames
- Database operations should be fast (already optimized with indexes)

### Common Patterns
1. **Early returns with `?`**: Prefer early returns over nested conditionals
2. **Builder pattern**: Not heavily used; direct struct initialization preferred
3. **Option/Result chaining**: Use `.and_then()`, `.map()`, `.unwrap_or()` for composition
4. **Event polling**: Uses `event::poll(Duration::from_millis(16))` (~60fps)
5. **Animation**: Smooth WPM counter using incremental interpolation in `on_tick()`

### Key Architecture Notes
- Single `App` struct manages global state
- `AppState` enum tracks current screen (Testing, Results, History, Stats)
- Database persists across sessions
- Config auto-saves on theme changes
- Quotes are pre-loaded from embedded sources
- Animated WPM display uses `animated_wpm` field with 15% interpolation per tick
