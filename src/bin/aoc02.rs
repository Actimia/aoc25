use anyhow::anyhow;
use itertools::Itertools;

const INPUT: &str = include_str!("data/02.txt");

struct IdRange {
  min: u64,
  max: u64,
}

impl TryFrom<&str> for IdRange {
  type Error = anyhow::Error;

  fn try_from(value: &str) -> Result<Self, Self::Error> {
    let (min, max) = value
      .trim()
      .split_once('-')
      .ok_or(anyhow!("no range found"))?;
    Ok(Self {
      min: min.parse()?,
      max: max.parse()?,
    })
  }
}

#[allow(unused)]
fn is_invalid_id(id: &u64) -> bool {
  let as_str = format!("{id}");
  let (left, right) = as_str.split_at(as_str.len() / 2);
  let result = left == right;
  if result {
    eprintln!("{id} is invalid");
  }
  result
}

#[allow(unused)]
fn is_invalid_id2(id: &u64) -> bool {
  let as_str = format!("{id}");
  let bytes = as_str.as_bytes();

  let invalid = (1..=(bytes.len() / 2))
    .filter(|x| bytes.len() % x == 0)
    .any(|chunk_size| bytes.chunks_exact(chunk_size).all_equal());
  if invalid {
    eprintln!("{id} is invalid");
  }
  invalid
}

fn main() -> anyhow::Result<()> {
  let ranges = INPUT.split(',').flat_map(IdRange::try_from);

  let mut total = 0u64;
  for IdRange { min, max } in ranges {
    eprintln!("Checking IDs between {min} and {max}");
    let invalids: u64 = (min..=max).filter(is_invalid_id2).sum();
    total += invalids;
  }
  println!("{total}");
  Ok(())
}
