#[derive(Debug, Clone, Copy, Default)]
pub struct Cursor {
    pub line: usize,
    pub col: usize,
}

impl Cursor {
    pub fn new() -> Self {
        Self { line: 0, col: 0 }
    }

    pub fn move_up(&mut self) {
        if self.line > 0 {
            self.line -= 1;
        }
    }

    pub fn move_down(&mut self, max_line: usize) {
        if self.line + 1 < max_line {
            self.line += 1;
        }
    }

    pub fn move_left(&mut self) {
        if self.col > 0 {
            self.col -= 1;
        }
    }

    pub fn move_right(&mut self, max_col: usize) {
        if self.col < max_col {
            self.col += 1;
        }
    }
}
