extern crate anyhow;
extern crate kuon;
extern crate termion;
extern crate tokio;

use crate::{context::Context, pages::home_timeline::HomeTimeline};
use anyhow::Result;
use std::io::Write;
use termion::{clear, cursor};

pub struct Twee {
    pub context: Context,
}

impl Twee {
    pub async fn new() -> Result<Twee> {
        let context = Context::new().await?;

        Ok(Twee { context })
    }

    pub async fn start(&mut self) -> Result<()> {
        // Home
        let context = &mut self.context;
        let mut home_timeline = HomeTimeline::new(&context.api).await?;
        home_timeline.start(context).await?;

        // Quit
        self.quit()?;

        Ok(())
    }

    fn quit(&mut self) -> Result<()> {
        write!(
            self.context.screen,
            "{}{}{}{}",
            cursor::Goto(1, 1),
            clear::All,
            cursor::Goto(1, 1),
            cursor::Show,
        )
        .unwrap();
        self.context.screen.flush()?;

        Ok(())
    }
}
