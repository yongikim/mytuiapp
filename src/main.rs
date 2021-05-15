extern crate anyhow;
extern crate kuon;
extern crate tokio;

use anyhow::Result;
use twee_rs::twee::Twee;

#[tokio::main]
async fn main() -> Result<()> {
    let mut app = Twee::new().await?;
    app.start().await?;

    Ok(())
}
