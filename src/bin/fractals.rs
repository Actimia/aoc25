use std::{
  ops::{Add, Mul},
  time::Instant,
};

use anyhow::Result;
use aoc25::exts::duration::DurationExt;
use image::{Rgb, RgbImage};

fn main() -> Result<()> {
  let start = Instant::now();
  let width = 1920 * 2;
  let height = 1080 * 2;
  let mut image = RgbImage::new(width, height);

  let center = Complex(-0.66, 0.0);
  let resolution = 0.0009;

  let re_min = center.re() - (width as f64 / 2.0) * resolution;
  let re_max = ((width / 2) as f64) * resolution;

  let im_min = center.im() - (height as f64 / 2.0) * resolution;
  let im_max = ((height / 2) as f64) * resolution;

  println!("re = {re_min}..{re_max}, im = {im_min}..{im_max}");

  let coords = |x: u32, y: u32| {
    let re = re_min + (x as f64 * resolution);
    let im = im_min + (y as f64 * resolution);
    Complex(re, im)
  };
  for x in 0..width {
    if x % 100 == 0 {
      println!("{x}/{width}");
    }
    for y in 0..height {
      let pixel = color(mandelbrot(coords(x, y)));
      image.put_pixel(x as u32, y as u32, pixel);
    }
  }

  image.save("image.png")?;

  println!("Finished in {}", start.elapsed().display());
  Ok(())
}

fn color(iters: Option<usize>) -> Rgb<u8> {
  // iters = 0..=64

  if let Some(iters) = iters {
    let value = iters as u8 * 4;
    Rgb([0, value, value])
  } else {
    Rgb([0, 0, 0])
  }
}

fn mandelbrot(c: Complex) -> Option<usize> {
  let mut iters = 0;
  let mut z = Complex::zero();

  loop {
    //while z.abs2() < 4.0 && iters < 64 {
    z = (z * z) + c;
    iters += 1;
    if iters > 64 {
      return None;
    }
    let abs2 = z.abs2();
    if abs2 < 1.0 {
        
    }
    if z.abs2() > 4.0 {
      break;
    }
  }

  Some(iters)
}

#[derive(Clone, Copy, PartialEq, Debug)]
struct Complex(f64, f64);

impl Complex {
  pub fn zero() -> Self {
    Complex(0.0, 0.0)
  }
  pub fn re(&self) -> f64 {
    self.0
  }

  pub fn im(&self) -> f64 {
    self.1
  }

  pub fn abs2(&self) -> f64 {
    self.0 * self.0 + self.1 * self.1
  }
}

impl Mul for Complex {
  type Output = Self;

  fn mul(self, rhs: Self) -> Self::Output {
    let Complex(a, b) = self;
    let Complex(c, d) = rhs;
    Complex(a * c - b * d, a * d + b * c)
  }
}

impl Mul<f64> for Complex {
  type Output = Self;

  fn mul(self, rhs: f64) -> Self::Output {
    let Complex(a, b) = self;
    Complex(a * rhs, b * rhs)
  }
}

impl Add for Complex {
  type Output = Self;

  fn add(self, rhs: Self) -> Self::Output {
    let Complex(a, b) = self;
    let Complex(c, d) = rhs;
    Complex(a + c, b + d)
  }
}
impl From<(f64, f64)> for Complex {
  fn from((re, im): (f64, f64)) -> Self {
    Self(re, im)
  }
}
