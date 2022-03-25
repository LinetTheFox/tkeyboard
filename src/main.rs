mod generator;
mod input;
mod tui;

use std::io::{stdin, stdout};

use termion::input::TermRead;
use termion::raw::IntoRawMode;
use termion::screen::*;

fn main() {
    let mut alt_screen = AlternateScreen::from(stdout().into_raw_mode().unwrap());
    let attempt: input::Attempt;

    tui::reset(&mut alt_screen);
    let text = generator::generate_text(0);
    attempt = input::Attempt::new_attempt(&text);

    tui::write_sample_text(text);

    input::handle_input(stdin().keys(), &attempt, &mut alt_screen);
}
