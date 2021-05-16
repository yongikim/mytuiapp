use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::BufReader;

#[derive(Serialize, Deserialize, Debug)]
pub struct Credits {
    pub api_key: String,
    pub api_secret_key: String,
    pub access_token: String,
    pub access_token_secret: String,
}

impl Default for Credits {
    fn default() -> Self {
        Self::new()
    }
}

impl Credits {
    pub fn new() -> Self {
        let path = {
            let mut path = dirs::home_dir().unwrap();
            path.push(".twitter_credentials.json");
            path
        };
        let file = File::open(path).unwrap();
        let reader = BufReader::new(file);
        let credits: Credits = serde_json::from_reader(reader).unwrap();

        credits
    }
}
