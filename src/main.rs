use rwitter::apis;
use rwitter::views;

fn main() {
    let credits = apis::credits::get_credits();
    let timeline = apis::tweets::get_home_timeline(&credits);

    views::tweets::home_timeline(&timeline);
}
