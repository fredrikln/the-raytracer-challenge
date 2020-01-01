mod point;
mod vector;
mod canvas;
mod matrix;
mod ray;
mod utils;
mod sphere;
mod intersection;

use point::Point;
// use vector::Vector;
use canvas::{Canvas, Color};
use matrix::Matrix;
use ray::Ray;
use sphere::Sphere;
use intersection::Intersection;

use std::time::SystemTime;

fn main() {
  let canvas_size = 200;

  let mut canvas = Canvas::new(canvas_size, canvas_size);
  let red = Color { r: 1.0, g: 0.0, b: 0.0 };

  let ray_origin = Point { x: 0.0, y: 0.0, z: -5.0 };
  let wall_z = 10.0;

  let wall_size = 7.0;

  let pixel_size = wall_size / canvas_size as f32;
  let half = wall_size / 2.0;

  let mut shape = Sphere::new();
  let transform = Matrix::shear(1.0, 0.0, 0.0, 0.0, 0.0, 0.0) * Matrix::scale(0.5, 1.0, 1.0);
  shape.set_transform(transform);

  for y in 0..canvas_size {
    let world_y = half - pixel_size * y as f32;

    for x in 0..canvas_size {
      let world_x = -half + pixel_size * x as f32;

      let position = Point { x: world_x, y: world_y, z: wall_z };

      let ray = Ray { origin: ray_origin, direction: (position - ray_origin).normalize() };

      let intersections = shape.intersect(ray);

      if Intersection::hit(intersections).is_some() {
        canvas.set_pixel(x, y, red);
      }
    }
  }

  let time = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).expect("error").as_secs();
  let filename: &str = &format!("images/image-{}.png", time);
  canvas.save(filename);
}
