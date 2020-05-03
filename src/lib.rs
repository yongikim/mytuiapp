extern crate termion;

// Import the coor module.
use std::io::{stdin, stdout, Write};
use std::thread;
use std::time::Duration;
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use termion::{clear, color, cursor, style};

pub fn color() {
    println!(
        "{red}more red than any comrade{reset}",
        red = color::Fg(color::Red),
        reset = color::Fg(color::Reset)
    );
}

pub fn clear() {
    println!("{}", clear::All);
}

pub fn cursor() {
    println!(
        "{red}more red than any comrade{reset}",
        red = color::Fg(color::Red),
        reset = color::Fg(color::Reset)
    );
    thread::sleep(Duration::from_millis(1000));
    println!("\r");
    print!(
        "{clear}{red}g{blue}a{green}y{red} space communism{reset}",
        clear = clear::CurrentLine,
        red = color::Fg(color::Red),
        blue = color::Fg(color::Blue),
        green = color::Fg(color::Green),
        reset = color::Fg(color::Reset)
    );
}

pub fn goto() {
    println!(
        "{clear}{goto}{red}more red than any comrade{reset}",
        clear = clear::All,
        goto = cursor::Goto(1, 1),
        red = color::Fg(color::Red),
        reset = color::Fg(color::Reset)
    );
}

pub fn style() {
    println!(
        "{clear}{goto}{bold}{red}g{blue}a{green}y{red} space communism{reset}",
        clear = clear::All,
        goto = cursor::Goto(1, 1),
        bold = style::Bold,
        red = color::Fg(color::Red),
        blue = color::Fg(color::Blue),
        green = color::Fg(color::Green),
        reset = style::Reset
    );
}

pub fn raw() {
    let mut stdout = stdout().into_raw_mode().unwrap();
    writeln!(stdout, "Hey there.").unwrap();
}

pub fn inputs() {
    let stdin = stdin();
    let mut stdout = stdout().into_raw_mode().unwrap();

    write!(
        stdout,
        "{}{}q to exit. Type stuff, use alt, and so on.{}",
        termion::clear::All,
        termion::cursor::Goto(1, 1),
        termion::cursor::Hide
    )
    .unwrap();

    stdout.flush().unwrap();

    for c in stdin.keys() {
        write!(
            stdout,
            "{}{}",
            termion::cursor::Goto(1, 1),
            termion::clear::CurrentLine
        )
        .unwrap();

        match c.unwrap() {
            Key::Char('q') => break,
            Key::Char(c) => println!("{}", c),
            Key::Alt(c) => println!("Alt-{}", c),
            Key::Ctrl(c) => println!("Ctrl-{}", c),
            Key::Left => println!("<left>"),
            Key::Right => println!("<right>"),
            Key::Up => println!("<up>"),
            Key::Down => println!("<down>"),
            _ => println!("Other"),
        }

        // stdout.flush().unwrap();
    }

    write!(stdout, "{}", termion::cursor::Show).unwrap();
}
