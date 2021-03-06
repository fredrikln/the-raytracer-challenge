extern crate image;

use std::path::Path;
use std::env;

use image::{ImageBuffer, Rgb};
use std::ops::{Add, Mul, Sub};
use crate::utils::equal;

#[derive(Debug, Clone, Copy)]
pub struct Color {
  pub r: f64,
  pub g: f64,
  pub b: f64,
}

impl Color {
  fn to_rgb(&self) -> Rgb<u8> {
    Rgb([
      (self.r * 255 as f64).min(255.0) as u8,
      (self.g * 255 as f64).min(255.0) as u8,
      (self.b * 255 as f64).min(255.0) as u8
    ])
  }
}

impl PartialEq for Color {
  fn eq(&self, other: &Self) -> bool {
    equal(self.r, other.r) && equal(self.g, other.g) && equal(self.b, other.b)
  }
}

impl Add<Color> for Color {
  type Output = Color;

  fn add(self, other: Self) -> Color {
    Color {
      r: self.r + other.r,
      g: self.g + other.g,
      b: self.b + other.b
    }
  }
}

impl Sub<Color> for Color {
  type Output = Color;

  fn sub(self, other: Self) -> Color {
    Color {
      r: self.r - other.r,
      g: self.g - other.g,
      b: self.b - other.b
    }
  }
}

impl Mul<Color> for Color {
  type Output = Color;

  fn mul(self, other: Self) -> Color {
    Color {
      r: self.r * other.r,
      g: self.g * other.g,
      b: self.b * other.b
    }
  }
}

impl Mul<f64> for Color {
  type Output = Color;

  fn mul(self, rhs: f64) -> Color {
    Color {
      r: self.r * rhs,
      g: self.g * rhs,
      b: self.b * rhs
    }
  }
}

#[derive(Debug)]
pub struct Canvas {
  buffer: ImageBuffer<Rgb<u8>, Vec<u8>>
}

impl Canvas {
  pub fn new(width: u32, height: u32) -> Canvas {
    let buf = ImageBuffer::new(width, height);

    Canvas {
      buffer: buf
    }
  }

  pub fn set_pixel(&mut self, x: u32, y: u32, c: Color) {
    self.buffer.put_pixel(x, y, c.to_rgb());
  }

  pub fn get_pixel(&self, x: u32, y: u32) -> Color {
    let pixel = self.buffer.get_pixel(x, y);

    Color {
      r: (pixel[0] as f64 / 255.0),
      g: (pixel[1] as f64 / 255.0),
      b: (pixel[2] as f64 / 255.0),
    }
  }

  pub fn save(&self, path: &str) {
    let base = env::current_dir().unwrap();
    let target = base.join("images").join(path);
    self.buffer.save(target).unwrap();
  }
}

#[cfg(test)]
mod tests {
  use crate::canvas::{Canvas, Color};

  #[test]
  fn can_set_and_get_pixel() {
    let mut c = Canvas::new(10, 10);
    let red = Color { r: 1.0, g: 0.0, b: 0.0 };

    c.set_pixel(5, 5, red);
    let pixel = c.get_pixel(5, 5);
    assert_eq!(pixel, red);
  }

  #[test]
  fn can_add_colors() {
    let c1 = Color { r: 0.9, g: 0.6, b: 0.75 };
    let c2 = Color { r: 0.7, g: 0.1, b: 0.25 };

    assert_eq!(c1 + c2, Color { r: 1.6, g: 0.7, b: 1.0 });
  }

  #[test]
  fn can_subtract_colors() {
    let c1 = Color { r: 0.9, g: 0.6, b: 0.75 };
    let c2 = Color { r: 0.7, g: 0.1, b: 0.25 };

    assert_eq!(c1 - c2, Color { r: 0.2, g: 0.5, b: 0.5 });
  }

  #[test]
  fn can_multiply_colors() {
    let c1 = Color { r: 1.0, g: 0.2, b: 0.4 };
    let c2 = Color { r: 0.9, g: 1.0, b: 0.1 };

    assert_eq!(c1 * c2, Color { r: 0.9, g: 0.2, b: 0.04 });

    let c3 = Color { r: 1.0, g: 2.0, b: 3.0 };

    assert_eq!(c3 * 2.0, Color { r: 2.0, g: 4.0, b: 6.0 });
  }
}
