use std::io::{Write, Stdout};

use termion::raw::RawTerminal;
use termion::screen::AlternateScreen;

pub fn init(screen: &mut AlternateScreen<RawTerminal<Stdout>>) {
    write!(
        *screen,
        "{}{}",
        termion::clear::All,
        termion::cursor::Goto(1, 1)
    )
    .unwrap();
    
    screen.flush().unwrap();
}
