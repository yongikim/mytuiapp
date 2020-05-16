use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Credits {
    pub api_key: String,
    pub api_secret_key: String,
    pub access_token: String,
    pub access_token_secret: String,
}
