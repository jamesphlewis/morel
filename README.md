# morel

A live file pager written in Rust - like `more`, but it automatically updates when the file changes!

## Features

- **Live Updates**: Automatically refreshes the display when the file is modified
- **Full Navigation**: Scroll forward and backward through files
- **Jump Commands**: Jump to specific lines or percentages
- **Cross-Platform**: Works on macOS, Linux, and Windows
- **Clean Exit**: Properly restores terminal state even on crashes

## Installation

```bash
cargo build --release
```

The binary will be available at `target/release/morel`.

## Usage

```bash
morel <filename>
```

### Keyboard Shortcuts

**Navigation:**
- `Space` - Scroll down one page
- `Enter` / `Down` - Scroll down one line
- `b` / `Up` - Scroll up one page/line
- `k` - Scroll up one line

**Jumping:**
- `g` - Jump to start of file
- `G` - Jump to end of file
- `[n]G` - Jump to line n (e.g., `42G` jumps to line 42)
- `[n]%` - Jump to n% through file (e.g., `50%` jumps to middle)

**Other:**
- `r` - Force refresh
- `h` / `?` - Show help
- `q` / `Esc` - Quit

## Live Update Feature

The key feature of morel is its ability to automatically detect file changes and update the display in real-time:

- File appended: New content is shown, your position is maintained
- File truncated: View adjusts to new file size
- File modified: Content updates automatically
- File deleted: Shows last known content with a warning

This makes morel perfect for monitoring log files, watching build outputs, or any scenario where you need to view a file that's being actively modified.

## Example

Try it out with the test file:

```bash
# In one terminal
cargo run -- test.txt

# In another terminal, modify the file
echo "Line 34: New content!" >> test.txt
```

You'll see the update appear automatically in the morel viewer!

## Architecture

- **File Reader** - Efficiently reads and buffers file content
- **Terminal Manager** - Handles terminal raw mode and rendering
- **View State** - Tracks current scroll position and viewport
- **Input Handler** - Processes keyboard commands
- **File Watcher** - Monitors file changes using the `notify` crate
- **Event Loop** - Coordinates all components

## Technical Details

- Uses `crossterm` for cross-platform terminal control
- Uses `notify` with debouncing for efficient file watching
- Loads entire file into memory (suitable for files < 100MB)
- Uses alternate screen buffer to preserve terminal history
- Implements Drop trait for guaranteed cleanup

## License

MIT
