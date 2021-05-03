extern crate term_cursor;
extern crate unicode_width;

use crate::{interactors::get_timeline_interactor::Render, models::tweet::Tweet};
use std::io::{stdin, Write};
use terminal_size::{terminal_size, Height, Width};
use termion::{color, cursor, event::Key, input::TermRead};

pub struct TweetDetail<'a> {
    tweet: &'a Tweet,
}

impl<'a> Render for TweetDetail<'a> {
    fn render<W: Write>(&self, writer: &mut W) {
        let (Width(column_size), Height(row_size)) = terminal_size().unwrap();

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

        write!(
            writer,
            "{}{}{}",
            cursor::Goto(column_size / 2 + 2, 2),
            color::Fg(color::Green),
            self.tweet.user.screen_name
        )
        .unwrap();

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

        writer.flush().unwrap();
    }
}

impl<'a> TweetDetail<'a> {
    pub fn from_tweet(tweet: &'a Tweet) -> TweetDetail {
        TweetDetail { tweet }
    }

    pub fn start<W: Write>(&self, writer: &mut W) {
        self.render(writer);

        for c in stdin().keys() {
            match c.unwrap() {
                Key::Char('q') => break,

                _ => {}
            }
        }
    }
}
