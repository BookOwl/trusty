#[derive(Clone, Copy, Debug, PartialEq)]
/// The Cursor holds the line and column of the users cursor.
pub struct Cursor {
    pub line: usize,
    pub column: usize,
}

impl Cursor {
    pub fn new(line: usize, column: usize) -> Cursor {
        Cursor {
            line: line,
            column: column,
        }
    }
    pub fn move_up(&mut self, lines: &[String]) {
        if self.line != 0 {
            self.line -= 1;
            let len = lines[self.line].len();
            if self.column > len {
                self.column = len;
            }
        }
    }
    pub fn move_down(&mut self, lines: &[String]) {
        if self.line + 1 != lines.len() {
            self.line += 1;
            let len = lines[self.line].len();
            if self.column > len {
                self.column = len;
            }
        }
    }
    pub fn move_left(&mut self, lines: &[String]) {
        // If we are at the top left corner...
        if self.column == 0 && self.line == 0 {
            // return without doing anything
            return;
        }
        // If we are at the begining of the line...
        if self.column == 0 {
            // Move to the end of the next line up
            self.line -= 1;
            self.column = lines[self.line].len();
        } else {
            self.column -= 1;
        }
    }
    pub fn move_right(&mut self, lines: &[String]) {
        let line_len = lines[self.line].len();
        // If we are at the bottom right corner...
        if self.column == line_len && self.line + 1 == lines.len() {
            // return without doing anything
            return;
        }
        // If we are at the end of the line...
        if self.column == line_len {
            // Move to the begining of the next line down
            self.line += 1;
            self.column = 0
        } else {
            self.column += 1;
        }
    }
}
