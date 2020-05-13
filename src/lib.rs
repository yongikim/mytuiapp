extern crate termion;

pub mod credits;
pub mod twitter;
pub mod vim;

// Import the coor module.
use std::io::{stdin, stdout, Write};
use std::thread;
use std::time::Duration;
use termion::event::*;
use termion::input::{MouseTerminal, TermRead};
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

pub fn vim() {
    let stdin = stdin();
    let mut stdout = stdout().into_raw_mode().unwrap();

    write!(
        stdout,
        "{}{}q to exit. Type stuff, use alt, and so on.",
        termion::clear::All,
        termion::cursor::Goto(1, 1)
    )
    .unwrap();

    stdout.flush().unwrap();

    for c in stdin.keys() {
        match c.unwrap() {
            Key::Char('q') => break,
            Key::Char('h') => {
                write!(stdout, "{}{}", cursor::Left(1), cursor::Show).unwrap();
            }
            Key::Char('j') => {
                write!(stdout, "{}{}", cursor::Down(1), cursor::Show).unwrap();
            }
            Key::Char('k') => {
                write!(stdout, "{}{}", cursor::Up(1), cursor::Show).unwrap();
            }
            Key::Char('l') => {
                write!(stdout, "{}{}", cursor::Right(1), cursor::Show).unwrap();
            }
            Key::Char(c) => {
                write!(stdout, "{}", c).unwrap();
            }
            _ => {}
        }

        stdout.flush().unwrap();
    }

    write!(
        stdout,
        "{}{}{}",
        termion::clear::All,
        cursor::Goto(1, 1),
        termion::cursor::Show
    )
    .unwrap();
}

pub fn mouse() {
    let stdin = stdin();
    let mut stdout = MouseTerminal::from(stdout().into_raw_mode().unwrap());

    writeln!(
        stdout,
        "{}{}{}q to exit. Type stuff, use alt, click around...{}{}",
        termion::clear::All,
        termion::cursor::Goto(1, 1),
        color::Fg(termion::color::Rgb(255, 0, 0)),
        color::Fg(termion::color::Reset),
        termion::cursor::Hide
    )
    .unwrap();

    for c in stdin.events() {
        let evt = c.unwrap();
        match evt {
            Event::Key(Key::Char('q')) => {
                write!(
                    stdout,
                    "{}{}",
                    termion::cursor::Goto(1, 1),
                    termion::clear::CurrentLine
                )
                .unwrap();
                break;
            }
            Event::Mouse(me) => match me {
                MouseEvent::Press(_, a, b) | MouseEvent::Release(a, b) | MouseEvent::Hold(a, b) => {
                    write!(stdout, "{}{}", cursor::Goto(a, b), cursor::Show).unwrap();
                }
            },
            _ => {}
        }

        stdout.flush().unwrap();
    }

    write!(stdout, "{}", termion::cursor::Show).unwrap();
}
