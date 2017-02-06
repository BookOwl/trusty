use std::io;
use std::io::{Read, Write, Error ,ErrorKind};
use std::fs::File;
use cursor::Cursor;

/// The Backend is responsible opening files and editing text.
/// It does this by managing a Vec of Buffers that actually edit the text.
/// By making the Backend handle the Buffers, the rest of the editor doesn't have to
/// worry about using the right buffer.
#[derive(Debug)]
pub struct Backend {
    /// The Buffers being edited
    buffers: Vec<Buffer>,
    /// The index of the current Buffer
    current: usize,
}

impl Backend {
    /// Constructs and returns a new Backend from the command line arguments
    pub fn new() -> Backend {
        // TODO: Replace with real argument parsing. In editor.rs, maybe?
        let arg = ::std::env::args().nth(1);
        let buffers = if let Some(filename) = arg {
            vec![Buffer::from_file(filename).unwrap()]
        } else {
            vec![Buffer::new()]
        };
        Backend {
            buffers: buffers,
            current: 0,
        }
    }
    /// Returns the lines of text from the buffer that is being edited
    // TODO: Make this higher level so that it is easier to change
    // the buffer representation? Maybe as an iterator?
    pub fn current_lines(&self) -> &Vec<String> {
        &self.current_buffer().lines
    }
    /// Returns the number of lines in the current buffer.
    pub fn number_of_lines(&self) -> usize {
        self.current_buffer().lines.len()
    }
    /// Returns the length of the line in the current buffer
    pub fn length_of_line(&self, line: usize) -> usize {
        self.current_buffer().lines[line].len()
    }
    /// Inserts a newline at the position given by the Cursor and updates
    /// the Cursor to reflect the new position
    pub fn insert_newline(&mut self) {
        // Get the (x, y) location of the cursor.
        // This happens in a seperate block to keep the borrow checker happy
        let (x, y) = {
            let cursor = self.cursor();
            (cursor.line, cursor.column)
        };
        {
            // Inserts the new line at the right place.
            // This is in a seperate block to make sure that buf gets destroyed
            // before we mutate self.cursor.
            let line_len = self.length_of_line(x);
            let mut buf = self.current_buffer_mut();
            buf.split_line_into_two_at(x, y);
        };
        // Updates the cursor to the new position
        let mut cursor = self.cursor_mut();
        cursor.line += 1;
        cursor.column = 0;
    }
    /// Inserts a backspace at the position given by the Cursor and updates
    /// the Cursor to reflect the new position
    pub fn insert_backspace(&mut self) {
        // Get the (x, y) location of the cursor.
        // This happens in a seperate block to keep the borrow checker happy
        let (x, y) = {
            let cursor = self.cursor();
            (cursor.line, cursor.column)
        };
        if x == 0 && y == 0 {
            // We are at the top left corner and there is nothing to delete.
            return;
        }
        if y == 0 {
            // If we are at the begining of the line we just move the current
            // line to the end of the previous line.
            self.cursor_mut().column = self.current_buffer().lines[x - 1].len();
            self.current_buffer_mut().join_lines_at(y);
            self.cursor_mut().line -= 1;
        } else {
            // We remove the char at the column before the cursor because
            // when you use backspace you are trying to delete what
            // comes before.
            self.current_buffer_mut().delete_char_at(x, y - 1);
            self.cursor_mut().column -= 1;
        }
    }
    /// Inserts a character at the position given by the Cursor and updates
    /// the Cursor to reflect the new position
    // BUG: Doesn't work for most non-ascii utf8 text. :(
    pub fn insert_char(&mut self, c: char) {
        // Get the (x, y) location of the cursor.
        // This happens in a seperate block to keep the borrow checker happy
        let (x, y) = {
            let cursor = self.cursor();
            (cursor.line as usize, cursor.column as usize)
        };
        {
            // Update the buffer in a seperate block to
            // keep the borrow checker happy.
            self.current_buffer_mut().insert_char_at(c, x, y);
        }
        // Update the cursor.
        self.cursor_mut().column += 1;
    }
    /// Returns the current buffer
    // TODO: Does this need to be public?
    pub fn current_buffer(&self) -> &Buffer {
        &self.buffers[self.current]
    }
    /// Returns the current buffer as a mutable reference
    // TODO: Does this need to be public?
    pub fn current_buffer_mut(&mut self) -> &mut Buffer {
        &mut self.buffers[self.current]
    }
    /// Returns the filename of the current buffer as an Option<String>
    pub fn filename(&self) -> &Option<String> {
        &self.current_buffer().filename
    }
    /// Sets the filename of the current buffer
    pub fn set_filename(&mut self, name: Option<String>) {
        self.current_buffer_mut().filename = name
    }
    /// Saves the current buffer to a file
    pub fn save(&mut self) -> io::Result<()> {
        self.current_buffer_mut().save()
    }
    /// Switches to the next buffer.
    pub fn switch_to_next_buffer(&mut self) {
        self.current = (self.current + 1) % self.buffers.len();
    }
    /// Switches to the previous buffer.
    pub fn switch_to_previous_buffer(&mut self) {
        self.current = (self.current - 1) % self.buffers.len();
    }
    /// Opens a new, empty buffer
    pub fn new_empty_buffer(&mut self) {
        self.buffers.insert(self.current + 1, Buffer::new());
        self.switch_to_next_buffer();
    }
    /// Returns a reference to the current buffer's Cursor.
    pub fn cursor(&self) -> &Cursor {
        &self.current_buffer().cursor
    }
    /// Returns a mutable reference to the current buffer's Cursor.
    pub fn cursor_mut(&mut self) -> &mut Cursor {
        &mut self.current_buffer_mut().cursor
    }
    
    // TODO: Make these function not need to clone the contents
    // of the text editor.

    /// Moves the cursor up
    pub fn move_up(&mut self) {
        let lines = self.current_lines().clone();
        self.cursor_mut().move_up(&lines)
    }
    /// Moves the cursor down
    pub fn move_down(&mut self) {
        let lines = self.current_lines().clone();
        self.cursor_mut().move_down(&lines)
    }
    /// Moves the cursor left
    pub fn move_left(&mut self) {
        let lines = self.current_lines().clone();
        self.cursor_mut().move_left(&lines)
    }
    /// Moves the cursor right
    pub fn move_right(&mut self) {
        let lines = self.current_lines().clone();
        self.cursor_mut().move_right(&lines)
    }
}

/// A Buffer contains the text being edited and applies the edits to it.
/// It is also responsible for opening and saving files.
#[derive(Debug)]
pub struct Buffer {
    /// The filename that the Buffer gets saved to.
    pub filename: Option<String>,
    /// The lines of text.
    lines: Vec<String>,
    /// The saved state of the buffer. If dirty is true then there are
    /// unsaved modifications to the Buffer that haven't saved.
    pub dirty: bool,
    /// The cursor position in the buffer.
    cursor: Cursor,
}
impl Buffer {
    /// Constructs a new, empty buffer that doesn't have a filename to save to.
    pub fn new() -> Buffer {
        Buffer {
            filename: None,
            lines: vec![String::new()],
            dirty: false,
            cursor: Cursor::new(0, 0),
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
            dirty: false,
            cursor: Cursor::new(0, 0)
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
        self.dirty = true;
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
        self.dirty = true;
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
        self.dirty = true;
    }
    /// Moves the line at `line` into the line before it and removes it.
    pub fn join_lines_at(&mut self, line: usize) {
        assert!(line >= 1, "Tried to move first line to the -1 line!");
        let s = self.lines.remove(line);
        self.lines[line - 1].push_str(&s);
        self.dirty = true;
    }
    /// Deletes the character at `line`, `column`.
    pub fn delete_char_at(&mut self, line: usize, column: usize) {
        self.lines[line].remove(column);
        self.dirty = true;
    }
}
