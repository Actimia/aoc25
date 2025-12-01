const INPUT: &'static str = include_str!("data/01.txt");

#[derive(Debug, PartialEq, Eq)]
struct Turn(i32);

impl TryFrom<&str> for Turn {
  fn try_from(value: &str) -> Result<Turn, anyhow::Error> {
    if let Some(value) = value.strip_prefix('R') {
      let value = value.parse::<i32>()?;
      Ok(Turn(value))
    } else if let Some(value) = value.strip_prefix('L') {
      let value = value.parse::<i32>()?;
      Ok(Turn(-value))
    } else {
      anyhow::bail!("invalid input")
    }
  }

  type Error = anyhow::Error;
}

fn main() {
  let turns = INPUT.lines().map(Turn::try_from).flatten();

  let mut zeros = 0;
  let mut acc = 50;
  for Turn(turn) in turns {
    let mut this_zeros = 0;

    let step = turn.signum();
    let count = turn.abs();
    for _ in 0..count {
      acc += step;
      if acc == 100 {
        acc = 0;
      } else if acc == -1 {
        acc = 99;
      }

      if acc == 0 {
        this_zeros += 1;
      }
    }

    println!("Turned {} to {} ({} + {})", turn, acc, zeros, this_zeros);
    zeros += this_zeros;
  }
  println!("{zeros}");
}
