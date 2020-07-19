use rwitter::interactors;
use std::io::{stdin, stdout};
use termion::cursor::DetectCursorPos;
use termion::event::*;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use termion::screen::AlternateScreen;

fn main() {
    let stdin = stdin();
    let mut screen = AlternateScreen::from(stdout().into_raw_mode().unwrap());

    interactors::get_timeline_interactor::call(&mut screen);

    for c in stdin.keys() {
        match c.unwrap() {
            // Quit app
            Key::Char('q') => break,

            // Reload timeline
            Key::Char('r') => {
                interactors::get_timeline_interactor::call(&mut screen);
            }

            Key::Char(_c) => {}
            _ => {}
        }
    }

    interactors::quit_app_interactor::call(&mut screen);
}
