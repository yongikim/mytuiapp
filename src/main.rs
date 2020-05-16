use rwitter::credits;
use rwitter::twitter;
use rwitter::view;

fn main() {
    let credits = credits::get_credits();
    let timeline = twitter::get_home_timeline(&credits);

    view::home_timeline::home_timeline(&timeline);
}
