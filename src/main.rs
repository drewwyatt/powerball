mod models;

use indicatif::{ProgressBar, ProgressStyle};
use std::time::Duration;

use models::{Numbers, Winners};

fn setup_progress_bar() -> ProgressBar {
    let pb = ProgressBar::new_spinner();
    pb.enable_steady_tick(Duration::from_millis(120));
    pb.set_style(
        ProgressStyle::with_template("{spinner:.blue} {msg}")
            .unwrap()
            .tick_strings(&[
                "▹▹▹▹▹",
                "▸▹▹▹▹",
                "▹▸▹▹▹",
                "▹▹▸▹▹",
                "▹▹▹▸▹",
                "▹▹▹▹▸",
                "▪▪▪▪▪",
            ]),
    );

    pb
}

fn main() {
    let winning_numbers = Numbers::draw();
    let mut entries: Vec<Numbers> = vec![];
    let mut winners = Winners::new();
    let pb = setup_progress_bar();

    print!("winners: {}", winners);

    while !winners.has_jackpot() {
        let next = Numbers::draw();
        entries.push(next);
        pb.set_message(format!("checking entry #{} {}", entries.len(), winners));

        match next.get_prize(&winning_numbers) {
            Some(prize) => winners.record(prize),
            _ => {}
        }
    }

    pb.finish_with_message("Done!");
    println!("It took {} entries before someone hit the jackpot.", entries.len());
    println!("Of those...");
    println!("    {} people won a million dollars", winners.one_million_dollars);
    println!("    {} people won fifty thousand dollars", winners.fifty_thousand_dollars);
    println!("    {} people won a hundred dollars", winners.one_hundred_dollars);
    println!("    {} people won seven dollars", winners.seven_dollars);
    println!("    {} people won four dollars", winners.four_dollars);
}
