use std::io::{Stdout, Write};
use termion::raw::RawTerminal;
use termion::screen::AlternateScreen;
use termion::{clear, cursor};

pub fn call(screen: &mut AlternateScreen<RawTerminal<Stdout>>) {
    write!(
        screen,
        "{}{}{}{}",
        cursor::Goto(1, 1),
        clear::All,
        cursor::Goto(1, 1),
        cursor::Show,
    )
    .unwrap();
    screen.flush().unwrap();
}
