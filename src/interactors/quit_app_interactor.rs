use std::io::{Stdout, Write};
use termion::raw::RawTerminal;
use termion::screen::AlternateScreen;
use termion::{clear, cursor};

pub fn call(screen: &mut AlternateScreen<RawTerminal<Stdout>>) {
    write!(screen, "{}{}", clear::All, cursor::Goto(1, 1)).unwrap();
    screen.flush().unwrap();
}
