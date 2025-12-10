use std::ops::RangeInclusive;

use aoc25::{
  exts::{duration::DurationExt, iterator::IteratorExt},
  time::{time, time_try},
};

const INPUT: &str = include_str!("data/05.txt");

#[derive(Clone, Eq, PartialEq)]
struct Inventory {
  fresh: Vec<RangeInclusive<u64>>,
  ids: Vec<u64>,
}

impl TryFrom<&str> for Inventory {
  type Error = anyhow::Error;

  fn try_from(value: &str) -> Result<Self, Self::Error> {
    let (ranges, ids) = value
      .split_once("\n\n")
      .ok_or(anyhow::anyhow!("part missing"))?;

    let fresh_ranges = ranges
      .lines()
      .map(|l| -> anyhow::Result<RangeInclusive<u64>> {
        let (lo, hi) = l.split_once("-").ok_or(anyhow::anyhow!("range missing"))?;
        Ok((lo.parse()?)..=(hi.parse()?))
      })
      .flatten_verbose()
      .collect();

    let ids = ids
      .lines()
      .map(|l| l.parse::<u64>())
      .flatten_verbose()
      .collect();

    Ok(Inventory {
      fresh: fresh_ranges,
      ids,
    })
  }
}

#[allow(unused)]
fn part_one(inventory: Inventory) -> usize {
  // 712
  let Inventory { ids, fresh } = inventory;
  ids
    .iter()
    .filter(|id| fresh.iter().any(|r| r.contains(id)))
    .count()
}

fn merge_ranges(ranges: Vec<RangeInclusive<u64>>) -> (Vec<RangeInclusive<u64>>, usize) {
  #[inline]
  fn overlaps(a: &RangeInclusive<u64>, b: &RangeInclusive<u64>) -> bool {
    a.start() <= b.end() && b.start() <= a.end()
  }

  let mut new = vec![];
  let mut count = 0;
  for range in ranges.into_iter() {
    if let Some(overlap) = new.iter_mut().find(|r| overlaps(r, &range)) {
      *overlap = (*overlap.start().min(range.start()))..=(*overlap.end().max(range.end()));
      count += 1;
    } else {
      new.push(range);
    }
  }
  (new, count)
}

#[allow(unused)]
fn part_two(inventory: Inventory) -> usize {
  // 338348170606125: too high
  // 332998283036769

  let mut ranges = inventory.fresh;
  loop {
    let (new, count) = merge_ranges(ranges);
    ranges = new;
    if count == 0 {
      break;
    }
  }

  ranges.into_iter().map(|r| r.count()).sum()
}

fn main() -> anyhow::Result<()> {
  let (inventory, dur) = time_try(|| INPUT.try_into())?;
  println!("Parsed input in {}", dur.display());

  let (part_one, dur) = time(|| part_one(inventory));
  println!("Part 1: {part_one} (in {})", dur.display());

  let inventory = INPUT.try_into()?;
  let (part_two, dur) = time(|| part_two(inventory));
  println!("Part 2: {part_two} (in {})", dur.display());
  Ok(())
}

#[cfg(test)]
mod tests {
  use super::*;

  const SAMPLE_INPUT: &str = "3-5\n10-14\n16-20\n12-18\n\n1\n5\n8\n11\n17\n32";

  #[test]
  fn test_one() {
    let inventory = SAMPLE_INPUT.try_into().unwrap();
    let fresh = part_one(inventory);
    assert_eq!(fresh, 3);
  }

  #[test]
  fn test_two() {
    let inventory = SAMPLE_INPUT.try_into().unwrap();
    let fresh = part_two(inventory);
    assert_eq!(fresh, 14);
  }
}
