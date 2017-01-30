use std::io;
use std::io::{Read, Write, Error ,ErrorKind};
use std::fs::File;
use cursor::Cursor;

/// The Backend is responsible opening files and editing text.
/// It does this by managing a Vec of Buffers that actually edit the text.
/// By making the Backend handle the Buffers, the rest of the editor doesn't have to
/// worry about using the right buffer.
pub struct Backend {
    /// The Buffers being edited
    buffers: Vec<Buffer>,
    /// The index of the current Buffer
    current: usize,
}

impl Backend {
    /// Constructs and returns a new Backend from the command line arguments
    pub fn new() -> Backend {
        Backend {
            buffers: vec![Buffer::from_file(
                // TODO: Replace with real argument parsing. In editor.rs, maybe?
                ::std::env::args().nth(1).unwrap_or(String::from("test.txt"))
            ).unwrap()],
            current: 0,
        }
    }
    /// Returns the lines of text from the buffer that is being edited
    // TODO: Make this higher level so that it is easier to change
    // the buffer representation? Maybe as an iterator?
    pub fn current_lines(&mut self) -> &Vec<String> {
        &self.current_buffer().lines
    }
    /// Returns the number of lines in the current buffer.
    pub fn number_of_lines(&mut self) -> usize {
        self.current_buffer().lines.len()
    }
    /// Returns the length of the line in the current buffer
    pub fn length_of_line(&mut self, line: usize) -> usize {
        self.current_buffer().lines[line].len()
    }
    /// Inserts a newline at the position given by the Cursor and updates
    /// the Cursor to reflect the new position
    pub fn insert_newline_at(&mut self, cursor: &mut Cursor) {
        let line_len = self.length_of_line(cursor.line);
        let mut buf = self.current_buffer();
        buf.split_line_into_two_at(cursor.line, cursor.column);
        cursor.line += 1;
        cursor.column = 0;

    }
    /// Inserts a backspace at the position given by the Cursor and updates
    /// the Cursor to reflect the new position
    pub fn insert_backspace_at(&mut self, cursor: &mut Cursor) {
        // TODO
        if cursor.column == 0 && cursor.line == 0 {
            // We are at the top left corner and there is nothing to delete.
            return;
        }
        let mut buf = self.current_buffer();
        if cursor.column == 0 {
            cursor.column = buf.lines[cursor.line - 1].len();
            buf.join_lines_at(cursor.line);
            cursor.line -= 1;
        } else {
            // We remove the char at the column before the cursor because
            // when you use backspace you are trying to delete what
            // comes before.
            buf.delete_char_at(cursor.line, cursor.column - 1);
            cursor.column -= 1;
        }
    }
    /// Inserts a character at the position given by the Cursor and updates
    /// the Cursor to reflect the new position
    pub fn insert_char_at(&mut self, c: char, cursor: &mut Cursor) {
        // BUG: Doesn't work for most non-ascii utf8 text. :(
        self.current_buffer().insert_char_at(c, cursor.line as usize, cursor.column as usize);
        cursor.column += 1;
    }
    /// Returns the current buffer
    // TODO: Does this need to be public?
    pub fn current_buffer(&mut self) -> &mut Buffer {
        &mut self.buffers[self.current]
    }
}

/// A Buffer contains the text being edited and applies the edits to it.
/// It is also responsible for opening and saving files.
pub struct Buffer {
    /// The filename that the Buffer gets saved to.
    pub filename: Option<String>,
    /// The lines of text.
    lines: Vec<String>,
    /// The saved state of the buffer. If dirty is true then there are
    /// unsaved modifications to the Buffer that haven't saved.
    pub dirty: bool,
}
impl Buffer {
    /// Constructs a new, empty buffer that doesn't have a filename to save to.
    pub fn new() -> Buffer {
        Buffer {
            filename: None,
            lines: vec![String::new()],
            dirty: false,
        }
    }
    /// Contructs a new buffer from the contents of a file.
    pub fn from_file(filename: String) -> io::Result<Buffer> {
        let mut file = File::open(&filename)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        let mut lines = Vec::new();
        for line in contents.lines() {
            lines.push(String::from(line))
        }
        Ok(Buffer {
            filename: Some(filename),
            lines: lines,
            dirty: false
        })
    }
    /// Saves the contents of the buffer to the file
    pub fn save(&mut self) -> io::Result<()> {
        if let Some(ref filename) = self.filename {
            let mut file = File::create(filename)?;
            for line in &self.lines {
                writeln!(&mut file, "{}", line)?;
            }
        } else {
            return Err(Error::new(ErrorKind::Other, "No file to write to"));
        }
        self.dirty = false;
        Ok(())
    }
    /// Inserts a newline
    pub fn insert_newline_at(&mut self, index: usize, content: String) {
        if index == self.lines.len() {
            self.push_newline(content);
        } else {
            self.lines.insert(index, content);
        }
    }
    /// Adds a new line at the end of the text
    pub fn push_newline(&mut self, content: String) {
        self.lines.push(content);
    }
    /// Returns a line of text as a String
    pub fn get_line(&self, index: usize) -> &String {
        &self.lines[index]
    }
    /// Splits the line at `line` into two lines at column.
    pub fn split_line_into_two_at(&mut self, line: usize, column: usize) {
        let (start, rest) = self.split_line_at(line, column);
        self.lines[line] = start;
        self.insert_newline_at(line + 1, rest);
    }
    /// Splits a line into two Strings
    pub fn split_line_at(&self, line: usize, column: usize) -> (String, String) {
        let (a, b) = self.lines[line].split_at(column);
        (String::from(a), String::from(b))
    }
    /// Inserts a char at a specific line, column
    pub fn insert_char_at(&mut self, c: char, line: usize, column: usize) {
        // TODO fix for unicode
        self.lines[line].insert(column, c);
    }
    /// Moves the line at `line` into the line before it and removes it.
    pub fn join_lines_at(&mut self, line: usize) {
        assert!(line >= 1, "Tried to move first line to the -1 line!");
        let s = self.lines.remove(line);
        self.lines[line - 1].push_str(&s);
    }
    /// Deletes the character at `line`, `column`.
    pub fn delete_char_at(&mut self, line: usize, column: usize) {
        self.lines[line].remove(column);
    }
}
