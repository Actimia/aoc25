use anyhow::{bail, ensure};

const INPUT: &'static str = include_str!("data/01.txt");

#[derive(Debug, PartialEq, Eq)]
struct Turn(i64);

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

fn multiples_of_100_between(a: i64, b: i64) -> u64 {
  let high = (a.max(b) as f64 / 100.).floor() as i64;
  let low = (a.min(b) as f64 / 100.).ceil() as i64;
  let adj = if a % 100 == 0 { 0 } else { 1 }; // do not count the starting point
  (high - low + adj) as u64
}

fn main() {
  let turns = INPUT.lines().map(Turn::try_from).flatten();

  let mut zeroes: u64 = 0;
  let mut total: i64 = 50;
  for Turn(turn) in turns {
    let prev = total;
    total += turn;
    let new_zeroes = multiples_of_100_between(prev, total);
    eprintln!("Turned {} to {} ({} + {})", turn, total, zeroes, new_zeroes);
    zeroes += new_zeroes;
  }
  println!("{zeroes}");
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

  #[test]
  fn test_multiples_between() {
    assert_eq!(multiples_of_100_between(101, 300), 2);
    assert_eq!(multiples_of_100_between(-101, 300), 5);
    assert_eq!(multiples_of_100_between(-1, 1), 1);
  }
}
