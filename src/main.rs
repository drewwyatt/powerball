mod balls;
use balls::{draw_white, draw_powerball};

fn main() {
    let white_balls = draw_white();
    let powerball = draw_powerball();

    println!("white: {:?}", white_balls);
    println!("powerball: {}", powerball);
}
