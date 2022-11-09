mod models;
use models::Draw;

fn main() {
    let winners = Draw::new();

    println!("white: {:?}", winners.white_balls);
    println!("powerball: {}", winners.powerball);
}
