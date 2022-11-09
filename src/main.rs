mod balls;

fn main() {
    let winners = balls::Draw::new();

    println!("white: {:?}", winners.white_balls);
    println!("powerball: {}", winners.powerball);
}
