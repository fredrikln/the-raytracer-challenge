mod point;
mod vector;
mod canvas;
mod matrix;

use point::Point;
use vector::Vector;
use canvas::{Canvas, Color};
use matrix::Matrix;


#[derive(Debug)]
struct Environment {
  gravity: Vector,
  wind: Vector,
}

#[derive(Debug)]
struct Projectile {
  position: Point,
  velocity: Vector,
}

fn main() {
  let width = 900;
  let height = 550;

  let mut canvas = Canvas::new(width, height);
  let red = Color { r: 1.0, g: 0.0, b: 0.0 };

  let velocity = Vector { x: 1.0, y: 1.8, z: 0.0 };

  let env = Environment { gravity: Vector { x: 0.0, y: -0.1, z: 0.0 }, wind: Vector { x: -0.02, y: 0.0, z: 0.0 } };
  let mut proj = Projectile { position: Point { x: 0.0, y: 1.0, z: 0.0 }, velocity: velocity.normalize() * 11.25 };

  while proj.position.y >= 0.0 {
    canvas.set_pixel(proj.position.x as u32, height - (proj.position.y as u32), red);

    tick(&env, &mut proj);
  }

  canvas.save("test.png");
}

fn tick(env: &Environment, proj: &mut Projectile) {
  proj.position = proj.position + proj.velocity;
  proj.velocity = proj.velocity + env.gravity + env.wind;
}
