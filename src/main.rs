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
mod world;
mod camera;
mod object;
mod plane;

use point::Point;
use vector::Vector;
use canvas::Color;
use matrix::Matrix;
use material::Material;
use point_light::PointLight;
use camera::Camera;
use world::World;
use sphere::Sphere;
use plane::Plane;
use object::{Object};

use std::time::SystemTime;

fn main() {
  let mut floor = Plane::new();
  let mut m1 = Material::new();
  m1.color = Color { r: 1.0, g: 0.9, b: 0.9 };
  m1.specular = 0.0;
  floor.material = m1;

  let mut left_wall = Plane::new();
  left_wall.transform = Matrix::translate(0.0, 0.0, 5.0) * Matrix::rotate_y(-std::f64::consts::PI / 4.0) * Matrix::rotate_x(std::f64::consts::PI / 2.0);
  left_wall.material = m1;

  let mut right_wall = Plane::new();
  right_wall.transform = Matrix::translate(0.0, 0.0, 5.0) * Matrix::rotate_y(std::f64::consts::PI / 4.0) * Matrix::rotate_x(std::f64::consts::PI / 2.0);
  right_wall.material = m1;

  let mut middle = Sphere::new();
  middle.transform = Matrix::translate(-0.5, 1.0, 0.5);
  let mut m2 = Material::new();
  m2.color = Color { r: 0.1, g: 1.0, b: 0.5 };
  m2.diffuse = 0.7;
  m2.specular = 0.3;
  middle.material = m2;

  let mut right = Sphere::new();
  right.transform = Matrix::translate(1.0, 0.5, -0.5) * Matrix::scale_linear(0.5);
  let mut m3 = Material::new();
  m3.color = Color { r: 0.5, g: 1.0, b: 0.1 };
  m3.diffuse = 0.7;
  m3.specular = 0.3;
  right.material = m3;

  let mut left = Sphere::new();
  left.transform = Matrix::translate(-1.5, 0.33, -0.75) * Matrix::scale_linear(0.33);
  let mut m4 = Material::new();
  m4.color = Color { r: 1.0, g: 0.8, b: 0.1 };
  m4.diffuse = 0.7;
  m4.specular = 0.3;
  left.material = m4;

  let light = PointLight {
    position: Point { x: -10.0, y: 10.0, z: -10.0 },
    intensity: Color { r: 1.0, g: 1.0, b: 1.0 }
  };

  let world = World {
    objects: vec![Object::Plane(floor), Object::Plane(left_wall), Object::Plane(right_wall), Object::Sphere(middle), Object::Sphere(right), Object::Sphere(left)],
    lights: vec![light]
  };

  let width: u32 = 500;
  let height = width / 2;

  let mut camera = Camera::new(width, height, std::f64::consts::PI / 3.0);
  // camera.antialias = true;
  camera.transform = Camera::view_transform(
    Point { x: 0.0, y: 1.5, z: -5.0 },
    Point { x: 0.0, y: 1.0, z: 0.0 },
    Vector { x: 0.0, y: 1.0, z: 0.0 }
  );

  let canvas = camera.render(world);

  let time = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).expect("error").as_secs();
  let filename: &str = &format!("images/image-{}.png", time);
  canvas.save(filename);
}
