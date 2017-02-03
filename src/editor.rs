use frontend::Frontend;
use backend::Backend;
use cursor::Cursor;
use std::io::{stdin, Write};
use termion::input::TermRead;
use termion::event::*;

static SAVE_PROMPT: &'static str = "Enter the filename to save to (Ctrl-c to exit)";

pub struct Editor<'a> {
    frontend: &'a mut Frontend,
    backend: &'a mut Backend,
    cursor: Cursor,
}

impl<'a> Editor<'a> {
    pub fn new(frontend: &'a mut Frontend, backend: &'a mut Backend) -> Editor<'a> {
        Editor {
            frontend: frontend,
            backend: backend,
            cursor: Cursor::new(0, 0),
        }
    }
    pub fn start(&mut self) {
        // TODO: Implement the main loop.
        // This requires Frontend and Backend to be done.
        self.draw();
        for event in stdin().events() {
            match event {
                Ok(ev) => match ev {
                    Event::Key(Key::Esc) => break,
                    Event::Key(Key::Up) => self.cursor.move_up(self.backend.current_lines()),
                    Event::Key(Key::Down) => self.cursor.move_down(self.backend.current_lines()),
                    Event::Key(Key::Left) => self.cursor.move_left(self.backend.current_lines()),
                    Event::Key(Key::Right) => self.cursor.move_right(self.backend.current_lines()),
                    Event::Key(Key::Char('\n')) => self.backend.insert_newline_at(&mut self.cursor),
                    Event::Key(Key::Backspace) => self.backend.insert_backspace_at(&mut self.cursor),
                    Event::Key(Key::Char(c)) => self.backend.insert_char_at(c, &mut self.cursor),
                    Event::Key(Key::Ctrl('s')) => {
                        if let &Some(_) = self.backend.filename() {
                            self.backend.save();
                        } else {
                            if let Some(name) = self.frontend.prompt_for_text(SAVE_PROMPT) {
                                self.backend.set_filename(Some(name));
                                self.backend.save().unwrap();
                            }
                        }
                    },
                    _ => {},
                },
                Err(e) => panic!("Error: {}", e),
            }
            self.draw();
        }
    }
    fn draw(&mut self) {
        self.frontend.clear_screen();
        self.frontend.draw(&self.cursor,
                           self.backend.filename(),
                           self.backend.current_lines());
        self.frontend.move_cursor(&self.cursor);
        self.frontend.flush();
    }
}
