use rwitter::controllers;
use std::io::{stdin, stdout};
use termion::cursor::DetectCursorPos;
use termion::event::*;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use termion::screen::AlternateScreen;

// TODO: 一番下の行にツイート取得時刻を表示する
fn main() {
    let stdin = stdin();
    let mut screen = AlternateScreen::from(stdout().into_raw_mode().unwrap());
    let mut tweets = controllers::timeline_controller::call();

    // let (_x, _y) = screen.cursor_pos().unwrap();

    rwitter::flush_clear_all(&mut screen);
    rwitter::flush_tweets(&mut screen, &tweets);

    for c in stdin.keys() {
        match c.unwrap() {
            // Quit app
            Key::Char('q') => break,

            // Reload timeline
            Key::Char('r') => {
                tweets = controllers::timeline_controller::call();
                rwitter::flush_tweets(&mut screen, &tweets);
            }

            Key::Char(_c) => {}
            _ => {}
        }
    }

    rwitter::flush_clear_all(&mut screen);
}
