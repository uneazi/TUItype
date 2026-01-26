# TUItype

A fast, keyboard-focused typing test built in Rust for the terminal. Type quotes at your own pace, track your stats, and improve your typing skills.

![Rust](https://img.shields.io/badge/Rust-1.70%2B-orange?logo=rust)
![License](https://img.shields.io/badge/License-MIT-blue)
![Platform](https://img.shields.io/badge/Platform-Linux%20%7C%20macOS%20%7C%20Windows-lightgrey)

## Features

- **Real-time typing test** with WPM (Words Per Minute) and accuracy metrics
- **Multiple difficulty modes**: Short, Medium, and Long quotes
- **Rich TUI interface** built with [ratatui](https://github.com/ratatui-org/ratatui) and [crossterm](https://github.com/crossterm-rs/crossterm)
- **Test history** — view your last 50 tests with timestamps and detailed stats
- **Statistics view** — analyze your performance trends, consistency, and raw WPM
- **Multiple themes** — Dark, Light, Nord, Dracula, Solarized, and Catppuccin Mocha
- **Auto-saving** — all results stored in a local SQLite database
- **Persistent config** — theme and preferences saved between sessions
- **Quote scrolling viewport** — large quotes stay visible without screen overflow
- **Error highlighting** — instantly see which characters you typed incorrectly

## Quick Start

### Installation

#### Windows (Prebuilt)

1. Download the latest release from [Releases](https://github.com/uneazi/TUItype/releases)
2. Extract `TUItype-windows-x86_64-vX.Y.Z.zip`
3. Run `TUItype.exe` from Command Prompt or PowerShell:
   ```powershell
   .\TUItype.exe
   ```
4. (Optional) Add the folder to your PATH to run `TUItype` from anywhere

#### Linux / macOS (Build from Source)

**Prerequisites:** [Rust 1.70+](https://rustup.rs/)

```bash
git clone https://github.com/uneazi/TUItype
cd TUItype
cargo build --release
./target/release/TUItype
```

## Usage

When you launch TUItype, you'll see a typing test screen with:

- **Header** — current mode, real-time WPM, accuracy, and error count
- **Quote box** — the text you're typing (with scrolling support for long quotes)
- **Footer** — quote attribution/source

### Keybinds

| Key | Action |
|-----|--------|
| `TAB` | Cycle through difficulty modes (Short → Medium → Long) |
| `Ctrl+H` | View test history (last 50 tests) |
| `Ctrl+S` | View statistics and trends |
| `Ctrl+T` | Cycle through color themes |
| `Ctrl+N` | Get a new quote in the current mode |
| `` ` `` | Quit the application |
| `Space` (after test complete) | Restart with a new quote |
| `Backspace` | Delete the last typed character |
| `Up/Down` (in history) | Navigate previous/next test |
| `Esc` (in history/stats) | Return to typing screen |

### Difficulty Modes

- **Short** — 20–40 word quotes; good for quick practice
- **Medium** — 40–80 word quotes; balanced challenge
- **Long** — 80–150 word quotes; full endurance test

## Screens

### Typing Screen

Real-time feedback as you type:

```
 TAB: Mode | Ctrl+H: History | Ctrl+S: Stats | Ctrl+T: Theme | Ctrl+N: New Quote | `: Quit 
 [MEDIUM]  | WPM: 72.3  | Acc: 98.5%  | Errors: 1

         ╔═══ QUOTE ═══╗
         ║ The quick   ║
         ║ brown fox   ║
         ║ jumps over  ║
         ║ the lazy... ║
         ╚═════════════╝

Source: Jane Austen, 1817
```

- **Green text** — correctly typed characters
- **Red text** — mistakes (bold highlight)
- **Gray text** — untyped characters ahead
- **Bold cursor** — current position

### Results Screen

After completing a test:

```
             ╔═══ RESULTS ═══╗
             ║               ║
             ║  TEST COMPLETE!║
             ║               ║
             ║  WPM: 75.2    ║
             ║  Accuracy: 99.1%║
             ║  Time: 45.32s ║
             ║               ║
             ║ Press SPACE to║
             ║ restart or ` to quit
             ║               ║
             ╚═══════════════╝
```

### History View

Browse your test history:

```
[Mode]  [WPM]   [Accuracy]  [Time]      [Date]
MEDIUM  72.3    98.5%       45.32s      2026-01-25 09:15
SHORT   85.1    99.2%       22.10s      2026-01-24 20:33
LONG    68.9    97.1%       120.45s     2026-01-23 15:22
...
```

Use `↑`/`↓` to navigate, `Esc` to return to typing.

### Statistics View

Performance analytics:

```
╔════════ STATS ════════╗
║ Total Tests:    47    ║
║ Avg WPM:        71.2  ║
║ Best WPM:       89.4  ║
║ Consistency:    94.3% ║
║ Best Accuracy:  99.8% ║
║ Total Time:     52m   ║
╚═══════════════════════╝
```

## Themes

TUItype includes 6 color schemes:

| Theme | Style |
|-------|-------|
| **Dark** | Clean dark blue/green (default) |
| **Light** | Light background with high contrast |
| **Nord** | Arctic, north-bluish palette |
| **Dracula** | Dark with vibrant accents |
| **Solarized** | Precision colors for readability |
| **Catppuccin Mocha** | Warm pastel dark theme |

Cycle through themes with `Ctrl+T`. Your choice is saved automatically.

## Data Storage

TUItype stores all data locally in your OS user data directory:

- **Linux/BSD**: `~/.local/share/TUItype/`
- **macOS**: `~/Library/Application Support/TUItype/`
- **Windows**: `%APPDATA%\TUItype\`

Files:
- `typing.db` — SQLite database with all test results
- `config.toml` — user preferences (theme, mode)

No data is ever sent to the internet. Everything stays on your machine.

## Metrics

### WPM (Words Per Minute)

Calculated as:
```
WPM = (characters_typed / 5) / (seconds_elapsed / 60)
```

Standard typing test definition: 1 word = 5 characters.

### Accuracy

```
Accuracy = (correct_characters / attempted_characters) × 100%
```

Includes all characters typed, counting mistakes.

### Consistency

Derived from WPM variance:
```
Consistency = 100% - (std_deviation / mean_wpm × 100%)
```

Higher consistency = more stable typing speed throughout the test.

### Raw WPM

WPM calculated as if every character were perfect (used for consistency calculation):
```
Raw WPM = (total_characters / 5) / (seconds_elapsed / 60)
```

## Building from Source

### Prerequisites

- Rust 1.70+ (install via [rustup](https://rustup.rs/))
- Cargo (comes with Rust)

### Build

```bash
git clone https://github.com/yourusername/TUItype
cd TUItype
cargo build --release
```

The binary is at `target/release/TUItype`.

### Development Build

For active development with faster compilation:

```bash
cargo run
```

### Run Tests

```bash
cargo test
```

## Project Structure

```
TUItype/
├── src/
│   ├── main.rs           # Terminal setup and main event loop
│   ├── app.rs            # Core typing app logic
│   ├── models.rs         # Data structures (TestResult, etc.)
│   ├── theme.rs          # Color themes
│   ├── quotes.rs         # Quote loading and selection
│   ├── storage/
│   │   ├── mod.rs
│   │   ├── db.rs         # SQLite database operations
│   │   ├── config.rs     # Config file management
│   │   └── history.rs    # Result history helpers
│   └── ui/
│       ├── mod.rs
│       ├── history.rs    # History view rendering
│       └── stats.rs      # Statistics view rendering
├── Cargo.toml            # Rust dependencies and metadata
└── README.md             # This file
```

## Dependencies

Core libraries:

- **ratatui** — Terminal UI framework
- **crossterm** — Cross-platform terminal abstraction
- **rusqlite** — SQLite database bindings
- **chrono** — Date and time handling
- **rand** — Random quote selection
- **serde** — Serialization/deserialization
- **clap** — Command-line argument parsing (future use)
- **toml** — Config file parsing

See `Cargo.toml` for all dependencies and versions.

## Contributing

Contributions welcome! Areas for improvement:

- Additional quote sources
- More themes
- Keyboard customization
- Multiplayer mode (future)
- Network leaderboards (future)

### Steps to contribute

1. Fork the repo
2. Create a feature branch: `git checkout -b feature/your-feature`
3. Commit your changes: `git commit -am 'Add feature'`
4. Push to the branch: `git push origin feature/your-feature`
5. Open a Pull Request

## Troubleshooting

### App won't start

**Error:** `No home dir found`

- **Cause**: `directories` crate can't find your home directory
- **Solution**: Ensure your system has a valid `$HOME` (Linux/macOS) or `%USERPROFILE%` (Windows)

### SQLite database errors

**Error:** `Error opening database`

- **Cause**: Permission issue or corrupted database file
- **Solution**: Delete `typing.db` from your data directory (safe; just clears history)

### Terminal colors look wrong

**Fix**: Try a different theme with `Ctrl+T`, or check your terminal's color support (requires 256-color support)

### Performance issues on very long quotes

**Note**: Scrolling viewport was added to prevent this. If issues persist, file a bug.

## Performance

- **Startup**: ~10–50ms on modern hardware
- **Typing latency**: <5ms input-to-display lag
- **History loading**: 50 tests load in ~50ms
- **Memory footprint**: ~15–20 MB at runtime

## License

MIT License — see LICENSE file for details.

## Changelog

### v0.1.0 (2026-01-25)

- Initial release
- 3 difficulty modes
- 6 color themes
- Test history and statistics
- Quote scrolling viewport
- SQLite persistence
- Persistent configuration

## Acknowledgments

- Quotes sourced from MonkeyType
- Inspired by [TypeRacer](https://play.typeracer.com/), [Monkeytype](https://monkeytype.com/)
- Built with [ratatui](https://github.com/ratatui-org/ratatui) community

## Contact

Questions or feedback? Open an issue on GitHub or reach out!

---

**Happy typing! 🚀**
