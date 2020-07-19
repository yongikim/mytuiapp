pub mod apis;
pub mod controllers;
pub mod models;
pub mod views;

use std::io::{Stdout, Write};
use termion::raw::RawTerminal;
use termion::screen::AlternateScreen;
use termion::{clear, cursor};

pub fn flush_clear_all(screen: &mut AlternateScreen<RawTerminal<Stdout>>) {
    write!(screen, "{}{}", clear::All, cursor::Goto(1, 1)).unwrap();
    screen.flush().unwrap();
}

pub fn flush_tweets(screen: &mut AlternateScreen<RawTerminal<Stdout>>, tweets: &Vec<String>) {
    for tweet in tweets {
        write!(screen, "{}", tweet).unwrap();
    }
    screen.flush().unwrap();
}
