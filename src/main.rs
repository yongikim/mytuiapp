use rwitter::interactors;
use rwitter::interactors::get_timeline_interactor::HomeTimeline;
use rwitter::interactors::get_timeline_interactor::Render;
use rwitter::models::credits::Credits;
use rwitter::models::cursor::Cursor;
use std::io::{stdin, stdout, Write};
use termion::event::*;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use termion::screen::AlternateScreen;
use termion::{clear, cursor};

/*
 * TODO
 * 1. Implement posting tweet
 * 2. Implement moving cursor
 * 3. Implement auto reload
 */
fn main() {
    let mut app = Rwitter::new();
    app.start()
}

struct Rwitter {
    pub credits: Credits,
    pub home_timeline: HomeTimeline,
}

impl Rwitter {
    fn new() -> Rwitter {
        // Get credentials
        let credits = Credits::new();

        // Home
        let home_timeline = interactors::get_timeline_interactor::HomeTimeline::new(&credits);

        Rwitter {
            credits,
            home_timeline,
        }
    }

    fn start(&mut self) {
        // Prepare screen
        let screen = &mut AlternateScreen::from(stdout().into_raw_mode().unwrap());

        self.home_timeline.render(screen);

        // Wait for user input
        for c in stdin().keys() {
            match c.unwrap() {
                // Quit app
                Key::Char('q') => break,

                // Reload timeline
                Key::Char('r') => {
                    self.home_timeline.update(&self.credits);
                    self.home_timeline.render(screen)
                }

                // Post a tweet
                Key::Char('c') => {
                    interactors::post_tweet_interactor::call(screen);
                }

                Key::Char('k') => {
                    self.home_timeline.handle_cursor_move(screen, -1);
                }

                Key::Char('j') => {
                    self.home_timeline.handle_cursor_move(screen, 1);
                }

                _ => {}
            }
        }

        // Quit
        interactors::quit_app_interactor::call(screen);
    }
}
