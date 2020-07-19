use crate::apis;
use crate::views;

pub fn call() -> Vec<String> {
    let credits = apis::credits::get_credits();
    let timeline = apis::tweets::get_home_timeline(&credits);
    let tweets = views::tweets::home_timeline(&timeline);

    tweets
}
