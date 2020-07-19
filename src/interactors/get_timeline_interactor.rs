use crate::apis;
use crate::presenters;

use std::io::{Stdout, Write};
use termion::raw::RawTerminal;
use termion::screen::AlternateScreen;
use termion::{clear, cursor};

pub fn call(screen: &mut AlternateScreen<RawTerminal<Stdout>>) {
    let credits = apis::credits::get_credits();
    let timeline = apis::tweets::get_home_timeline(&credits);
    let tweets = presenters::tweets::home_timeline(&timeline);

    flush_tweets(screen, &tweets);
}

fn flush_tweets(screen: &mut AlternateScreen<RawTerminal<Stdout>>, tweets: &Vec<String>) {
    write!(screen, "{}{}", clear::All, cursor::Goto(1, 1)).unwrap();
    for tweet in tweets {
        write!(screen, "{}", tweet).unwrap();
    }
    screen.flush().unwrap();
}
