use rwitter::controllers;
use std::io::{stdin, stdout, Stdout, Write};
use termion::cursor::DetectCursorPos;
use termion::event::*;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use termion::raw::RawTerminal;
use termion::screen::AlternateScreen;
use termion::{clear, cursor};

// TODO: 一番下の行にツイート取得時刻を表示する
fn main() {
    let stdin = stdin();
    let mut screen = AlternateScreen::from(stdout().into_raw_mode().unwrap());

    // let (_x, _y) = screen.cursor_pos().unwrap();

    write!(screen, "{}{}", clear::All, cursor::Goto(1, 1)).unwrap();
    screen.flush().unwrap();

    let mut tweets = controllers::timeline_controller::call();
    for tweet in &tweets {
        write!(screen, "{}", tweet).unwrap();
    }
    screen.flush().unwrap();

    for c in stdin.keys() {
        match c.unwrap() {
            // Quit app
            Key::Char('q') => break,

            // Reload timeline
            Key::Char('r') => {
                tweets = controllers::timeline_controller::call();
                for tweet in &tweets {
                    write!(screen, "{}", tweet).unwrap();
                }
                screen.flush().unwrap();
            }

            Key::Char(_c) => {}
            _ => {}
        }
    }

    write!(screen, "{}{}", clear::All, cursor::Goto(1, 1)).unwrap();
    screen.flush().unwrap();
}

// TODO: 実装
fn clear_all_screen(screen: AlternateScreen<RawTerminal<Stdout>>) {}
