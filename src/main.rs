mod generator;
mod input;
mod tui;

use std::{
    io::{stdin, stdout},
    time::Instant,
};

use termion::input::TermRead;
use termion::raw::IntoRawMode;
use termion::screen::*;

fn main() {
    let mut alt_screen = AlternateScreen::from(stdout().into_raw_mode().unwrap());
    let mut attempt: input::Attempt;
    let generator = generator::Generator::init("res/words_padded.txt");
    loop {
        tui::reset(&mut alt_screen);
        let text = generator.generate(3u64);
        attempt = input::Attempt::new_attempt(&text);
        tui::write_sample_text(text);

        let begin_time = Instant::now();
        input::handle_printable_input(stdin().keys(), &mut attempt, &mut alt_screen);
        let duration = begin_time.elapsed();

        tui::show_result(&mut alt_screen, duration.as_millis(), 5);
        tui::try_again_prompt(&mut alt_screen);
        let answer = input::handle_y_n_input(stdin().keys());
        if answer {
            continue;
        } else {
            break;
        }
    }
}
