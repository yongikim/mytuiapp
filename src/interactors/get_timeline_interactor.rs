extern crate terminal_size;
extern crate termion;

use std::io::Write;
use terminal_size::{terminal_size, Height, Width};

use crate::{
    apis,
    models::{credits::Credits, cursor::Cursor, tweet::TweetLine},
};

pub trait Render {
    fn render<W: Write>(&self, writer: &mut W);
}

pub struct HomeTimeline {
    cursor: Cursor,
    tweet_lines: Vec<TweetLine>,
}

impl Render for HomeTimeline {
    fn render<W: Write>(&self, writer: &mut W) {
        for tweet_line in &self.tweet_lines {
            tweet_line.render(writer);
        }
    }
}

impl HomeTimeline {
    pub fn new(credits: &Credits) -> HomeTimeline {
        let cursor = Cursor { x: 1, y: 1 };

        let (Width(_column_size), Height(row_size)) = terminal_size().unwrap();
        let timeline = apis::tweets::get_home_timeline(credits);
        let tweet_lines = timeline[0..(row_size as usize - 1)]
            .iter()
            .enumerate()
            .map(|(i, tweet)| {
                let is_focused = cursor.y as usize == i + 1;
                TweetLine::from_tweet(tweet, i, is_focused)
            })
            .collect();

        HomeTimeline {
            cursor,
            tweet_lines,
        }
    }

    pub fn update(&mut self, credits: &Credits) {
        let Self { cursor, .. } = self;

        let (Width(_column_size), Height(row_size)) = terminal_size().unwrap();
        let timeline = apis::tweets::get_home_timeline(credits);
        let tweet_lines = timeline[0..(row_size as usize - 1)]
            .iter()
            .enumerate()
            .map(|(i, tweet)| {
                let is_focused = cursor.y as usize == i + 1;
                TweetLine::from_tweet(tweet, i, is_focused)
            })
            .collect();

        self.tweet_lines = tweet_lines;
    }

    pub fn handle_cursor_move<W: Write>(&mut self, writer: &mut W, dy: i32) {
        let (Width(_column_size), Height(row_size)) = terminal_size().unwrap();
        let pre_y = self.cursor.y;
        let new_y = self.cursor.y as i32 + dy;
        if new_y > 0 && new_y < row_size as i32 {
            self.cursor.y = new_y as u16;

            // new
            let tweet_line = &mut self.tweet_lines[new_y as usize - 1];
            tweet_line.is_focused = true;
            tweet_line.render(writer);

            // prev
            let tweet_line = &mut self.tweet_lines[pre_y as usize - 1];
            tweet_line.is_focused = false;
            tweet_line.render(writer);
        }
    }
}
