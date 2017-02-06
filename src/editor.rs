use frontend::Frontend;
use backend::Backend;
use cursor::Cursor;
use std::io::{stdin, Write};
use termion::input::TermRead;
use termion::event::*;

static SAVE_PROMPT: &'static str = "Enter the filename to save to (Ctrl-c to exit)";

/// The Editor struct is responsible recieving events
/// from the user and directing the frontend and backend.
pub struct Editor<'a> {
    frontend: &'a mut Frontend,
    backend: &'a mut Backend,
}

impl<'a> Editor<'a> {
    /// Constructs a new editor instance from the given Frontend and Backend.
    pub fn new(frontend: &'a mut Frontend, backend: &'a mut Backend) -> Editor<'a> {
        Editor {
            frontend: frontend,
            backend: backend,
        }
    }
    /// Starts the event loop.
    /// This function doesn't return until the program should end.
    pub fn start(&mut self) {
        // First, we need to render the editor before we start
        // the event loop so that the user can see the editor
        // before they do anything.
        self.draw();
        // This is the event loop. We go through each event
        // in stdin().event() (provided by termion) and run
        // the appropiate action in response.
        for event in stdin().events() {
            match event {
                Ok(ev) => match ev {
                    // Escape quits the program.
                    // TODO: Make it prompt to save the text before you leave?
                    Event::Key(Key::Esc) => break,
                    // You can move the cursor around with the arrow keys.
                    Event::Key(Key::Up) => self.backend.move_up(),
                    Event::Key(Key::Down) => self.backend.move_down(),
                    Event::Key(Key::Left) => self.backend.move_left(),
                    Event::Key(Key::Right) => self.backend.move_right(),
                    // Enter inserts a neline like you would expect.
                    Event::Key(Key::Char('\n')) => self.backend.insert_newline(),
                    // Backspace also works like you would expect.
                    Event::Key(Key::Backspace) => self.backend.insert_backspace(),
                    // Any other normal character just types that character.
                    Event::Key(Key::Char(c)) => self.backend.insert_char(c),
                    // Ctrl-s saves the current buffer.
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
                // If an error occured, panic!
                // TODO: Change this to return an Err result.
                Err(e) => panic!("Error: {}", e),
            }
            // Render the new state of the editor to the screen.
            self.draw();
        }
    }
    /// Renders the current state of the editor to the screen.
    /// It's pretty simple because it just passes the important info
    /// from the backend to the frontend.
    fn draw(&mut self) {
        self.frontend.clear_screen();
        self.frontend.draw(self.backend.cursor(),
                           self.backend.filename(),
                           self.backend.current_lines());
        self.frontend.move_cursor(self.backend.cursor());
        self.frontend.flush();
    }
}
