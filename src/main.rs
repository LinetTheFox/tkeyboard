mod generator;
mod input;
mod tui;

use std::io::{stdin, stdout};

use termion::input::TermRead;
use termion::raw::IntoRawMode;
use termion::screen::*;

fn main() {
    let mut alt_screen = AlternateScreen::from(stdout().into_raw_mode().unwrap());
    let mut attempt: input::Attempt;

    loop {
        tui::reset(&mut alt_screen);
        let text = generator::generate_text(0);
        attempt = input::Attempt::new_attempt(&text);
    
        tui::write_sample_text(text);
    
        input::handle_printable_input(stdin().keys(), &mut attempt, &mut alt_screen);

        tui::try_again_prompt(&mut alt_screen);
        let answer = input::handle_y_n_input(stdin().keys());
        
        if answer {
            continue;
        } else {
            break;
        }
    }

}
