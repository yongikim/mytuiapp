use rwitter::credits;
use rwitter::twitter;

fn main() {
    let credits = credits::get_credits();
    twitter::timeline(&credits);
}
