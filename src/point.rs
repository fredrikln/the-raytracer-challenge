use std::ops::{Add, Sub};
use crate::vector::Vector;

#[derive(Debug)]
pub struct Point {
  pub x: f32,
  pub y: f32,
  pub z: f32,
}

impl PartialEq for Point {
  fn eq(&self, other: &Self) -> bool {
    let epsilon = std::f32::EPSILON;

    ((self.x-other.x).abs() < epsilon) && ((self.y-other.y).abs() < epsilon) && ((self.z-other.z).abs() < epsilon)
  }
}

impl Add<Vector> for Point {
  type Output = Point;

  fn add(self, other: Vector) -> Point {
    Point {
      x: self.x + other.x,
      y: self.y + other.y,
      z: self.z + other.z,
    }
  }
}

impl Sub<Point> for Point {
  type Output = Vector;

  fn sub(self, other: Self) -> Vector {
    Vector {
      x: self.x - other.x,
      y: self.y - other.y,
      z: self.z - other.z,
    }
  }
}

impl Sub<Vector> for Point {
  type Output = Point;

  fn sub(self, other: Vector) -> Point {
    Point {
      x: self.x - other.x,
      y: self.y - other.y,
      z: self.z - other.z,
    }
  }
}

#[cfg(test)]
mod tests {
  use crate::point::Point;
  use crate::vector::Vector;

  #[test]
  fn can_be_instanciated() {
    let p: Point = Point { x: 1.0, y: 2.0, z: 3.0 };

    assert_eq!(p.x, 1.0);
    assert_eq!(p.y, 2.0);
    assert_eq!(p.z, 3.0);
  }

  #[test]
  fn can_be_compared() {
    let p1 = Point { x: 1.0, y: 2.0, z: 3.0 };
    let p2 = Point { x: 1.0, y: 2.0, z: 3.0 };

    assert_eq!(p1, p2);

    let p3 = Point { x: 3.0, y: 2.0, z: 3.0 };
    let p4 = Point { x: 1.0, y: 2.0, z: 3.0 };

    assert_ne!(p3, p4);

    let p5 = Point { x: 0.15 + 0.15 + 0.15, y: 1.0, z: 1.0 };
    let p6 = Point { x: 0.1 + 0.1 + 0.25, y: 1.0, z: 1.0 };

    assert_eq!(p5, p6);
  }

  #[test]
  fn can_be_added() {
    let p1 = Point { x: 1.0, y: 2.0, z: 3.0 };
    let v1 = Vector { x: 1.0, y: 2.0, z: 3.0 };

    assert_eq!(p1 + v1, Point { x: 2.0, y: 4.0, z: 6.0 });
  }

  #[test]
  fn can_be_subtracted() {
    let p1 = Point { x: 1.0, y: 2.0, z: 3.0 };
    let p2 = Point { x: 1.0, y: 2.0, z: 3.0 };

    assert_eq!(p1 - p2, Vector { x: 0.0, y: 0.0, z: 0.0 });

    let p3 = Point { x: 1.0, y: 2.0, z: 3.0 };
    let v1 = Vector { x: 1.0, y: 2.0, z: 3.0 };

    assert_eq!(p3 - v1, Point { x: 0.0, y: 0.0, z: 0.0 });
  }
}
