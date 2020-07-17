extern crate terminal_size;
extern crate termion;

use std::io::{stdout, Write};
use terminal_size::{terminal_size, Height, Width};
use termion::clear;
use termion::color;
use termion::cursor;
use termion::cursor::DetectCursorPos;
use termion::raw::IntoRawMode;
use termion::screen::AlternateScreen;

use crate::models::tweet::Tweet;

pub fn home_timeline(timeline: &Vec<Tweet>) {
    let mut screen = AlternateScreen::from(stdout().into_raw_mode().unwrap());
    let (Width(column_size), Height(row_size)) = terminal_size().unwrap();
    let mut v: Vec<String> = vec![];

    write!(screen, "{}{}", clear::All, cursor::Goto(1, 1)).unwrap();
    screen.flush().unwrap();

    for tweet in timeline {
        let time: String = tweet.created_at.clone();
        let lines: Vec<&str> = tweet.text.split("\n").collect();
        let text_max_width = column_size as usize - time.len() - 18;
        let mut text: String = lines.concat();
        let mut user_name = tweet.user.screen_name.clone();

        while count_str_width(&text) > text_max_width {
            text.pop();
        }

        while user_name.len() < 15 {
            user_name.push(' ');
        }

        v.push(format!(
            "{}{}{}: {}{}{} {}\r\n",
            color::Fg(color::Blue),
            time,
            color::Fg(color::Reset),
            color::Fg(color::Green),
            user_name,
            color::Fg(color::Reset),
            text,
        ));
    }

    let mut row_offset = 0;
    if v.len() > row_size as usize {
        row_offset = v.len() - (row_size as usize) + 1;
    }

    for l in row_offset..v.len() {
        write!(screen, "{}", v[l]).unwrap();
    }
    let (_x, y) = screen.cursor_pos().unwrap();
    write!(
        screen,
        "tweets: {:?}, offset: {:?}{}",
        v.len(),
        row_offset,
        cursor::Goto(1, y - 1)
    )
    .unwrap();
    screen.flush().unwrap();
}

fn count_str_width(s: &String) -> usize {
    // 文字幅を計算する
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
