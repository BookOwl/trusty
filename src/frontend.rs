use std::io::{Read, Write, Stdin, stdin, Stdout, stdout};
use std::ops::Drop;
use termion;
use termion::{clear, color};
use termion::raw::IntoRawMode;
use left_pad::leftpad;
use cursor::Cursor;

// The Frontend is responsible for rendering the state of the editor
// to the screen and interacting with the user.
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
    /// Draws the state of the editor to the screen.
    pub fn draw(&mut self, cursor: &Cursor, filename: &Option<String>, lines: &[String]) {
        let (width, height) = self.terminal_size();
        let num_lines = lines.len();
        // The index of the first line of text that is rendered.
        let start = if cursor.line > height { cursor.line - height } else { 0 };
        // The filename of the current buffer or a no filename message.
        let name = filename.clone().unwrap_or_else(|| String::from("**no filename**"));
        let padding = (width - name.len()) / 2;
        let need_extra = padding*2+name.len() != width;
        self.goto_term(0, 0);
        // Draw the title bar.
        write!(&mut self.stdout, "{}{}{}{}{}{}{}{}",
               color::Bg(color::White),
               color::Fg(color::Black),
               leftpad(" ", padding),
               name,
               leftpad(" ", padding),
               if need_extra { " " } else { "" },
               color::Fg(color::Reset),
               color::Bg(color::Reset),
        ).unwrap();
        // Draw the lines of text.
        for (y, line_number) in (start..start + height - 1).enumerate() {
            self.goto_term(0, (y + 1) as u16);
            if line_number < num_lines {
                // Draw the line of text
                write!(self.stdout, "{}{}{} {}",
                       color::Fg(color::Cyan),
                       leftpad(format!("{}", line_number + 1), 3),
                       color::Fg(color::Reset),
                       lines[line_number],
                ).unwrap();
            } else {
                // Draw a ~ to show that there is no line.
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
        let (_, height) = self.terminal_size();
        let x = (cursor.column + 4) as u16;
        let y = if cursor.line > height {
            cursor.line as u16
        } else {
            (cursor.line + 1) as u16
        };
        self.goto_term(x, y)
    }
    /// Returns the size of the terminal as (width, height)
    pub fn terminal_size(&self) -> (usize, usize) {
        let (w, h) = termion::terminal_size().unwrap();
        (w as usize, h as usize)
    }
    /// Prompts for a line of text
    pub fn prompt_for_text(&mut self, prompt: &str) -> Option<String> {
        let (width, height) = termion::terminal_size().unwrap();
        self.goto_term(0, height - 1);
        // Draw the background.
        write!(&mut self.stdout, "{}{}{}{}",
               termion::clear::CurrentLine,
               termion::color::Bg(color::White),
               termion::color::Fg(color::Black),
               leftpad("", width as usize)).unwrap();
        self.goto_term(0, height - 1);
        // Draw the prompt.
        write!(&mut self.stdout, "{}: ", prompt).unwrap();
        // Show it.
        self.flush();
        // Get the input from the user,
        let input = self.read_line();
        // Reset the forground and background.
        write!(self.stdout, "{}{}", color::Fg(color::Reset), color::Bg(color::Reset)).unwrap();
        input
    }
    /// Prompts for a yes/no response from the user
    pub fn prompt_for_bool(&mut self, prompt: &str) -> bool {
        let response = self.prompt_for_text(&format!("{} (y/n)", prompt));
        if let Some(r) = response {
            !r.is_empty() && r.chars().nth(0).unwrap() == 'y'
        } else {
            false
        }
    }
    /// Reads a line of text from the user.
    /// TODO: Fix for Unicode. I think that the actual user input is handled
    /// correctly, but echoing the typed characters may not be.
    fn read_line(&mut self) -> Option<String> {
        // Start with a buffer of size 40 so that small inputs don't require
        // reallocating the buffer.
        let mut buf = Vec::with_capacity(30);
        loop {
            // Get one byte of input
            let mut b = [0; 1];
            self.stdin.read_exact(&mut b[..]).unwrap();
            match b[0] {
                0 | 3 | 4 => return None,
                // 0x7f is backspace
                0x7f if !buf.is_empty() => {
                    // Delete the last character typed
                    buf.pop();
                    // Clear the last character from the screen
                    write!(&mut self.stdout, "{}{}",
                           termion::cursor::Left(1),
                           termion::clear::UntilNewline).unwrap();
                    self.flush();
                },
                0x7f => {},
                // Newline or CR ends the input
                b'\n' | b'\r' => break,
                c => {
                    // Add the typed character to the input
                    buf.push(c);
                    // Draw it to the screen
                    write!(&mut self.stdout, "{}", char::from(c)).unwrap();
                    self.flush();
                },
            };
        }
        // Convert the buffer to a String.
        Some(String::from_utf8(buf).unwrap())
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
