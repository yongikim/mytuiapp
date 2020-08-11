use crate::apis;

use std::io::{stdin, Write};
use termion::event::*;
use termion::input::TermRead;
use termion::{clear, cursor};

pub fn call<W: Write>(writer: &mut W) {
    let text = compose_tweet(writer);
    let credits = apis::credits::get_credits();
    let resp = apis::tweets::post_tweet(&credits, &text);

    /* for debug */
    write!(writer, "{}{}", clear::All, cursor::Goto(1, 1)).unwrap();
    write!(writer, "{:?}", resp).unwrap();
    writer.flush().unwrap();
}

fn compose_tweet<W: Write>(writer: &mut W) -> String {
    write!(writer, "{}{}", clear::All, cursor::Goto(1, 1)).unwrap();
    writer.flush().unwrap();

    let stdin = stdin();
    for c in stdin.keys() {
        match c.unwrap() {
            Key::Ctrl('c') => {
                break;
            }
            Key::Char(c) => {
                /* TODO: store and edit text */
                write!(writer, "{}", c).unwrap();
            }
            _ => {}
        }

        writer.flush().unwrap();
    }

    "hi everyone.".to_string()
}
