use std::io;
use std::io::{Read, Write, Stdin, stdin, Stdout, stdout};
use std::ops::Drop;
use termion;
use termion::{clear, color, event};
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use left_pad::leftpad;
use cursor::Cursor;

pub struct Frontend {
    stdin: Stdin,
    stdout: termion::raw::RawTerminal<Stdout>,
}

impl Frontend {
    /// Creates a new Frontend
    pub fn new() -> Frontend {
        let in_ = stdin();
        let out = stdout().into_raw_mode().unwrap();
        Frontend {
            stdin: in_,
            stdout: out,
        }
    }
    /// Clears the screen
    pub fn clear_screen(&mut self) {
        write!(self.stdout, "{}", clear::All).unwrap();
    }
    pub fn draw_lines(&mut self, cursor: &Cursor, lines: &[String]) {
        let (_, height) = termion::terminal_size().unwrap();
        let num_lines = lines.len();
        let start = if cursor.line > height as usize { cursor.line } else { 0 };
        for (y, line_number) in (start..start + height as usize).enumerate() {
            self.goto_term(0, y as u16);
            if line_number < num_lines {
                write!(self.stdout, "{}{}{} {}",
                       color::Fg(color::Cyan),
                       leftpad(format!("{}", line_number + 1), 3),
                       color::Fg(color::Reset),
                       lines[line_number],
                ).unwrap();
            } else {
                write!(self.stdout, "{}  ~{}",
                       color::Fg(color::Cyan),
                       color::Fg(color::Reset),
                ).unwrap();
            }
        }
    }
    /// Flushes stdout to make the changes show
    pub fn flush(&mut self) {
        self.stdout.flush().unwrap();
    }
    /// Hides the cursor
    pub fn hide_cursor(&mut self) {
        write!(self.stdout, "{}", termion::cursor::Hide{}).unwrap();
    }
    /// Shows the cursor
    pub fn show_cursor(&mut self) {
        write!(self.stdout, "{}", termion::cursor::Show{}).unwrap();
    }
    /// Moves the cursor to x, y, which are both 0 based in terminal cordinates
    pub fn goto_term(&mut self, x: u16, y: u16) {
        write!(self.stdout, "{}", termion::cursor::Goto(x+1, y+1)).unwrap();
    }
    /// Moves the cursor to the position specified by the Cursor
    pub fn move_cursor(&mut self, cursor: &Cursor) {
        //let (width, height) = termion::terminal_size().unwrap();
        let x = (cursor.column + 4) as u16;
        let y = cursor.line as u16;
        self.goto_term(x, y)
    }
}

impl Drop for Frontend {
    /// Clean up the terminal after the we go out of scope.
    fn drop(&mut self) {
        self.clear_screen();
        self.goto_term(0, 0);
        self.show_cursor();
        self.flush();
    }
}
