use crate::error::{MorelError, Result};
use crate::view_state::ViewState;
use crossterm::{
    cursor,
    execute,
    style::{Color, ResetColor, SetBackgroundColor, SetForegroundColor},
    terminal::{self, ClearType, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::io::{self, Write};

pub struct Terminal {
    _stdout: io::Stdout,
}

impl Terminal {
    pub fn new() -> Result<Self> {
        let mut stdout = io::stdout();

        // Enter alternate screen to preserve user's terminal content
        execute!(stdout, EnterAlternateScreen)
            .map_err(|e| MorelError::Terminal(e.to_string()))?;

        // Enable raw mode
        terminal::enable_raw_mode()
            .map_err(|e| MorelError::Terminal(e.to_string()))?;

        Ok(Self { _stdout: stdout })
    }

    pub fn get_size() -> Result<(u16, u16)> {
        terminal::size().map_err(|e| MorelError::Terminal(e.to_string()))
    }

    pub fn render(
        &mut self,
        lines: &[String],
        view: &ViewState,
        filename: &str,
        status_message: Option<&str>,
    ) -> Result<()> {
        let mut stdout = io::stdout();

        // Clear screen
        execute!(stdout, terminal::Clear(ClearType::All))
            .map_err(|e| MorelError::Terminal(e.to_string()))?;

        // Render visible lines
        let term_width = view.terminal_width() as usize;
        for (row, line) in lines.iter().enumerate() {
            // Move cursor to the beginning of the row
            execute!(stdout, cursor::MoveTo(0, row as u16))
                .map_err(|e| MorelError::Terminal(e.to_string()))?;

            // Truncate or pad the line to fit terminal width
            let display_line = if line.len() > term_width {
                &line[..term_width]
            } else {
                line
            };

            write!(stdout, "{}", display_line)
                .map_err(|e| MorelError::Terminal(e.to_string()))?;
        }

        // Render status line at the bottom
        self.render_status_line(view, filename, status_message)?;

        stdout.flush()
            .map_err(|e| MorelError::Terminal(e.to_string()))?;

        Ok(())
    }

    fn render_status_line(
        &mut self,
        view: &ViewState,
        filename: &str,
        status_message: Option<&str>,
    ) -> Result<()> {
        let mut stdout = io::stdout();
        let (width, height) = Self::get_size()?;
        let status_row = height - 1;

        // Move to status line
        execute!(stdout, cursor::MoveTo(0, status_row))
            .map_err(|e| MorelError::Terminal(e.to_string()))?;

        // Set status line colors (inverted)
        execute!(
            stdout,
            SetBackgroundColor(Color::White),
            SetForegroundColor(Color::Black)
        )
        .map_err(|e| MorelError::Terminal(e.to_string()))?;

        // Build status message
        let status = if let Some(msg) = status_message {
            format!("{:<width$}", msg, width = width as usize)
        } else {
            let percentage = view.get_percentage();
            let current_line = view.top_line() + 1;
            let total_lines = view.get_visible_range().0 + view.get_visible_range().1;

            let status_text = format!(
                "--More-- ({}%) line {}/{}  {}",
                percentage,
                current_line,
                total_lines.max(current_line),
                filename
            );

            // Pad to full width
            format!("{:<width$}", status_text, width = width as usize)
        };

        // Print status line
        write!(stdout, "{}", status)
            .map_err(|e| MorelError::Terminal(e.to_string()))?;

        // Reset colors
        execute!(stdout, ResetColor)
            .map_err(|e| MorelError::Terminal(e.to_string()))?;

        Ok(())
    }

    pub fn render_help(&mut self) -> Result<()> {
        let mut stdout = io::stdout();
        let (_width, height) = Self::get_size()?;

        // Clear screen
        execute!(stdout, terminal::Clear(ClearType::All))
            .map_err(|e| MorelError::Terminal(e.to_string()))?;

        let help_text = vec![
            "",
            "  Morel - Live File Pager",
            "  =======================",
            "",
            "  Navigation:",
            "    Space         Scroll down one page",
            "    Enter / Down  Scroll down one line",
            "    b / Up        Scroll up one page/line",
            "    k             Scroll up one line",
            "",
            "  Jumping:",
            "    g             Jump to start of file",
            "    G             Jump to end of file",
            "    [n]G          Jump to line n",
            "    [n]%          Jump to n% through file",
            "",
            "  Other:",
            "    r             Force refresh",
            "    h / ?         Show this help",
            "    q / Esc       Quit",
            "",
            "  Live Update:",
            "    File changes are automatically detected and displayed.",
            "",
            "",
            "  Press any key to continue...",
        ];

        let start_row = (height / 2).saturating_sub(help_text.len() as u16 / 2);

        for (i, line) in help_text.iter().enumerate() {
            let row = start_row + i as u16;
            if row < height - 1 {
                execute!(stdout, cursor::MoveTo(0, row))
                    .map_err(|e| MorelError::Terminal(e.to_string()))?;
                write!(stdout, "{}", line)
                    .map_err(|e| MorelError::Terminal(e.to_string()))?;
            }
        }

        stdout.flush()
            .map_err(|e| MorelError::Terminal(e.to_string()))?;

        Ok(())
    }
}

impl Drop for Terminal {
    fn drop(&mut self) {
        // Always cleanup, even on panic
        let mut stdout = io::stdout();
        let _ = execute!(stdout, LeaveAlternateScreen);
        let _ = terminal::disable_raw_mode();
    }
}
