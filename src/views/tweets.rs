extern crate termion;

use std::io::{stdin, stdout, Write};
use termion::clear;
use termion::cursor::{self, DetectCursorPos};
use termion::event::*;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

use crate::models::tweet::Tweet;

pub fn home_timeline(timeline: &Vec<Tweet>) {
    let stdin = stdin();
    let mut stdout = stdout().into_raw_mode().unwrap();

    write!(stdout, "{}{}", clear::All, cursor::Goto(1, 1)).unwrap();

    for tweet in timeline {
        write!(stdout, "{}", tweet.text).unwrap();
        let (_x, y) = stdout.cursor_pos().unwrap();
        write!(stdout, "{}", cursor::Goto(1, y + 1)).unwrap();
    }

    stdout.flush().unwrap();

    for c in stdin.keys() {
        match c.unwrap() {
            Key::Char('q') => break,
            Key::Char(c) => {
                write!(stdout, "{}", c).unwrap();
            }
            _ => {}
        }

        stdout.flush().unwrap();
    }

    write!(stdout, "{}{}", clear::All, cursor::Goto(1, 1)).unwrap();
}
