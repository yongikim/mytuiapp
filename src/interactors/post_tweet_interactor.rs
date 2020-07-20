use crate::apis;

use std::error::Error;
use std::io::prelude::*;
use std::io::{stdout, Stdout, Write};
use std::os::unix::process::CommandExt;
use std::process::{Command, Stdio};
use termion::raw::RawTerminal;
use termion::screen::{AlternateScreen, ToMainScreen};
use termion::{clear, cursor};

pub fn call(screen: &mut AlternateScreen<RawTerminal<Stdout>>) {
    /*
     * TODO: vi起動用のサービス？を作り、パターンマッチで処理する。
     *       そのサービスでは、File I/Oも担う。
     */
    let mut process = match Command::new("vi").spawn() {
        Err(why) => panic!("{}", why),
        Ok(process) => process,
    };
    let result = process.wait().unwrap();

    // let credits = apis::credits::get_credits();
    // let text = "debug3".to_string();
    // let resp = apis::tweets::post_tweet(&credits, &text);

    /* for debug */
    write!(screen, "{}{}", clear::All, cursor::Goto(1, 1)).unwrap();
    write!(screen, "{:?}", result).unwrap();
    screen.flush().unwrap();
}
