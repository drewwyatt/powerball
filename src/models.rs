use rand::prelude::*;

fn draw_white() -> [i8; 5] {
  let mut rng = rand::thread_rng();
  let mut all_balls = (1..=69).collect::<Vec<i8>>();
  all_balls.shuffle(&mut rng);

  let mut selections: [i8; 5] = [0; 5];
  for (index, draw) in all_balls.drain(0..5).enumerate() {
    selections[index] = draw;
  }

  selections
}

fn draw_powerball() -> i8 {
  rand::thread_rng().gen_range(1..=26)
}

pub enum Prize {
  FourDollars,
  SevenDollars,
  OneHundredDollars,
  FiftyThousandDollars,
  OneMillionDollars,
  Jackpot
}

pub struct Numbers {
  pub powerball: i8,
  pub white_balls: [i8; 5],
}

impl Numbers {
  pub fn new(white_balls: [i8; 5], powerball: i8) -> Numbers {
    let mut sorted = white_balls.clone();
    sorted.sort();
    Numbers {
      powerball: powerball,
      white_balls: sorted,
    }
  }

  pub fn draw() -> Numbers {
    Numbers::new(draw_white(), draw_powerball())
  }

  pub fn get_prize(&self, winners: &Numbers) -> Option<Prize> {
    match (self.get_matches(winners), self.powerball == winners.powerball) {
      (5, true) => Some(Prize::Jackpot),
      (5, _) => Some(Prize::OneMillionDollars),
      (4, true) => Some(Prize::FiftyThousandDollars),
      (4, _) | (3, true) => Some(Prize::OneHundredDollars),
      (3, _) | (2, true) => Some(Prize::SevenDollars),
      (1, _) => Some(Prize::FourDollars),
      _ => None,
    }
  }

  fn get_matches(&self, winners: &Numbers) -> i8 {
    let mut matches = 0;
    for ball in &self.white_balls {
      if winners.white_balls.contains(ball) {
        matches += 1;
      }
    }

    matches
  }
}

