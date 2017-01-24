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
    pub fn new() -> Frontend {
        let in_ = stdin();
        let out = stdout().into_raw_mode().unwrap();
        Frontend {
            stdin: in_,
            stdout: out,
        }
    }
    pub fn clear_screen(&mut self) {
        write!(self.stdout, "{}", clear::All).unwrap();
    }
    pub fn draw_line_numbers(&mut self, pos: &Cursor) {
        let (width, height) = termion::terminal_size().unwrap();
        for (y, line_number) in (pos.line..pos.line+(height as usize)).enumerate() {
            self.goto(0, y as u16);
            write!(self.stdout, "{}{}{}",
                   color::Fg(color::Cyan),
                   leftpad(format!("{}", line_number + 1), 3),
                   color::Fg(color::Reset),
            ).unwrap();

        }
    }
    pub fn flush(&mut self) {
        self.stdout.flush().unwrap();
    }
    // Private methods
    fn goto(&mut self, x: u16, y: u16) {
        write!(self.stdout, "{}", termion::cursor::Goto(x+1, y+1)).unwrap();
    }
}

impl Drop for Frontend {
    /// Clean up the terminal after the we go out of scope.
    fn drop(&mut self) {
        self.clear_screen();
        self.goto(0, 0);
        self.flush();
    }
}
