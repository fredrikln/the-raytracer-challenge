use crate::point::Point;
use crate::vector::Vector;
use crate::matrix::Matrix;
use std::ops::{Mul};

#[derive(Debug, Copy, Clone)]
pub struct Ray {
  pub origin: Point,
  pub direction: Vector
}

impl Ray {
  pub fn position(&self, time: f32) -> Point {
    Point {
      x: self.origin.x + (self.direction.x * time),
      y: self.origin.y + (self.direction.y * time),
      z: self.origin.z + (self.direction.z * time),
    }
  }
}

impl Mul<Matrix> for Ray {
  type Output = Ray;

  fn mul(self, rhs: Matrix) -> Ray {
    Ray {
      origin: self.origin * rhs,
      direction: self.direction * rhs
    }
  }
}

impl Mul<Ray> for Matrix {
  type Output = Ray;

  fn mul(self, rhs: Ray) -> Ray {
    rhs * self
  }
}

#[cfg(test)]
mod tests {
  use crate::ray::Ray;
  use crate::vector::Vector;
  use crate::point::Point;
  use crate::sphere::Sphere;
  use crate::matrix::Matrix;

  #[test]
  fn can_create_new_rays() {
    let origin = Point { x: 1.0, y: 2.0, z: 3.0 };
    let direction = Vector { x: 4.0, y: 5.0, z: 6.0 };

    let ray = Ray { origin, direction };

    assert_eq!(ray.origin, origin);
    assert_eq!(ray.direction, direction);
  }

  #[test]
  fn can_compute_point_from_distance() {
    let ray = Ray { origin: Point { x: 2.0, y: 3.0, z: 4.0 }, direction: Vector { x: 1.0, y: 0.0, z: 0.0 } };

    assert_eq!(ray.position(0.0), Point { x: 2.0, y: 3.0, z: 4.0 });
    assert_eq!(ray.position(1.0), Point { x: 3.0, y: 3.0, z: 4.0 });
    assert_eq!(ray.position(-1.0), Point { x: 1.0, y: 3.0, z: 4.0 });
    assert_eq!(ray.position(2.5), Point { x: 4.5, y: 3.0, z: 4.0 });
  }

  #[test]
  fn translating_a_ray() {
    let ray = Ray { origin: Point { x: 1.0, y: 2.0, z: 3.0 }, direction: Vector { x: 0.0, y: 1.0, z: 0.0 } };
    let transformation = Matrix::translate(3.0, 4.0, 5.0);

    let translated_ray = transformation * ray;

    assert_eq!(translated_ray.origin, Point { x: 4.0, y: 6.0, z: 8.0 });
    assert_eq!(translated_ray.direction, Vector { x: 0.0, y: 1.0, z: 0.0 });
  }

  #[test]
  fn scaling_a_ray() {
    let ray = Ray { origin: Point { x: 1.0, y: 2.0, z: 3.0 }, direction: Vector { x: 0.0, y: 1.0, z: 0.0 } };
    let transformation = Matrix::scale(2.0, 3.0, 4.0);

    let scaled_ray = transformation * ray;

    assert_eq!(scaled_ray.origin, Point { x: 2.0, y: 6.0, z: 12.0 });
    assert_eq!(scaled_ray.direction, Vector { x: 0.0, y: 3.0, z: 0.0 });
  }
}
