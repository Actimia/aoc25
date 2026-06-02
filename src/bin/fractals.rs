use std::{
  f32::consts::PI,
  ops::{Add, Mul},
  sync::{Arc, Mutex},
  time::Instant,
};

use anyhow::Result;
use aoc25::exts::duration::DurationExt;
use color::{Lch, OpaqueColor};
use image::{Rgb, RgbImage};
use rayon::iter::{IntoParallelIterator, ParallelIterator};

fn main() -> Result<()> {
  let start = Instant::now();
  let width = 1920 * 2;
  let height = 1080 * 2;
  let image = Arc::new(Mutex::new(RgbImage::new(width, height)));

  let center = Complex(-0.66, 0.0);
  let resolution = 0.00066;

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

  (0..width).into_par_iter().for_each(|x| {
    for y in 0..height {
      let pixel = mandelbrot(coords(x, y)).into();
      image.lock().unwrap().put_pixel(x as u32, y as u32, pixel);
    }
  });

  /*
  for x in 0..width {
    if x % 100 == 0 {
      println!("{x}/{width}");
    }
    for y in 0..height {
      let pixel = mandelbrot(coords(x, y));
      image.put_pixel(x as u32, y as u32, pixel.into());
    }
  }*/

  image.lock().unwrap().save("image.png")?;

  println!("Finished in {}", start.elapsed().display());
  Ok(())
}

enum FractalPixel {
  Diverges(usize),
  Converges(()),
}

const MAX_ITERS: usize = 4096;

impl From<FractalPixel> for Rgb<u8> {
  fn from(value: FractalPixel) -> Self {
    match value {
      FractalPixel::Diverges(iters) => {
        let s = iters as f32 / MAX_ITERS as f32;
        let v = 1.0 - (PI * s).cos().powf(2.0);

        let l = 75.0 - (75.0 * v);
        let c = 28.0 + (75.0 - (75.0 * v));
        let h = (360.0 * s).powf(1.5) % 360.0;
        let lch = OpaqueColor::<Lch>::new([l, c, h]);
        let rgb = lch.to_rgba8();

        Rgb([rgb.r, rgb.g, rgb.b])

        // Rgb([0, 4 * (x % 64) as u8, 4 * (x % 64) as u8])
      }
      FractalPixel::Converges(_) => Rgb([0, 0, 0]),
    }
  }
}

fn mandelbrot(c: Complex) -> FractalPixel {
  let mut iters = 0;
  let mut z = Complex::zero();
  //let mut seen = vec![];

  loop {
    //while z.abs2() < 4.0 && iters < 64 {
    z = (z * z) + c;
    iters += 1;
    /*if seen.contains(&z) {
      return FractalPixel::Converges(iters);
    } else {
      seen.push(z);
    }*/
    if iters > MAX_ITERS {
      return FractalPixel::Converges(());
    }
    if z.abs2() > 4.0 {
      return FractalPixel::Diverges(iters);
    }
  }
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
