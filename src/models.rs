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

pub struct Draw {
  pub powerball: i8,
  pub white_balls: [i8; 5],
}

impl Draw {
  pub fn from(white_balls: [i8; 5], powerball: i8) -> Draw {
    let mut sorted = white_balls.clone();
    sorted.sort();
    Draw {
      powerball: powerball,
      white_balls: sorted,
    }
  }

  pub fn new() -> Draw {
    Draw::from(draw_white(), draw_powerball())
  }
}

impl PartialEq for Draw {
  fn eq(&self, other: &Draw) -> bool {
    self.white_balls == other.white_balls && self.powerball == other.powerball
  }
}

impl Eq for Draw {
}
