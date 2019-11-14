mod point;
mod vector;
mod canvas;
mod matrix;
mod utils;

use point::Point;
use vector::Vector;
use canvas::{Canvas, Color};
use matrix::Matrix;

use uuid::Uuid;

fn main() {
  let width = 500;
  let height = 500;

  let mut canvas = Canvas::new(width, height);
  let red = Color { r: 1.0, g: 0.0, b: 0.0 };

  let mut point = Point { x: 0.0, y: 0.0, z: 0.0 };
  point = point * Matrix::translate(-150.0, 0.0, 0.0);

  for i in 0..12 {
    let transform = Matrix::rotate_z(i as f32 * (std::f32::consts::PI / 6.0))  ;
    let new_point = transform * point;

    let x: u32 = (new_point.x + width as f32 / 2.0) as u32;
    let y: u32 = (new_point.y + height as f32 / 2.0) as u32;

    canvas.set_pixel(x, y, red);
  }

  let filename: &str = &format!("image-{}.png", Uuid::new_v4());
  canvas.save(filename);
}
