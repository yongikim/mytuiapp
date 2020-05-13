extern crate oauth_client as oauth;
extern crate serde;
extern crate serde_json;

use crate::credits;

use oauth::Token;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
pub struct Tweet {
    created_at: String,
    text: String,
    retweet_count: i32,
    favorite_count: i32,
    user: User,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    name: String,
    screen_name: String,
}

pub fn timeline(credits: &credits::Credits) {
    let endpoint = "https://api.twitter.com/1.1/statuses/home_timeline.json";

    let consumer = Token::new(&credits.api_key, &credits.api_secret_key);
    let access = Token::new(&credits.access_token, &credits.access_token_secret);
    let req_param = HashMap::new();

    let bytes = oauth::get(endpoint, &consumer, Some(&access), Some(&req_param)).unwrap();
    let resp = String::from_utf8(bytes).unwrap();

    let json: Vec<Tweet> = serde_json::from_str(&resp).unwrap();

    println!("timeline response: {:#?}", json);
    println!("{} tweets", json.len());
}
