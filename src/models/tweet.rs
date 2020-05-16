use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Tweet {
    pub created_at: String,
    pub text: String,
    pub retweet_count: i32,
    pub favorite_count: i32,
    pub user: User,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    name: String,
    screen_name: String,
}
