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
mod pattern;

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
use pattern::{Pattern,StripedPattern,GradientPattern};

use std::time::SystemTime;

fn main() {
  let starttime = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).expect("error");

  let mut sp = StripedPattern::new(Color { r: 1.0, g: 0.0, b: 0.0 }, Color { r: 0.0, g: 0.0, b: 1.0 });
  sp.transform = Matrix::scale_linear(0.125) * Matrix::rotate_z(-std::f64::consts::PI / 4.0) * Matrix::rotate_y(-std::f64::consts::PI / 8.0);
  let pattern = Pattern::Stripe(sp);

  let mut sp2 = GradientPattern::new(Color { r: 1.0, g: 0.0, b: 0.0 }, Color { r: 0.0, g: 1.0, b: 0.0 });
  sp2.transform = Matrix::rotate_z(std::f64::consts::PI / 4.0) * Matrix::translate(1.0, 0.0, 0.0) * Matrix::scale_linear(2.0);
  let pattern2 = Pattern::Gradient(sp2);

  let mut sp3 = StripedPattern::new(Color { r: 1.0, g: 1.0, b: 0.0 }, Color { r: 0.0, g: 1.0, b: 0.0 });
  sp3.transform = Matrix::scale_linear(0.25) * Matrix::rotate_x(std::f64::consts::PI / 4.0);
  let pattern3 = Pattern::Stripe(sp3);

  let mut floor = Plane::new();
  let mut m1 = Material::new();
  m1.color = Color { r: 1.0, g: 0.9, b: 0.9 };
  m1.specular = 0.0;
  m1.reflective = 0.0;
  floor.casts_shadow = false;
  floor.material = m1;

  let mut roof = Plane::new();
  roof.transform = Matrix::translate(0.0, 15.0, 0.0);
  roof.casts_shadow = false;
  roof.material = m1;

  let mut left_wall = Plane::new();
  left_wall.transform = Matrix::translate(-15.0, 0.0, 0.0) * Matrix::rotate_y(-std::f64::consts::PI / 2.0) * Matrix::rotate_x(std::f64::consts::PI / 2.0);
  left_wall.casts_shadow = false;
  left_wall.material = m1;

  let mut right_wall = Plane::new();
  right_wall.transform = Matrix::translate(15.0, 0.0, 0.0) * Matrix::rotate_y(std::f64::consts::PI / 2.0) * Matrix::rotate_x(std::f64::consts::PI / 2.0);
  right_wall.casts_shadow = false;
  right_wall.material = m1;

  let mut far_wall = Plane::new();
  far_wall.transform = Matrix::translate(0.0, 0.0, 15.0) * Matrix::rotate_x(std::f64::consts::PI / 2.0);
  far_wall.casts_shadow = false;
  far_wall.material = m1;

  let mut near_wall = Plane::new();
  near_wall.transform = Matrix::translate(0.0, 0.0, -15.0) * Matrix::rotate_x(std::f64::consts::PI / 2.0);
  near_wall.casts_shadow = false;
  near_wall.material = m1;

  let mut middle = Sphere::new();
  middle.transform = Matrix::translate(-0.5, 1.0, 0.5);
  let mut m2 = Material::new();
  m2.color = Color { r: 0.373, g: 0.404, b: 0.55 };
  m2.ambient = 0.0;
  m2.diffuse = 0.2;
  m2.specular = 1.0;
  m2.shininess = 200.0;
  m2.pattern = None;//Some(pattern);
  m2.reflective = 0.7;
  middle.material = m2;

  let mut right = Sphere::new();
  right.transform = Matrix::translate(1.0, 0.5, -0.5) * Matrix::scale_linear(0.5);
  let mut m3 = Material::new();
  m3.color = Color { r: 0.5, g: 1.0, b: 0.1 };
  m3.diffuse = 0.7;
  m3.specular = 0.3;
  m3.pattern = Some(pattern2);
  right.material = m3;

  let mut left = Sphere::new();
  left.transform = Matrix::translate(-1.5, 0.33, -0.75) * Matrix::scale_linear(0.33);
  let mut m4 = Material::new();
  m4.color = Color { r: 1.0, g: 0.8, b: 0.1 };
  m4.diffuse = 0.7;
  m4.specular = 0.3;
  m4.pattern = Some(pattern3);
  left.material = m4;

  let light = PointLight {
    position: Point { x: -5.0, y: 7.5, z: -5.0 },
    intensity: Color { r: 1.0, g: 1.0, b: 1.0 }
  };

  let world = World {
    objects: vec![Object::Plane(floor), Object::Plane(roof), Object::Plane(left_wall), Object::Plane(right_wall), Object::Plane(far_wall), Object::Plane(near_wall), Object::Sphere(middle), Object::Sphere(right), Object::Sphere(left)],
    lights: vec![light]
  };

  let width: u32 = 500;
  let height = (width as f64 / 1.77777777777777778) as u32;

  let mut camera = Camera::new(width, height, std::f64::consts::PI / 3.0);
  camera.antialias = false;
  camera.transform = Camera::view_transform(
    Point { x: 2.0, y: 1.5, z: -5.0 },
    Point { x: 0.0, y: 1.0, z: 0.0 },
    Vector { x: 0.0, y: 1.0, z: 0.0 }
  );

  let canvas = camera.render(world);

  let endtime = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).expect("error");

  println!("{}x{} = {} pixels", width, height, width * height);
  println!("Render took {:.3} seconds", (endtime - starttime).as_millis() as f64 / 1000.0);
  println!("Average {:.3} microseconds per pixel", (endtime - starttime).as_micros() as f64 / (width*height) as f64);

  let filetime = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).expect("error").as_secs();
  let filename: &str = &format!("images/image-{}-{}x{}.png", filetime, width, height);
  canvas.save(filename);
}
