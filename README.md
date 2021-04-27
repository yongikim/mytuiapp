# Rwitter
A TUI Twitter client written in Rust.

## Requirements
- macOS
- `vi` command to compose tweet

## Usage
### Credentials
Get credentials from Twitter Developer website and put it in the home directory.
```(~/.twitter_credential.json)
{
  "api_key": "xxxx",
  "api_secret_key": "xxxx"
  "access_token": "xxxx",
  "access_token_secret": "xxxx"
}
```

Then run
```
cargo run
```

## Key comamnds
- `r`: reload timeline
- `c`: compose tweet
- `q`: exit
