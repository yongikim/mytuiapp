extern crate anyhow;
extern crate kuon;
extern crate tokio;

use anyhow::Result;
use kuon::TwitterAPI;
use rwitter::{
    interactors::quit_app_interactor, models::credits::Credits, pages::home_timeline::HomeTimeline,
};

use std::io::stdout;
use termion::{raw::IntoRawMode, screen::AlternateScreen};

#[tokio::main]
async fn main() -> Result<()> {
    let mut app = Rwitter::new().await?;
    app.start().await?;

    Ok(())
}

struct Rwitter {
    pub credits: Credits,
    pub home_timeline: HomeTimeline,
    pub api: TwitterAPI,
}

impl Rwitter {
    async fn new() -> Result<Rwitter> {
        // Get credentials
        let credits = Credits::new();
        let builder = kuon::TwitterAPI::builder()
            .access_token(&credits.access_token)
            .access_token_secret(&credits.access_token_secret)
            .api_key(&credits.api_key)
            .api_secret_key(&credits.api_secret_key);
        let api = builder.build().await?;

        // Home
        let home_timeline = HomeTimeline::new(&api).await?;

        Ok(Rwitter {
            credits,
            home_timeline,
            api,
        })
    }

    async fn start(&mut self) -> Result<()> {
        // Prepare screen
        let screen = &mut AlternateScreen::from(stdout().into_raw_mode().unwrap());

        self.home_timeline.start(screen, &self.api).await?;

        // Quit
        quit_app_interactor::call(screen);

        Ok(())
    }
}
