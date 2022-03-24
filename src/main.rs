mod generator;
mod tui;

use std::io::{stdin, stdout};

use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use termion::screen::*;

fn main() {
    let mut alt_screen = AlternateScreen::from(stdout().into_raw_mode().unwrap());
    tui::reset(&mut alt_screen);

    let text = generator::generate_text(0);
    tui::write_sample_text(text);

    for c in stdin().keys() {
        match c.unwrap() {
            Key::Ctrl('c') => break,
            Key::Backspace => println!("<BS>"),
            _ => {}
        };
    }
}
