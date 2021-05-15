extern crate anyhow;
extern crate kuon;
extern crate termion;

use crate::models::credits::Credits;
use anyhow::Result;
use std::io::{stdout, Stdout};
use termion::{
    raw::{IntoRawMode, RawTerminal},
    screen::AlternateScreen,
};

pub struct Context {
    pub api: kuon::TwitterAPI,
    pub screen: AlternateScreen<RawTerminal<Stdout>>,
}

impl Context {
    pub async fn new() -> Result<Self> {
        // Api
        let credits = Credits::new();
        let builder = kuon::TwitterAPI::builder()
            .access_token(&credits.access_token)
            .access_token_secret(&credits.access_token_secret)
            .api_key(&credits.api_key)
            .api_secret_key(&credits.api_secret_key);
        let api = builder.build().await?;

        // Screen
        let screen = AlternateScreen::from(stdout().into_raw_mode().unwrap());

        Ok(Context { api, screen })
    }
}
