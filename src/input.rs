use crate::error::{MorelError, Result};
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyModifiers};
use std::time::Duration;

#[derive(Debug, PartialEq)]
pub enum Command {
    ScrollDownPage,
    ScrollDownLine,
    ScrollUpPage,
    ScrollUpLine,
    JumpToLine(usize),
    JumpToPercentage(u8),
    JumpToStart,
    JumpToEnd,
    Quit,
    Refresh,
    Help,
    None,
}

pub struct InputHandler {
    number_buffer: String,
}

impl InputHandler {
    pub fn new() -> Self {
        Self {
            number_buffer: String::new(),
        }
    }

    pub fn read_command(&mut self, timeout: Duration) -> Result<Command> {
        if !event::poll(timeout)
            .map_err(|e| MorelError::Terminal(e.to_string()))?
        {
            return Ok(Command::None);
        }

        let event = event::read()
            .map_err(|e| MorelError::Terminal(e.to_string()))?;

        match event {
            Event::Key(key_event) => Ok(self.handle_key(key_event)),
            Event::Resize(_, _) => {
                // Resize is handled in main loop
                Ok(Command::None)
            }
            _ => Ok(Command::None),
        }
    }

    fn handle_key(&mut self, key: KeyEvent) -> Command {
        // Handle Ctrl+C as quit
        if key.modifiers.contains(KeyModifiers::CONTROL) && key.code == KeyCode::Char('c') {
            return Command::Quit;
        }

        match key.code {
            // Quit commands
            KeyCode::Char('q') | KeyCode::Esc => Command::Quit,

            // Scroll down
            KeyCode::Char(' ') => {
                self.number_buffer.clear();
                Command::ScrollDownPage
            }
            KeyCode::Enter | KeyCode::Down | KeyCode::Char('j') => {
                self.number_buffer.clear();
                Command::ScrollDownLine
            }

            // Scroll up
            KeyCode::Char('b') | KeyCode::Up | KeyCode::Char('k') => {
                self.number_buffer.clear();
                Command::ScrollUpPage
            }

            // Jump commands
            KeyCode::Char('g') => {
                let cmd = if self.number_buffer.is_empty() {
                    Command::JumpToStart
                } else {
                    if let Ok(line) = self.number_buffer.parse::<usize>() {
                        Command::JumpToLine(line)
                    } else {
                        Command::None
                    }
                };
                self.number_buffer.clear();
                cmd
            }

            KeyCode::Char('G') => {
                let cmd = if self.number_buffer.is_empty() {
                    Command::JumpToEnd
                } else {
                    if let Ok(line) = self.number_buffer.parse::<usize>() {
                        Command::JumpToLine(line)
                    } else {
                        Command::None
                    }
                };
                self.number_buffer.clear();
                cmd
            }

            KeyCode::Char('%') => {
                let cmd = if let Ok(percent) = self.number_buffer.parse::<u8>() {
                    Command::JumpToPercentage(percent)
                } else {
                    Command::None
                };
                self.number_buffer.clear();
                cmd
            }

            // Number input for jump commands
            KeyCode::Char(c) if c.is_ascii_digit() => {
                self.number_buffer.push(c);
                Command::None
            }

            // Help
            KeyCode::Char('h') | KeyCode::Char('?') => {
                self.number_buffer.clear();
                Command::Help
            }

            // Refresh
            KeyCode::Char('r') => {
                self.number_buffer.clear();
                Command::Refresh
            }

            _ => {
                self.number_buffer.clear();
                Command::None
            }
        }
    }
}

impl Default for InputHandler {
    fn default() -> Self {
        Self::new()
    }
}
