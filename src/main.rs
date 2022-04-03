mod args;
mod generator;
mod input;
mod settings;
mod tui;

use crate::args::parse_args;
use crate::settings::Settings;

use std::{
    io::{stdin, stdout},
    time::Instant,
};
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use termion::screen::*;

fn main() {
    let mut settings = Settings::default();
    parse_args(&mut settings);
    let mut alt_screen = AlternateScreen::from(stdout().into_raw_mode().unwrap());
    let mut attempt: input::Attempt;
    let generator = generator::Generator::init("res/words_padded.txt");
    loop {
        tui::reset(&mut alt_screen);
        let text = generator.generate(settings.word_count as u64);
        attempt = input::Attempt::new_attempt(&text);
        tui::write_sample_text(text);

        let begin_time = Instant::now();
        input::handle_printable_input(stdin().keys(), &mut attempt, &mut alt_screen);
        let duration = begin_time.elapsed();

        tui::show_result(
            &mut alt_screen,
            duration.as_millis(),
            settings.word_count as u16,
            attempt.get_text().len(),
        );
        tui::try_again_prompt(&mut alt_screen);
        let answer = input::handle_y_n_input(stdin().keys());
        if answer {
            continue;
        } else {
            break;
        }
    }
}
