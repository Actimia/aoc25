#[macro_export]
macro_rules! assert_approx_eq {
  ($left:expr, $right:expr $(,)?) => {
    assert_approx_eq!($left, $right, 0.01)
  };
  ($left:expr, $right:expr, $tol:expr) => {
    let left: f64 = ($left);
    let right: f64 = ($right);
    let tolerance: f64 = ($tol);

    if (left - right).abs() > tolerance {
      panic!(
        "assertion `left ~= right` failed\n  left: {}\n right: {}",
        left, right
      );
    }
  };
}
