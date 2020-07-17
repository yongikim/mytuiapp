use crate::apis;
use crate::views;

pub fn call() {
    let credits = apis::credits::get_credits();
    let timeline = apis::tweets::get_home_timeline(&credits);

    views::tweets::home_timeline(&timeline);
}
