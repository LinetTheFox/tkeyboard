use std::io::{stdout, Stdin, Stdout, Write};

use termion::color;
use termion::cursor::DetectCursorPos;
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

    pub fn get_text(self) -> String {
        self.text
    }

    pub fn is_correct_prefix(&self) -> bool {
        self.text.starts_with(&self.input_text)
    }

    pub fn handle_key(&mut self, c: char) {
        if c == '\x08' {
            self.input_text.pop();
        } else {
            // Any other printable character from
            // c.is_alphanumeric()
            self.input_text.push(c);
        }
    }
}

pub fn handle_printable_input(keys: Keys<Stdin>, attempt: &mut Attempt, screen: &mut RawScreen) {
    for c in keys {
        match c.unwrap() {
            // To be able to actually stop the program by the normal SIGINT (Ctrl+C)
            Key::Ctrl('c') => break,
            Key::Backspace => {
                write!(*screen, "\x08").unwrap();
                // Backspace won't get rid of the symbol, so we need to manually
                // clear everything after the cursor
                write!(*screen, "{}", termion::clear::AfterCursor,).unwrap();
                (*attempt).handle_key('\x08');
            }
            Key::Char(' ') => {
                write!(*screen, " ").unwrap();
                (*attempt).handle_key(' ');
            }
            Key::Char(c) => {
                if c.is_alphanumeric() {
                    write!(*screen, "{}", c).unwrap();
                }
                (*attempt).handle_key(c);
            }
            _ => {}
        }
        let is_correct = attempt.is_correct_prefix();
        let cursor_pos = stdout().cursor_pos().unwrap();
        if is_correct {
            // Because color::{White, Red} are different
            // structs, so cannot put them into write in
            // the if block
            write!(
                *screen,
                "{}{}{}{}",
                termion::clear::CurrentLine,
                termion::cursor::Goto(1, cursor_pos.1),
                color::Fg(color::Reset),
                (*attempt).input_text,
            )
            .unwrap();
        } else {
            write!(
                *screen,
                "{}{}{}{}",
                termion::clear::CurrentLine,
                termion::cursor::Goto(1, cursor_pos.1),
                color::Fg(color::Red),
                (*attempt).input_text,
            )
            .unwrap();
        }

        stdout().flush().unwrap();
        if *attempt.input_text == *attempt.text {
            break;
        }
    }
}

pub fn handle_y_n_input(keys: Keys<Stdin>) -> bool {
    for c in keys {
        match c.unwrap() {
            Key::Char('y') => {
                return true;
            }
            Key::Char('n') => {
                return false;
            }
            Key::Ctrl('c') => {
                // Interpret SIGINT as "no"
                return false;
            }
            _ => {}
        }
    }
    unreachable!("Supposed to loop forever until getting 'y', 'n' or 'SIGINT'");
}
