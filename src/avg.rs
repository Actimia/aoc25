pub fn average(data: &[f64]) -> f64 {
  data.iter().sum::<f64>() / data.len() as f64
}

pub fn rolling_average(data: &[f64], window: usize) -> impl Iterator<Item = f64> {
  data.windows(window).map(average)
}

pub fn weighted_rolling_average(data: &[f64], weights: &[f64]) -> impl Iterator<Item = f64> {
  let weight_sum = weights.iter().sum::<f64>();
  data
    .windows(weights.len())
    .map(move |xs| xs.iter().zip(weights).map(|(x, w)| x * w).sum::<f64>() / weight_sum)
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_average() {
    let data = vec![2.0, 4.0, 12.0];
    let avgs = average(&data);
    assert_eq!(avgs, 6.0);
  }
  #[test]
  fn test_rolling() {
    let data = vec![3.0, 6.0, 15.0, 9.0, 0.0];
    let avgs: Vec<_> = rolling_average(&data, 3).collect();
    assert_eq!(avgs, [8.0, 10.0, 8.0]);
  }
}
