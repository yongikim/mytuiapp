extern crate anyhow;
extern crate kuon;
extern crate terminal_size;
extern crate termion;

use anyhow::Result;
use kuon::{TrimTweet, TwitterAPI};
use std::io::{stdin, Write};
use terminal_size::{terminal_size, Height, Width};
use termion::{event::*, input::TermRead};

use crate::{
    context::Context,
    interactors::post_tweet_interactor,
    models::{cursor::Cursor, tweet::TweetLine},
    pages::tweet_detail::TweetDetail,
    render::Render,
};

pub struct HomeTimeline {
    cursor: Cursor,
    tweet_lines: Vec<TweetLine>,
}

impl Render for HomeTimeline {
    fn render<W: Write>(&self, writer: &mut W) {
        for tweet_line in &self.tweet_lines {
            tweet_line.render_without_flush(writer);
        }
        writer.flush().unwrap();
    }
}

impl HomeTimeline {
    pub async fn new(api: &TwitterAPI) -> Result<HomeTimeline> {
        let cursor = Cursor { x: 1, y: 1 };

        let (Width(_column_size), Height(row_size)) = terminal_size().unwrap();
        let timeline = api.home_timeline().count(100).send().await?;
        let tweet_lines = timeline[0..(row_size as usize)]
            .iter()
            .enumerate()
            .map(|(i, tweet)| {
                let is_focused = cursor.y as usize == i + 1;
                TweetLine::from_tweet(tweet, i, is_focused)
            })
            .collect();

        Ok(HomeTimeline {
            cursor,
            tweet_lines,
        })
    }

    pub async fn update(&mut self, api: &TwitterAPI) -> Result<()> {
        let Self { cursor, .. } = self;

        let (Width(_column_size), Height(row_size)) = terminal_size().unwrap();
        let timeline = api.home_timeline().count(100).send().await?;
        let tweet_lines = timeline[0..(row_size as usize)]
            .iter()
            .enumerate()
            .map(|(i, tweet)| {
                let is_focused = cursor.y as usize == i + 1;
                TweetLine::from_tweet(tweet, i, is_focused)
            })
            .collect();

        self.tweet_lines = tweet_lines;
        Ok(())
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

    pub fn get_focused_tweet(&self) -> &TrimTweet {
        let tweet_line = self
            .tweet_lines
            .iter()
            .find(|tweet_line| tweet_line.is_focused)
            .unwrap();

        &tweet_line.tweet
    }
}

impl HomeTimeline {
    pub async fn start(&mut self, context: &mut Context) -> Result<()> {
        self.render(&mut context.screen);

        // Wait for user input
        for c in stdin().keys() {
            let mut render_home = false;

            match c.unwrap() {
                // Quit app
                Key::Char('q') => break,

                // Reload timeline
                Key::Char('r') => {
                    self.update(&context.api).await?;
                    render_home = true;
                }

                // Post a tweet
                Key::Char('c') => {
                    post_tweet_interactor::call(context).await?;
                    render_home = true;
                }

                Key::Char('k') => {
                    self.handle_cursor_move(&mut context.screen, -1);
                }

                Key::Char('j') => {
                    self.handle_cursor_move(&mut context.screen, 1);
                }

                Key::Char('\n') => {
                    let tweet = self.get_focused_tweet();
                    let s = "".to_string();
                    let mut tweet_detail_page = TweetDetail::from_tweet_id(
                        &mut context.screen,
                        tweet.id_str.as_ref().unwrap_or(&s),
                        &context.api,
                    )
                    .await?;

                    tweet_detail_page.start(context).await?;
                    render_home = true;
                }

                _ => {}
            }
            if render_home {
                self.render(&mut context.screen);
            }
        }

        Ok(())
    }
}
