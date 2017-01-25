use std::io;
use std::io::{Read, Write, Error ,ErrorKind};
use std::fs::File;

pub struct Backend {
    buffers: Vec<Buffer>,
    current: usize,
}

impl Backend {
    pub fn new() -> Backend {
        Backend {
            buffers: vec![Buffer::from_file(String::from("test.txt")).unwrap()],
            current: 0,
        }
    }
    pub fn current_lines(&self) -> &Vec<String> {
        &self.buffers[self.current].lines
    }
    pub fn number_of_lines(&self) -> usize {
        self.buffers[self.current].lines.len()
    }
    pub fn current_line_length(&self, line: usize) -> usize {
        self.buffers[self.current].lines[line].len()
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
    /// Adds a new line at the end of the text
    pub fn push_newline(&mut self) {
        self.lines.push(String::new());
    }
    /// Returns a line of text as a String
    pub fn get_line(&self, index: usize) -> &String {
        &self.lines[index]
    }
}
