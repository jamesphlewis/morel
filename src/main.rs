mod error;
mod file_reader;
mod input;
mod terminal;
mod view_state;
mod watcher;

use error::{MorelError, Result};
use file_reader::{FileReader, ReloadResult};
use input::{Command, InputHandler};
use terminal::Terminal;
use view_state::ViewState;
use watcher::{FileEvent, FileWatcher};

use std::env;
use std::path::PathBuf;
use std::time::Duration;

struct App {
    file_reader: FileReader,
    view_state: ViewState,
    terminal: Terminal,
    input_handler: InputHandler,
    watcher: FileWatcher,
    running: bool,
    status_message: Option<String>,
}

impl App {
    fn new(path: PathBuf) -> Result<Self> {
        let file_reader = FileReader::new(&path)?;
        let terminal = Terminal::new()?;
        let (width, height) = Terminal::get_size()?;
        let total_lines = file_reader.total_lines();
        let view_state = ViewState::new(width, height, total_lines);
        let input_handler = InputHandler::new();
        let watcher = FileWatcher::new(&path)?;

        Ok(Self {
            file_reader,
            view_state,
            terminal,
            input_handler,
            watcher,
            running: true,
            status_message: None,
        })
    }

    fn run(&mut self) -> Result<()> {
        while self.running {
            // Check for file changes
            self.handle_file_changes()?;

            // Check for terminal resize and render if needed
            if self.view_state.needs_redraw {
                self.render()?;
                self.view_state.needs_redraw = false;
            }

            // Read input with timeout for responsiveness
            let command = self
                .input_handler
                .read_command(Duration::from_millis(50))?;

            // Handle command
            self.handle_command(command)?;
        }

        Ok(())
    }

    fn handle_file_changes(&mut self) -> Result<()> {
        match self.watcher.check_for_changes()? {
            FileEvent::Modified | FileEvent::Created => {
                match self.file_reader.reload()? {
                    ReloadResult::NoChange => {}
                    ReloadResult::Appended => {
                        self.view_state
                            .update_total_lines(self.file_reader.total_lines());
                        self.status_message = Some("[File appended]".to_string());
                        self.view_state.needs_redraw = true;
                    }
                    ReloadResult::Truncated => {
                        self.view_state
                            .update_total_lines(self.file_reader.total_lines());
                        self.status_message = Some("[File truncated]".to_string());
                        self.view_state.needs_redraw = true;
                    }
                    ReloadResult::Modified => {
                        self.view_state
                            .update_total_lines(self.file_reader.total_lines());
                        self.status_message = Some("[File modified]".to_string());
                        self.view_state.needs_redraw = true;
                    }
                    ReloadResult::Deleted => {
                        self.status_message = Some("[File deleted - showing last content]".to_string());
                        self.view_state.needs_redraw = true;
                    }
                }
            }
            FileEvent::Deleted => {
                self.status_message = Some("[File deleted - showing last content]".to_string());
                self.view_state.needs_redraw = true;
            }
            FileEvent::NoChange => {
                // Don't clear status message too quickly - let it persist
            }
        }

        Ok(())
    }

    fn handle_command(&mut self, command: Command) -> Result<()> {
        match command {
            Command::Quit => {
                self.running = false;
            }
            Command::ScrollDownPage => {
                self.status_message = None;
                self.view_state.scroll_down_page();
            }
            Command::ScrollDownLine => {
                self.status_message = None;
                self.view_state.scroll_down_line();
            }
            Command::ScrollUpPage => {
                self.status_message = None;
                self.view_state.scroll_up_page();
            }
            Command::ScrollUpLine => {
                self.status_message = None;
                self.view_state.scroll_up_line();
            }
            Command::JumpToLine(line) => {
                self.status_message = None;
                self.view_state.jump_to_line(line);
            }
            Command::JumpToPercentage(percent) => {
                self.status_message = None;
                self.view_state.jump_to_percentage(percent);
            }
            Command::JumpToStart => {
                self.status_message = None;
                self.view_state.jump_to_start();
            }
            Command::JumpToEnd => {
                self.status_message = None;
                self.view_state.jump_to_end();
            }
            Command::Refresh => {
                match self.file_reader.reload()? {
                    ReloadResult::NoChange => {
                        self.status_message = Some("[No changes]".to_string());
                    }
                    _ => {
                        self.view_state
                            .update_total_lines(self.file_reader.total_lines());
                        self.status_message = Some("[Refreshed]".to_string());
                    }
                }
                self.view_state.needs_redraw = true;
            }
            Command::Help => {
                self.show_help()?;
            }
            Command::None => {}
        }

        Ok(())
    }

    fn show_help(&mut self) -> Result<()> {
        self.terminal.render_help()?;

        // Wait for any key press
        loop {
            if let Ok(cmd) = self
                .input_handler
                .read_command(Duration::from_millis(100))
            {
                if cmd != Command::None {
                    break;
                }
            }
        }

        self.view_state.needs_redraw = true;
        Ok(())
    }

    fn render(&mut self) -> Result<()> {
        let (start, count) = self.view_state.get_visible_range();
        let lines = self.file_reader.get_lines(start, count);
        let filename = self
            .file_reader
            .path()
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("unknown");

        self.terminal.render(
            lines,
            &self.view_state,
            filename,
            self.status_message.as_deref(),
        )?;

        Ok(())
    }
}

fn main() {
    if let Err(e) = run() {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}

fn run() -> Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: morel <filename>");
        eprintln!();
        eprintln!("A live file pager that automatically updates when the file changes.");
        eprintln!();
        eprintln!("Press 'h' or '?' while viewing to see keyboard shortcuts.");
        std::process::exit(1);
    }

    let path = PathBuf::from(&args[1]);

    if !path.exists() {
        return Err(MorelError::FileNotFound(path.display().to_string()));
    }

    let mut app = App::new(path)?;
    app.run()?;

    Ok(())
}
