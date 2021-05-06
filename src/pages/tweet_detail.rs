extern crate anyhow;
extern crate kuon;
extern crate term_cursor;
extern crate unicode_width;

use crate::pages::home_timeline::Render;
use anyhow::Result;
use kuon::{Error, TrimTweet, TwitterAPI};
use std::{
    io::{stdin, Write},
    thread, time,
};
use terminal_size::{terminal_size, Height, Width};
use termion::{color, cursor, event::Key, input::TermRead, style};

pub struct TweetDetail {
    tweet: TrimTweet,
}

impl Render for TweetDetail {
    fn render<W: Write>(&self, writer: &mut W) {
        let (Width(column_size), Height(row_size)) = terminal_size().unwrap();

        // Open modal
        let empty = " ".repeat(column_size as usize / 2);
        for i in 0..(row_size as usize) {
            write!(
                writer,
                "{}{}{}{}",
                cursor::Goto(column_size / 2, i as u16 + 1),
                color::Fg(color::Blue),
                "|",
                empty,
            )
            .unwrap();
        }

        // Format to lines
        let mut chars = self.tweet.text.chars();
        let mut lines: Vec<String> = vec![];
        let mut flag = true;
        while flag {
            let mut line = "".to_string();
            while unicode_width::UnicodeWidthStr::width(line.as_str())
                < column_size as usize / 2 - 1
            {
                match chars.next() {
                    Some(c) => {
                        if c == '\n' {
                            break;
                        } else {
                            line.push(c);
                        }
                    }
                    None => {
                        flag = false;
                        break;
                    }
                }
            }
            lines.push(line);
        }

        let s = "".to_string();
        let user_name = self.tweet.user.name.as_ref().unwrap_or(&s);
        let screen_name = self.tweet.user.screen_name.as_ref().unwrap_or(&s);
        // Header
        write!(
            writer,
            "{}{}{}{}{} {}@{}",
            cursor::Goto(column_size / 2 + 2, 2),
            style::Bold,
            color::Fg(color::White),
            user_name,
            style::Reset,
            color::Fg(color::Green),
            screen_name
        )
        .unwrap();

        // Body
        for i in 0..lines.len() {
            let line = &lines[i];
            write!(
                writer,
                "{}{}{}",
                cursor::Goto(column_size / 2 + 2, i as u16 + 4),
                color::Fg(color::White),
                line,
            )
            .unwrap();
        }

        // Footer
        let rt_icon = if self.tweet.retweeted {
            format!(
                "{}{}♻  {}{}",
                style::Bold,
                color::Fg(color::Green),
                self.tweet.retweet_count,
                style::Reset,
            )
        } else {
            format!("{}♻  {}", color::Fg(color::White), self.tweet.retweet_count)
        };
        let fav_icon = if self.tweet.favorited {
            format!(
                "{}{}♡  {}{}",
                style::Bold,
                color::Fg(color::Red),
                self.tweet.favorite_count,
                style::Reset,
            )
        } else {
            format!(
                "{}♡  {}",
                color::Fg(color::White),
                self.tweet.favorite_count
            )
        };
        write!(
            writer,
            "{}{} {}",
            cursor::Goto(column_size / 2 + 2, lines.len() as u16 + 5),
            rt_icon,
            fav_icon
        )
        .unwrap();

        writer.flush().unwrap();
    }
}

impl TweetDetail {
    pub async fn from_tweet_id<W: Write>(
        screen: &mut W,
        id_str: &str,
        api: &TwitterAPI,
    ) -> Result<TweetDetail> {
        let (Width(column_size), Height(row_size)) = terminal_size().unwrap();
        match api.show_tweet().id(id_str).send().await {
            Ok(tweet) => Ok(TweetDetail { tweet }),
            Err(e) => {
                write!(screen, "{:?}", e).unwrap();
                screen.flush().unwrap();
                let two_secs = time::Duration::from_millis(10000);
                thread::sleep(two_secs);
                panic!("error");
            }
            _ => panic!("Unexpected error!"),
        }
    }

    pub async fn start<W: Write>(&mut self, writer: &mut W, api: &TwitterAPI) -> Result<()> {
        let (Width(column_size), Height(row_size)) = terminal_size().unwrap();

        self.render(writer);

        for c in stdin().keys() {
            match c.unwrap() {
                Key::Char('q') => break,
                Key::Char('f') => {
                    match api.favorite().id(self.tweet.id).send().await {
                        Ok(_) => {
                            let id = self.tweet.id;
                            match api.show_tweet().id(id).send().await {
                                Ok(tweet) => {
                                    self.tweet = tweet;
                                    self.render(writer);
                                }
                                _ => panic!("Unexpected error!"),
                            };
                        }
                        Err(Error::TwitterAPIError(e, _param_str)) => {
                            write!(
                                writer,
                                "{}{}",
                                cursor::Goto(column_size / 2 + 2, row_size),
                                e.to_string().trim()
                            )
                            .unwrap();
                            writer.flush().unwrap();

                            let two_secs = time::Duration::from_millis(2000);
                            thread::sleep(two_secs);

                            self.render(writer);
                        }
                        Err(Error::HTTPRequestError(_err)) => {}
                        _ => panic!("Unexpected error!"),
                    };
                }

                _ => {}
            }
        }

        Ok(())
    }
}
