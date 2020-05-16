extern crate oauth_client as oauth;
extern crate serde;
extern crate serde_json;

use crate::models::credits::Credits;
use crate::models::tweet::Tweet;

use oauth::Token;
use std::collections::HashMap;

pub fn get_home_timeline(credits: &Credits) -> Vec<Tweet> {
    let endpoint = "https://api.twitter.com/1.1/statuses/home_timeline.json";

    let consumer = Token::new(&credits.api_key, &credits.api_secret_key);
    let access = Token::new(&credits.access_token, &credits.access_token_secret);
    let req_param = HashMap::new();

    // TODO: Suggest to reload instead of calling `panic!`
    let bytes = oauth::get(endpoint, &consumer, Some(&access), Some(&req_param)).unwrap();
    let resp = String::from_utf8(bytes).unwrap();

    let timeline: Vec<Tweet> = serde_json::from_str(&resp).unwrap();

    timeline
}
