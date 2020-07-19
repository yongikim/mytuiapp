use crate::apis;

use std::io::{Stdout, Write};
use termion::raw::RawTerminal;
use termion::screen::AlternateScreen;
use termion::{clear, cursor};

pub fn call(screen: &mut AlternateScreen<RawTerminal<Stdout>>) {
    let credits = apis::credits::get_credits();
    let text = "debug".to_string();
    let resp = apis::tweets::post_tweet(&credits, &text);

    // for debug
    write!(screen, "{}{}", clear::All, cursor::Goto(1, 1)).unwrap();
    write!(screen, "{:#?}", resp).unwrap();
    screen.flush().unwrap();
}
