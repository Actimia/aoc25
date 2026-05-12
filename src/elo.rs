pub enum Result {
  White,
  Tie,
  Black,
}

pub fn elo(white: f64, black: f64, result: Result) -> (f64, f64) {
  const K: f64 = 32.;

  let (result_white, result_black) = match result {
    Result::White => (1.0, 0.0),
    Result::Tie => (0.5, 0.5),
    Result::Black => (0.0, 1.0),
  };
  let (expected_white, expected_black) = expected_win(white, black);

  let white_delta = K * (result_white - expected_white);
  let black_delta = K * (result_black - expected_black);

  (white_delta, black_delta)
}

pub fn expected_win(white: f64, black: f64) -> (f64, f64) {
  let q_white = 10.0f64.powf(white / 400.0);
  let q_black = 10.0f64.powf(black / 400.0);
  let q_sum = q_white + q_black;
  (q_white / q_sum, q_black / q_sum)
}

#[cfg(test)]
pub mod tests {
  use crate::assert_approx_eq;

  use super::*;

  #[test]
  fn test_expected_win() {
    assert_eq!(expected_win(1500.0, 1500.0), (0.5, 0.5));
  }

  #[test]
  fn test_elo() {
    assert_eq!(elo(1500.0, 1500.0, Result::White), (16.0, -16.0));
    assert_eq!(elo(1500.0, 1500.0, Result::Tie), (0.0, 0.0));
    assert_eq!(elo(1500.0, 1500.0, Result::Black), (-16.0, 16.0));

    let (w_delta, b_delta) = elo(2400.0, 2700.0, Result::White);
    assert_approx_eq!(w_delta, 27.17);
    assert_approx_eq!(b_delta, -27.17);

    let (w_delta, b_delta) = elo(2400.0, 2700.0, Result::Tie);
    assert_approx_eq!(w_delta, 11.17);
    assert_approx_eq!(b_delta, -11.17);

    let (w_delta, b_delta) = elo(2400.0, 2700.0, Result::Black);
    assert_approx_eq!(w_delta, -4.83);
    assert_approx_eq!(b_delta, 4.83);
  }
}
