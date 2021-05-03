use crate::interactors::get_timeline_interactor::Render;
use serde::{Deserialize, Serialize};
use std::{io::Write, string::ToString};
use terminal_size::{terminal_size, Height, Width};
use termion::{color, cursor, style};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Tweet {
    pub created_at: String,
    pub text: String,
    pub retweet_count: i32,
    pub favorite_count: i32,
    pub user: User,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TweetLine {
    pub tweet: Tweet,
    pub is_focused: bool,
    pub index: usize,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct User {
    pub name: String,
    pub screen_name: String,
}

impl Render for TweetLine {
    fn render<W: Write>(&self, writer: &mut W) {
        write!(
            writer,
            "{}{}{}",
            cursor::Goto(1, self.index as u16 + 1),
            self.to_string(),
            cursor::Goto(1, self.index as u16 + 1),
        )
        .unwrap();
        writer.flush().unwrap();
    }
}

impl TweetLine {
    pub fn render_without_flush<W: Write>(&self, writer: &mut W) {
        write!(
            writer,
            "{}{}{}",
            cursor::Goto(1, self.index as u16 + 1),
            self.to_string(),
            cursor::Goto(1, self.index as u16 + 1),
        )
        .unwrap();
    }
}

impl ToString for TweetLine {
    fn to_string(&self) -> String {
        let (Width(column_size), Height(_row_size)) = terminal_size().unwrap();
        let time = &self.tweet.created_at;
        let lines: Vec<&str> = self.tweet.text.split("\n").collect();
        let text_max_width = column_size as usize - time.len() - 19;
        let mut text: String = lines.concat();
        let mut user_name = self.tweet.user.screen_name.clone();

        while self.count_str_width(&text) > text_max_width {
            text.pop();
        }

        while self.count_str_width(&text) < text_max_width {
            text.push(' ');
        }

        while user_name.len() < 15 {
            user_name.push(' ');
        }

        let mut string = if self.is_focused {
            format!(
                "{}{}{}{}{}: {} {}{}",
                cursor::Hide,
                style::Bold,
                color::Bg(color::Green),
                color::Fg(color::White),
                time,
                user_name,
                text,
                style::Reset
            )
        } else {
            format!(
                "{}{}{}{}{}: {}{}{} {}",
                cursor::Hide,
                color::Bg(color::Reset),
                color::Fg(color::Blue),
                time,
                color::Fg(color::Reset),
                color::Fg(color::Green),
                user_name,
                color::Fg(color::Reset),
                text,
            )
        };

        let (Width(_column_size), Height(row_size)) = terminal_size().unwrap();
        if row_size - 1 != self.index as u16 {
            string.push_str("\r\n");
        }

        string
    }
}

impl TweetLine {
    pub fn from_tweet(tweet: &Tweet, index: usize, is_focused: bool) -> TweetLine {
        TweetLine {
            tweet: tweet.clone(),
            index,
            is_focused,
        }
    }

    fn count_str_width(&self, s: &String) -> usize {
        let mut n = 0;
        for c in s.chars() {
            if c.len_utf8() == 1 {
                n += 1
            } else {
                n += 2
            }
        }

        n
    }
}
