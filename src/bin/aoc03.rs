use std::fmt::Display;

const INPUT: &str = include_str!("data/03.txt");

struct Bank(Vec<u32>);

impl TryFrom<&str> for Bank {
  type Error = anyhow::Error;

  fn try_from(value: &str) -> Result<Self, Self::Error> {
    anyhow::ensure!(!value.is_empty(), "No data");
    let values: Vec<_> = value.bytes().map(|c| (c - b'0') as u32).collect();
    Ok(Self(values))
  }
}

impl Display for Bank {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    for d in self.0.iter() {
      write!(f, "{}", d)?;
    }
    Ok(())
  }
}
impl Bank {
  // 16951: too low
  // 17193
  #[allow(unused)]
  fn part_one(&self) -> Option<u32> {
    let Bank(bank) = self;
    // all except last
    let search = &bank[..(bank.len() - 1)];
    let first = search.iter().max()?;
    let first_index = search.iter().position(|x| x == first)?;

    // all after first
    let offset = first_index + 1;
    let search2 = &bank[offset..];
    let second = search2.iter().max()?;

    let max = (10 * first) + second;
    println!("{self}");
    println!("{first_index}: {first}, {second} ({max})");
    Some(max)
  }

  // 171297349921310
  #[allow(unused)]
  fn part_two(&self) -> Option<u64> {
    let Bank(bank) = self;

    // finds the largest digit and its offset in the haystack
    fn find_next(haystack: &[u32]) -> Option<(u32, usize)> {
      let value = haystack.iter().max()?;
      let index = haystack.iter().position(|x| x == value)?;
      Some((*value, index))
    }

    let mut joltage: u64 = 0;
    let mut start = 0;
    for end in (0..12).rev().map(|x| bank.len() - x) {
      let (value, offset) = find_next(&bank[start..end])?;
      joltage = (joltage * 10) + value as u64;
      start += offset + 1;
    }

    Some(joltage)
  }
}

fn main() -> anyhow::Result<()> {
  let banks = INPUT.split('\n').flat_map(Bank::try_from);

  let mut total = 0u64;

  for bank in banks {
    match bank.part_two() {
      Some(max) => total += max as u64,
      None => eprintln!("failure!"),
    }
  }

  println!("{total}");
  Ok(())
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_parsing() {
    let bank: Bank = "123456".try_into().unwrap();
    assert_eq!(bank.0, vec![1, 2, 3, 4, 5, 6])
  }

  #[test]
  fn test_part_one() {
    let bank: Bank = "123456123".try_into().unwrap();
    assert_eq!(bank.part_one(), Some(63));

    let bank: Bank = "987654321111111".try_into().unwrap();
    assert_eq!(bank.part_one(), Some(98));
    let bank: Bank = "811111111111119".try_into().unwrap();
    assert_eq!(bank.part_one(), Some(89));
    let bank: Bank = "234234234234278".try_into().unwrap();
    assert_eq!(bank.part_one(), Some(78));
    let bank: Bank = "818181911112111".try_into().unwrap();
    assert_eq!(bank.part_one(), Some(92));
    let bank: Bank = "818181989111121".try_into().unwrap();
    assert_eq!(bank.part_one(), Some(99));
  }

  #[test]
  fn test_part_two() {
    let bank: Bank = "987654321111111".try_into().unwrap();
    assert_eq!(bank.part_two(), Some(987654321111));
    let bank: Bank = "811111111111119".try_into().unwrap();
    assert_eq!(bank.part_two(), Some(811111111119));
    let bank: Bank = "234234234234278".try_into().unwrap();
    assert_eq!(bank.part_two(), Some(434234234278));
    let bank: Bank = "818181911112111".try_into().unwrap();
    assert_eq!(bank.part_two(), Some(888911112111));
  }
}
