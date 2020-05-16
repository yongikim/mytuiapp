use rwitter::credits;
use rwitter::twitter;

fn main() {
    let credits = credits::get_credits();
    let timeline = twitter::get_home_timeline(&credits);

    println!("{:#?}", timeline);
    println!("{:?} tweets", timeline.len());
}
