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
}
