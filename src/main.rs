use rwitter::controllers;
use std::io::{stdin, stdout, Write};
use termion::event::*;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use termion::screen::AlternateScreen;
use termion::{clear, cursor};

fn main() {
    controllers::timeline_controller::call();

    let stdin = stdin();
    let mut screen = AlternateScreen::from(stdout().into_raw_mode().unwrap());

    for c in stdin.keys() {
        match c.unwrap() {
            Key::Char('q') => break,
            Key::Char('r') => {
                controllers::timeline_controller::call();
            }
            Key::Char(_c) => {}
            _ => {}
        }
    }

    write!(screen, "{}{}", clear::All, cursor::Goto(1, 1)).unwrap();
    screen.flush().unwrap();
}
