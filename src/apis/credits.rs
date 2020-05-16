extern crate dirs;
extern crate serde;
extern crate serde_json;

use std::fs::File;
use std::io::BufReader;

use crate::models::credits::Credits;

// Read from config file for now.
// TODO: Hit twitter api to request access token.
pub fn get_credits() -> Credits {
    let mut path = dirs::home_dir().unwrap();
    path.push(".twitter_credentials.json");
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);
    let credits: Credits = serde_json::from_reader(reader).unwrap();

    credits
}
