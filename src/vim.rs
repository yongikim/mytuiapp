extern crate termion;

use std::io::{stdin, stdout, Write};
use termion::cursor;
use termion::event::*;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

pub fn vim() {
    let stdin = stdin();
    let mut stdout = stdout().into_raw_mode().unwrap();

    write!(
        stdout,
        "{}{}q to exit. Type stuff, use alt, and so on.",
        termion::clear::All,
        termion::cursor::Goto(1, 1)
    )
    .unwrap();

    stdout.flush().unwrap();

    for c in stdin.keys() {
        match c.unwrap() {
            Key::Char('q') => break,
            Key::Char('h') => {
                write!(stdout, "{}{}", cursor::Left(1), cursor::Show).unwrap();
            }
            Key::Char('j') => {
                write!(stdout, "{}{}", cursor::Down(1), cursor::Show).unwrap();
            }
            Key::Char('k') => {
                write!(stdout, "{}{}", cursor::Up(1), cursor::Show).unwrap();
            }
            Key::Char('l') => {
                write!(stdout, "{}{}", cursor::Right(1), cursor::Show).unwrap();
            }
            Key::Char(c) => {
                write!(stdout, "{}", c).unwrap();
            }
            _ => {}
        }

        stdout.flush().unwrap();
    }

    write!(
        stdout,
        "{}{}{}",
        termion::clear::All,
        cursor::Goto(1, 1),
        termion::cursor::Show
    )
    .unwrap();
}
