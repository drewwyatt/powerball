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

#[derive(Clone, Copy)]
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

pub struct Winners {
  pub four_dollars: i64,
  pub seven_dollars: i64,
  pub one_hundred_dollars: i64,
  pub fifty_thousand_dollars: i64,
  pub one_million_dollars: i64,
  pub jackpot: i8,
}

impl Winners {
  pub fn new() -> Winners {
      Winners {
          four_dollars: 0,
          seven_dollars: 0,
          one_hundred_dollars: 0,
          fifty_thousand_dollars: 0,
          one_million_dollars: 0,
          jackpot: 0,
      }
  }

  pub fn has_jackpot(&self) -> bool {
    self.jackpot > 0
  }

  pub fn record(&mut self, prize: Prize) {
      match prize {
          Prize::FourDollars => self.four_dollars += 1,
          Prize::SevenDollars => self.seven_dollars += 1,
          Prize::OneHundredDollars => self.one_hundred_dollars += 1,
          Prize::FiftyThousandDollars => self.fifty_thousand_dollars += 1,
          Prize::OneMillionDollars => self.one_million_dollars += 1,
          Prize::Jackpot => self.jackpot += 1,
      }
  }
}

