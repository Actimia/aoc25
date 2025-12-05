use std::ops::RangeInclusive;

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
      .flat_map(|l| -> anyhow::Result<RangeInclusive<u64>> {
        let (lo, hi) = l.split_once("-").ok_or(anyhow::anyhow!("range missing"))?;
        Ok((lo.parse()?)..=(hi.parse()?))
      })
      .collect();

    let ids = ids.lines().flat_map(|l| l.parse()).collect();

    Ok(Inventory {
      fresh: fresh_ranges,
      ids,
    })
  }
}

#[allow(unused)]
fn part_one(inventory: Inventory) -> usize {
  // 712
  inventory
    .ids
    .iter()
    .filter(|id| inventory.fresh.iter().any(|r| r.contains(id)))
    .count()
}

fn overlaps(a: &RangeInclusive<u64>, b: &RangeInclusive<u64>) -> bool {
  b.contains(a.start()) || b.contains(a.end()) || a.contains(b.start()) && a.contains(b.end())
}

fn merge_ranges(ranges: Vec<RangeInclusive<u64>>) -> (Vec<RangeInclusive<u64>>, usize) {
  let mut new = vec![];
  let mut count = 0;
  for range in ranges.into_iter() {
    if let Some(r) = new.iter_mut().find(|r| overlaps(r, &range)) {
      let merged = (*r.start().min(range.start()))..=(*r.end().max(range.end()));
      eprintln!(
        "merging {}-{} and {}-{} to {}-{}",
        range.start(),
        range.end(),
        r.start(),
        r.end(),
        merged.start(),
        merged.end(),
      );
      count += 1;
      *r = merged;
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
  let inventory: Inventory = INPUT.try_into()?;
  let part_one = part_one(inventory.clone());
  println!("Part I: {part_one}");

  let part_two = part_two(inventory);
  println!("Part II: {part_two}");
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
