# Rwitter
A TUI Twitter client written in Rust.

## Requirements
- `vi` command to compose tweet

## Usage
### Credentials
Get credentials from Twitter Developer website and put it in the home directory, and name it `~/.twitter_credentials.json`.
``` json
{
  "api_key": "xxxx",
  "api_secret_key": "xxxx",
  "access_token": "xxxx",
  "access_token_secret": "xxxx"
}
```

Then run
``` sh
cargo run
```

## Key comamnds
### Home Timeline page
- `r`: reload timeline
- `c`: compose tweet
- `q`: exit
- `j`: cursor down
- `k`: cursor up
- `return`: navigate to tweet detail page

### Tweet Detail page
- `f`: favorite
- `q`: back to home timeline page
