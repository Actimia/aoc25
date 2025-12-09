use anyhow::{anyhow, bail, ensure};
use aoc25::{exts::duration::DurationExt, time::time_try};

const INPUT: &str = include_str!("data/06.txt");

#[derive(Clone, Eq, PartialEq)]
enum Operator {
  Plus,
  Times,
}

impl TryFrom<&str> for Operator {
  type Error = anyhow::Error;

  fn try_from(value: &str) -> Result<Self, Self::Error> {
    Ok(match value {
      "+" => Self::Plus,
      "*" => Self::Times,
      c => bail!("unknown operator: {}", c),
    })
  }
}

#[derive(Clone, Eq, PartialEq)]
struct Problem {
  numbers: Vec<u64>,
  operator: Operator,
}

impl Problem {
  fn compute(&self) -> u64 {
    match self.operator {
      Operator::Plus => self.numbers.iter().sum(),
      Operator::Times => self.numbers.iter().product(),
    }
  }
}

#[derive(Clone, Eq, PartialEq)]
struct ProblemsOne(Vec<Problem>);

impl TryFrom<&str> for ProblemsOne {
  type Error = anyhow::Error;

  fn try_from(value: &str) -> Result<Self, Self::Error> {
    let mut lines: Vec<_> = value.lines().collect();
    ensure!(lines.len() >= 3, "too few lines ({})", lines.len());
    let mut numbers: Vec<Vec<u64>> = {
      let first = lines.first().unwrap();
      let split: Vec<_> = first.split_whitespace().collect();
      let numbers = vec![vec![]; split.len()];
      numbers
    };

    let last = lines.pop().ok_or(anyhow!("no last"))?;

    let operators: Vec<Operator> = last.split_whitespace().flat_map(|c| c.try_into()).collect();
    ensure!(
      operators.len() == numbers.len(),
      "too few operators found ({} != {})",
      operators.len(),
      numbers.len()
    );

    for line in lines {
      let split: Vec<u64> = line.split_whitespace().flat_map(|x| x.parse()).collect();
      ensure!(
        split.len() == numbers.len(),
        "too few numbers found: ({} != {})",
        split.len(),
        numbers.len()
      );
      split
        .into_iter()
        .zip(numbers.iter_mut())
        .for_each(|(x, list)| list.push(x));
    }

    let problems = operators
      .into_iter()
      .zip(numbers.into_iter())
      .map(|(operator, numbers)| Problem { numbers, operator })
      .collect();

    Ok(Self(problems))
  }
}

#[allow(unused)]
fn part_one(problems: ProblemsOne) -> Vec<u64> {
  // 6605396225322
  problems.0.iter().map(Problem::compute).collect()
}

#[derive(Clone, Eq, PartialEq)]
struct ProblemsTwo(Vec<Problem>);

impl TryFrom<&str> for ProblemsTwo {
  type Error = anyhow::Error;

  fn try_from(value: &str) -> Result<Self, Self::Error> {
    fn transpose(input: &str) -> anyhow::Result<Vec<String>> {
      let chars: Vec<Vec<char>> = input
        .lines()
        .filter(|l| !l.is_empty())
        .map(|l| l.chars().collect::<Vec<_>>())
        .collect();

      let range = 0..chars.first().unwrap().len();
      let result: Vec<String> = range
        .map(|idx| chars.iter().map(|l| l.get(idx).unwrap_or(&' ')).collect())
        .collect();

      Ok(result)
    }
    let new_lines: Vec<String> = transpose(value)?;

    let mut problems: Vec<Problem> = vec![];
    for problem in new_lines.split(|l| l.trim().is_empty()) {
      if problem.is_empty() {
        continue;
      }
      let operator = {
        let first = problem.first().unwrap();
        if first.ends_with("*") {
          Operator::Times
        } else if first.ends_with("+") {
          Operator::Plus
        } else {
          bail!("first line did not end with operator ('{}')", first)
        }
      };

      let numbers: Vec<u64> = problem
        .iter()
        .flat_map(|l| {
          let num = l[..l.len() - 1].trim();
          num.parse()
        })
        .collect();
      problems.push(Problem { numbers, operator })
    }

    Ok(Self(problems))
  }
}

#[allow(unused)]
fn part_two(problems: ProblemsTwo) -> Vec<u64> {
  // 11052310600986
  problems.0.iter().map(Problem::compute).collect()
}

fn main() -> anyhow::Result<()> {
  let (part_one, dur) = time_try(|| -> anyhow::Result<u64> {
    let problem = INPUT.try_into()?;
    let totals_one = part_one(problem);
    Ok(totals_one.iter().sum())
  })?;
  println!("Part 1: {part_one} (in {})", dur.display());

  let (part_two, dur) = time_try(|| -> anyhow::Result<u64> {
    let problem = INPUT.try_into()?;
    let total = part_two(problem);
    Ok(total.iter().sum())
  })?;
  println!("Part 2: {part_two} (in {})", dur.display());
  Ok(())
}

#[cfg(test)]
mod tests {
  use super::*;

  const SAMPLE_INPUT: &str = "123 328  51 64 \n 45 64  387 23 \n  6 98  215 314\n*   +   *   +  ";

  #[test]
  fn test_one() {
    let problems = SAMPLE_INPUT.try_into().unwrap();
    let totals = part_one(problems);
    assert_eq!(totals, vec![33210, 490, 4243455, 401]);
    let grand_total: u64 = totals.iter().sum();
    assert_eq!(grand_total, 4277556);
  }

  #[test]
  fn test_two() {
    let problems = SAMPLE_INPUT.try_into().unwrap();
    let totals = part_two(problems);
    assert_eq!(totals, vec![8544, 625, 3253600, 1058]);
    let grand_total: u64 = totals.iter().sum();
    assert_eq!(grand_total, 3263827);
  }
}
