use rand::prelude::*;

pub fn draw_white() -> [i8; 5] {
  let mut rng = rand::thread_rng();
  let mut all_balls = (1..=69).collect::<Vec<i8>>();
  all_balls.shuffle(&mut rng);

  let mut selections: [i8; 5] = [0; 5];
  for (index, draw) in all_balls.drain(0..5).enumerate() {
    selections[index] = draw;
  }

  selections
}

pub fn draw_powerball() -> i8 {
  rand::thread_rng().gen_range(1..26)
}
