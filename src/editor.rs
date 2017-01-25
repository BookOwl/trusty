use frontend::Frontend;
use backend::Backend;
use cursor::Cursor;
use std::io::{stdin, Write};
use termion::input::TermRead;
use termion::event::*;

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
                Ok(Event::Key(Key::Esc)) => break,
                Ok(ev) => match ev {
                    Event::Key(Key::Up) => self.cursor.move_up(self.backend.current_lines()),
                    Event::Key(Key::Down) => self.cursor.move_down(self.backend.current_lines()),
                    Event::Key(Key::Left) => self.cursor.move_left(self.backend.current_lines()),
                    Event::Key(Key::Right) => self.cursor.move_right(self.backend.current_lines()),
                    _ => {},
                },
                Err(e) => panic!("Error: {}", e),
            }
            self.draw();
        }
    }
    fn draw(&mut self) {
        self.frontend.clear_screen();
        self.frontend.draw_lines(&self.cursor, self.backend.current_lines());
        self.frontend.move_cursor(&self.cursor);
        self.frontend.flush();
    }
}
