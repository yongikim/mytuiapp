extern crate anyhow;
extern crate kuon;

use anyhow::Result;
use kuon::TwitterAPI;
use std::fs;
use std::io::Write;
use std::process::{Command, Stdio};
use termion::{clear, cursor};

pub async fn call<W: Write>(writer: &mut W, api: &TwitterAPI) -> Result<()> {
    let text = compose_tweet(writer);
    api.tweet().status(&text).send().await?;

    Ok(())
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
