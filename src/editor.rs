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
}

impl<'a> Editor<'a> {
    pub fn new(frontend: &'a mut Frontend, backend: &'a mut Backend) -> Editor<'a> {
        Editor {
            frontend: frontend,
            backend: backend,
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
                    Event::Key(Key::Up) => self.backend.move_up(),
                    Event::Key(Key::Down) => self.backend.move_down(),
                    Event::Key(Key::Left) => self.backend.move_left(),
                    Event::Key(Key::Right) => self.backend.move_right(),
                    Event::Key(Key::Char('\n')) => self.backend.insert_newline(),
                    Event::Key(Key::Backspace) => self.backend.insert_backspace(),
                    Event::Key(Key::Char(c)) => self.backend.insert_char(c),
                    Event::Key(Key::Ctrl('s')) => {
                        if let &Some(_) = self.backend.filename() {
                            self.backend.save().unwrap();
                        } else {
                            if let Some(name) = self.frontend.prompt_for_text(SAVE_PROMPT) {
                                self.backend.set_filename(Some(name));
                                self.backend.save().unwrap();
                            }
                        }
                    },
                    //Event::Key(Key::Ctrl('n')) => self.backend.new_empty_buffer(),
                    _ => {},
                },
                Err(e) => panic!("Error: {}", e),
            }
            self.draw();
        }
    }
    fn draw(&mut self) {
        self.frontend.clear_screen();
        self.frontend.draw(self.backend.cursor(),
                           self.backend.filename(),
                           self.backend.current_lines());
        self.frontend.move_cursor(self.backend.cursor());
        self.frontend.flush();
    }
}
