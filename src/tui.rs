use std::io::{Stdout, Write};

use termion::raw::RawTerminal;
use termion::screen::AlternateScreen;

type RawScreen = AlternateScreen<RawTerminal<Stdout>>;

/// Mostly used to reset the screen (move cursor to the beginning of
/// the first line), but can really work as initialization too
pub fn reset(screen: &mut RawScreen) {
    write!(
        *screen,
        "{}{}",
        termion::clear::All,
        termion::cursor::Goto(1, 1)
    )
    .unwrap();
    screen.flush().unwrap();
}

pub fn write_sample_text(text: String) {
    let hint = "Type the following text:";
    println!(
        "{}{}{}{}",
        hint,
        termion::cursor::Goto(1, 2),
        text,
        termion::cursor::Goto(1, 3)
    );
}

pub fn show_result(screen: &mut RawScreen, millis: u128, word_count: u16) {
    let seconds = millis / 1000;
    let rem_millis = millis % 1000;

    let type_speed = (word_count as f64) / (millis as f64 / 1000.0) * 60.0;
    write!(
        *screen,
        "{}Typed the text in {}.{}",
        termion::cursor::Goto(1, 8),
        seconds,
        rem_millis
    )
    .unwrap();
    write!(
        *screen,
        "{}Approx. typing speed: {:.2} words/minute",
        termion::cursor::Goto(1, 9),
        type_speed,
    ).unwrap();
    screen.flush().unwrap();
}

pub fn try_again_prompt(screen: &mut RawScreen) {
    let prompt = "Do you want to try again? (y/n): ";
    write!(*screen, "{}{} ", termion::cursor::Goto(1, 10), prompt,).unwrap();
    screen.flush().unwrap();
}
