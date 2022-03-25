use std::io::{stdout, Stdin, Stdout, Write};

use termion::event::Key;
use termion::input::Keys;
use termion::raw::RawTerminal;
use termion::screen::AlternateScreen;

// Dunno yet how to normally set global typedef
type RawScreen = AlternateScreen<RawTerminal<Stdout>>;

pub struct Attempt {
    text: String,
    input_text: String,
}

impl Attempt {
    pub fn new_attempt(text: &String) -> Attempt {
        let length = text.len();
        Attempt {
            text: (*text).clone(),
            input_text: String::with_capacity(length),
        }
    }

    pub fn handle_key(self, key: Key) {
        if key == Key::Backspace {
        } else {
        }
    }
}

pub fn handle_input(keys: Keys<Stdin>, attempt: &Attempt, screen: &mut RawScreen) {
    for c in keys {
        match c.unwrap() {
            Key::Ctrl('c') => break,
            Key::Backspace => {
                print!("\x08");
                // Backspace won't get rid of the symbol, so we need to manually
                // clear everything after the cursor
                write!(*screen, "{}", termion::clear::AfterCursor,).unwrap();
            }
            Key::Char(c) => {
                if c.is_alphanumeric() {
                    print!("{}", c);
                }
            }
            _ => {}
        }
        stdout().flush().unwrap();
    }
}
