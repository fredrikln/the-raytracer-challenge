mod point;
mod vector;

use point::Point;
use vector::Vector;

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

  let env = Environment { gravity: Vector { x: 0.0, y: -0.1, z: 0.0 }, wind: Vector { x: -0.01, y: 0.0, z: 0.0 } };
  let mut proj = Projectile { position: Point { x: 0.0, y: 0.0, z: 0.0 }, velocity: Vector { x: 1.0, y: 1.0, z: 0.0 } };

  let mut i = 0;
  while proj.position.y >= 0.0 {
    i += 1;
    tick(&env, &mut proj);

    println!("{:?}: {:?}", i, proj.position);
  }

}

fn tick(env: &Environment, proj: &mut Projectile) {
  proj.position = proj.position + proj.velocity;
  proj.velocity = proj.velocity + env.gravity + env.wind;
}
