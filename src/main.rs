mod point;
mod vector;
mod canvas;
mod matrix;
mod ray;
mod utils;
mod sphere;
mod intersection;
mod point_light;
mod material;

use point::Point;
// use vector::Vector;
use canvas::{Canvas, Color};
use matrix::Matrix;
use ray::Ray;
use sphere::Sphere;
use intersection::Intersection;
use material::Material;
use point_light::PointLight;

use std::time::SystemTime;

fn main() {
  let canvas_size = 200;

  let mut canvas = Canvas::new(canvas_size, canvas_size);

  let ray_origin = Point { x: 0.0, y: 0.0, z: -5.0 };
  let wall_z = 10.0;

  let wall_size = 7.0;

  let pixel_size = wall_size / canvas_size as f32;
  let half = wall_size / 2.0;

  let mut shape = Sphere::new();
  shape.material.color = Color { r: 1.0, g: 0.2, b: 1.0 };

  // let transform = Matrix::shear(0.5, 0.0, 0.0, 0.0, 0.0, 0.0) * Matrix::scale(0.5, 1.0, 1.0);
  // let transform = Matrix::scale(1.0, 0.5, 1.0);
  // shape.set_transform(transform);

  let light_position = Point { x: -10.0, y: 10.0, z: -10.0 };
  let light_color = Color { r: 1.0, g: 1.0, b: 1.0 };
  let light = PointLight { position: light_position, intensity: light_color };

  for y in 0..canvas_size {
    let world_y = half - pixel_size * y as f32;

    for x in 0..canvas_size {
      let world_x = -half + pixel_size * x as f32;

      let position = Point { x: world_x, y: world_y, z: wall_z };

      let ray = Ray { origin: ray_origin, direction: (position - ray_origin).normalize() };

      let intersections = shape.intersect(ray);

      let hit = Intersection::hit(intersections);
      if hit.is_some() {
        let unwrapped_hit = hit.unwrap();

        let point = ray.position(unwrapped_hit.time);
        let normal = unwrapped_hit.object.normal(point);
        let eye_vector = -ray.direction;
        let material = unwrapped_hit.object.material;

        let color = material.lighting(light, point, eye_vector, normal);

        canvas.set_pixel(x, y, color);
      }
    }
  }

  let time = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).expect("error").as_secs();
  let filename: &str = &format!("images/image-{}.png", time);
  canvas.save(filename);
}
