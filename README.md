# morel 🍄

<p align="center">
  <em>A live file pager that automatically updates when files change</em>
</p>

Morel is a terminal-based file viewer written in Rust, inspired by `more` and `less`, but with a unique twist: **it automatically refreshes when the file changes**. Perfect for monitoring log files, watching build outputs, or viewing any file that's being actively modified.

## Why Morel?

Traditional pagers like `more` and `less` show a static snapshot of a file. If the file changes, you have to manually reload it. Morel continuously monitors the file and updates the display in real-time, making it ideal for:

- 📝 **Log monitoring** - Watch application logs as they're written
- 🔨 **Build watching** - Monitor build outputs and test results
- 📊 **Data streams** - View files being written by data pipelines
- 🐛 **Debugging** - Monitor debug output files in real-time

## Features

✨ **Live Updates** - Automatically refreshes when the file is modified, appended, or truncated
⬆️⬇️ **Full Navigation** - Scroll forward and backward through files
🎯 **Jump Commands** - Jump to specific lines, percentages, start, or end
🖥️ **Cross-Platform** - Works on macOS, Linux, and Windows
🛡️ **Safe Exit** - Properly restores terminal state even on crashes
⚡ **Efficient** - Debounced file watching prevents excessive reloads

## Installation

### From Source

```bash
git clone https://github.com/yourusername/morel.git
cd morel
cargo build --release
```

The binary will be in `target/release/morel`.

### From Cargo

```bash
cargo install morel
```

## Quick Start

```bash
# View a file
morel myfile.txt

# View a log file that's being actively written
morel /var/log/myapp.log
```

Press `h` or `?` while viewing for help, `q` to quit.

## Usage

### Basic Usage

```bash
morel <filename>
```

### Keyboard Shortcuts

| Key | Action |
|-----|--------|
| **Navigation** ||
| `Space` | Scroll down one page |
| `Enter` / `↓` | Scroll down one line |
| `b` / `↑` | Scroll up one page |
| `k` | Scroll up one line |
| **Jumping** ||
| `g` | Jump to start of file |
| `G` | Jump to end of file |
| `[n]G` | Jump to line n (e.g., `42G` → line 42) |
| `[n]%` | Jump to n% through file (e.g., `50%` → middle) |
| **Other** ||
| `r` | Force refresh |
| `h` / `?` | Show help |
| `q` / `Esc` / `Ctrl+C` | Quit |

### Live Update Behavior

Morel automatically detects and displays different types of file changes:

- **File appended** → New content appears, your position is maintained
- **File truncated** → View adjusts to new file size
- **File modified** → Content updates automatically
- **File deleted** → Shows last known content with a warning

Status messages appear briefly in the status bar to indicate what changed.

## Example

Monitor a log file while another process writes to it:

```bash
# Terminal 1: Start morel
morel app.log

# Terminal 2: Write to the log
echo "New log entry: $(date)" >> app.log
```

The new line appears automatically in the morel viewer!

## How It Works

Morel uses an event-driven architecture:

1. **File Reader** - Loads file content into memory (line-based)
2. **File Watcher** - Monitors the file's directory using `notify` with 100ms debouncing
3. **Terminal Manager** - Handles raw mode, rendering, and alternate screen buffer
4. **View State** - Tracks scroll position and viewport
5. **Input Handler** - Processes keyboard commands
6. **Event Loop** - Coordinates all components in real-time

### Technical Stack

- **Terminal Control**: [`crossterm`](https://crates.io/crates/crossterm) for cross-platform terminal manipulation
- **File Watching**: [`notify`](https://crates.io/crates/notify) with [`notify-debouncer-full`](https://crates.io/crates/notify-debouncer-full) for efficient event handling
- **Error Handling**: [`thiserror`](https://crates.io/crates/thiserror) and [`anyhow`](https://crates.io/crates/anyhow)

### Limitations

- Files are loaded entirely into memory (works best for files < 100MB)
- Binary files are not supported
- Unicode and wide characters are supported but may render incorrectly depending on terminal

## Contributing

Contributions are welcome! Feel free to:

- 🐛 Report bugs
- 💡 Suggest features
- 🔧 Submit pull requests

Please open an issue first to discuss major changes.

## Development

```bash
# Clone the repo
git clone https://github.com/yourusername/morel.git
cd morel

# Build
cargo build

# Run tests
cargo test

# Run with a sample file
echo "Line 1
Line 2
Line 3" > sample.txt
cargo run -- sample.txt
```

## License

MIT License - see [LICENSE](LICENSE) for details

## Etymology

Named after the **morel mushroom** (*Morchella*), known for appearing and changing with the seasons - much like this pager watches for changes in files!
