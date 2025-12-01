use anyhow::{bail, ensure};

const INPUT: &'static str = include_str!("data/01.txt");

#[derive(Debug, PartialEq, Eq)]
struct Turn(i32);

impl TryFrom<&str> for Turn {
  type Error = anyhow::Error;

  fn try_from(value: &str) -> Result<Turn, anyhow::Error> {
    ensure!(value.len() >= 2, "input too short");
    Ok(match value.split_at(1) {
      ("R", val) => Turn(val.parse()?),
      ("L", val) => Turn(-val.parse()?),
      _ => bail!("invalid input"),
    })
  }
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

    eprintln!("Turned {} to {} ({} + {})", turn, acc, zeros, this_zeros);
    zeros += this_zeros;
  }
  println!("{zeros}");
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_parsing() {
    let a: Turn = "R56".try_into().unwrap();
    assert_eq!(a.0, 56);

    let a: Turn = "L33".try_into().unwrap();
    assert_eq!(a.0, -33);

    let a: Turn = "L133".try_into().unwrap();
    assert_eq!(a.0, -133);

    let a: Result<Turn, _> = "Q3".try_into();
    assert!(a.is_err());
  }
}
