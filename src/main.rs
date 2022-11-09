mod models;
use models::{Numbers, Winners};

fn main() {
    let winning_numbers = Numbers::draw();
    let mut entries: Vec<Numbers> = vec![];
    let mut winners = Winners::new();

    while !winners.has_jackpot() {
        let next = Numbers::draw();
        entries.push(next);

        match next.get_prize(&winning_numbers) {
            Some(prize) => winners.record(prize),
            _ => {}
        }
    }

    println!("It took {} entries before someone hit the jackpot.", entries.len());
    println!("Of those...");
    println!("    {} people won a million dollars", winners.one_million_dollars);
    println!("    {} people won fifty thousand dollars", winners.fifty_thousand_dollars);
    println!("    {} people won a hundred dollars", winners.one_hundred_dollars);
    println!("    {} people won seven dollars", winners.seven_dollars);
    println!("    {} people won four dollars", winners.four_dollars);
}
