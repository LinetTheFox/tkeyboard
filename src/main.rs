mod generator;

use std::io::{stdin, stdout, Write};

use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use termion::screen::*;

fn main() {
    let mut alt_screen = AlternateScreen::from(stdout().into_raw_mode().unwrap());

    write!(
        alt_screen,
        "{}{}Hello, World!",
        termion::clear::All,
        termion::cursor::Goto(1, 1)
    )
    .unwrap();

    alt_screen.flush().unwrap();

    for c in stdin().keys() {
        write!(
            stdout(),
            "{}{}",
            termion::cursor::Goto(1, 1),
            termion::clear::CurrentLine
        )
        .unwrap();

        match c.unwrap() {
            Key::Ctrl('c') => break,
            Key::Backspace => println!("BS"),
            _ => {}
        };
    }
}
