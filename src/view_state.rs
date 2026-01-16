pub struct ViewState {
    top_line: usize,
    terminal_height: u16,
    terminal_width: u16,
    total_lines: usize,
    pub needs_redraw: bool,
}

impl ViewState {
    pub fn new(terminal_width: u16, terminal_height: u16, total_lines: usize) -> Self {
        Self {
            top_line: 0,
            terminal_height: terminal_height.saturating_sub(1), // Reserve 1 line for status
            terminal_width,
            total_lines,
            needs_redraw: true,
        }
    }

    pub fn scroll_down_page(&mut self) {
        let page_size = self.terminal_height as usize;
        self.top_line = (self.top_line + page_size).min(self.max_top_line());
        self.needs_redraw = true;
    }

    pub fn scroll_down_line(&mut self) {
        if self.top_line < self.max_top_line() {
            self.top_line += 1;
            self.needs_redraw = true;
        }
    }

    pub fn scroll_up_page(&mut self) {
        let page_size = self.terminal_height as usize;
        self.top_line = self.top_line.saturating_sub(page_size);
        self.needs_redraw = true;
    }

    pub fn scroll_up_line(&mut self) {
        if self.top_line > 0 {
            self.top_line -= 1;
            self.needs_redraw = true;
        }
    }

    pub fn jump_to_line(&mut self, line: usize) {
        self.top_line = line.saturating_sub(1).min(self.max_top_line());
        self.needs_redraw = true;
    }

    pub fn jump_to_percentage(&mut self, percent: u8) {
        let percent = percent.min(100) as usize;
        let target_line = (self.total_lines * percent) / 100;
        self.top_line = target_line.min(self.max_top_line());
        self.needs_redraw = true;
    }

    pub fn jump_to_start(&mut self) {
        self.top_line = 0;
        self.needs_redraw = true;
    }

    pub fn jump_to_end(&mut self) {
        self.top_line = self.max_top_line();
        self.needs_redraw = true;
    }

    pub fn update_total_lines(&mut self, total: usize) {
        self.total_lines = total;
        // Adjust top_line if it's now out of bounds
        if self.top_line > self.max_top_line() {
            self.top_line = self.max_top_line();
        }
        self.needs_redraw = true;
    }

    pub fn update_dimensions(&mut self, width: u16, height: u16) {
        self.terminal_width = width;
        self.terminal_height = height.saturating_sub(1); // Reserve 1 line for status
        // Adjust top_line if necessary
        if self.top_line > self.max_top_line() {
            self.top_line = self.max_top_line();
        }
        self.needs_redraw = true;
    }

    pub fn get_visible_range(&self) -> (usize, usize) {
        let start = self.top_line;
        let count = self.terminal_height as usize;
        (start, count)
    }

    pub fn get_percentage(&self) -> u8 {
        if self.total_lines == 0 {
            return 100;
        }
        if self.is_at_end() {
            return 100;
        }
        ((self.top_line as f64 / self.total_lines as f64) * 100.0) as u8
    }

    pub fn is_at_end(&self) -> bool {
        self.top_line >= self.max_top_line() && self.max_top_line() > 0
    }

    pub fn top_line(&self) -> usize {
        self.top_line
    }

    pub fn terminal_width(&self) -> u16 {
        self.terminal_width
    }

    fn max_top_line(&self) -> usize {
        self.total_lines.saturating_sub(self.terminal_height as usize)
    }
}
