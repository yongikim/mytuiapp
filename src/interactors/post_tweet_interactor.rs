use crate::apis;

use std::fs;
use std::io::Write;
use std::process::{Command, Stdio};
use termion::{clear, cursor};

pub fn call<W: Write>(writer: &mut W) {
    let text = compose_tweet(writer);
    let credits = apis::credits::get_credits();
    let resp = apis::tweets::post_tweet(&credits, &text);

    /* Show reponse from the api for debug */
    write!(writer, "{}{}", clear::All, cursor::Goto(1, 1)).unwrap();
    write!(writer, "{:#?}", resp).unwrap();
    writer.flush().unwrap();
}

fn compose_tweet<W: Write>(writer: &mut W) -> String {
    let tweet_file = "tweet.txt";
    Command::new("vi")
        .arg(tweet_file)
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .output()
        .unwrap();

    write!(writer, "{}{}", clear::All, cursor::Goto(1, 1)).unwrap();
    writer.flush().unwrap();

    let tweet = fs::read_to_string(tweet_file).expect("Something went wrong reading the tweet");

    // TODO: validate tweet

    write!(writer, "{}", "posting tweet...").unwrap();
    writer.flush().unwrap();

    fs::remove_file("tweet.txt").unwrap();

    tweet
}
