use frontend::Frontend;
use backend::Backend;
use cursor::Cursor;
use std::io::{stdin, Write};
use termion::input::TermRead;
use termion::event::*;

pub struct Editor<'a> {
    frontend: &'a mut Frontend,
    backend: &'a mut Backend,
    pos: Cursor,
}

impl<'a> Editor<'a> {
    pub fn new(frontend: &'a mut Frontend, backend: &'a mut Backend) -> Editor<'a> {
        Editor {
            frontend: frontend,
            backend: backend,
            pos: Cursor::new(0, 0),
        }
    }
    pub fn start(&mut self) {
        // TODO: Implement the main loop.
        // This requires Frontend and Backend to be done.
        self.draw();
        for event in stdin().events() {
            self.draw();
            match event {
                Ok(Event::Key(Key::Char('q'))) => break,
                Err(_) => break,
                _ => {},
            }
        }
    }
    fn draw(&mut self) {
        self.frontend.clear_screen();
        self.frontend.draw_line_numbers(&self.pos);
        self.frontend.flush();
    }
}
