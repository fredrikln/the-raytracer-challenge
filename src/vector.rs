use std::ops::{Add, Sub};

#[derive(Debug)]
pub struct Vector {
  pub x: f32,
  pub y: f32,
  pub z: f32,
}

impl PartialEq for Vector {
  fn eq(&self, other: &Self) -> bool {
    let epsilon = std::f32::EPSILON;

    ((self.x-other.x).abs() < epsilon) && ((self.y-other.y).abs() < epsilon) && ((self.z-other.z).abs() < epsilon)
  }
}

impl Add<Vector> for Vector {
  type Output = Vector;

  fn add(self, other: Self) -> Vector {
    Vector {
      x: self.x + other.x,
      y: self.y + other.y,
      z: self.z + other.z,
    }
  }
}

impl Sub<Vector> for Vector {
  type Output = Vector;

  fn sub(self, other: Self) -> Vector {
    Vector {
      x: self.x - other.x,
      y: self.y - other.y,
      z: self.z - other.z,
    }
  }
}

#[cfg(test)]
mod tests {
  use crate::vector::Vector;

  #[test]
  fn can_be_instanciated() {
    let v = Vector { x: 1.0, y: 2.0, z: 3.0 };

    assert_eq!(v.x, 1.0);
    assert_eq!(v.y, 2.0);
    assert_eq!(v.z, 3.0);
  }

  #[test]
  fn can_be_compared() {
    let v1 = Vector { x: 1.0, y: 2.0, z: 3.0 };
    let v2 = Vector { x: 1.0, y: 2.0, z: 3.0 };

    assert_eq!(v1, v2);

    let v3 = Vector { x: 3.0, y: 2.0, z: 3.0 };
    let v4 = Vector { x: 1.0, y: 2.0, z: 3.0 };

    assert_ne!(v3, v4);

    let v5 = Vector { x: 0.15 + 0.15 + 0.15, y: 1.0, z: 1.0 };
    let v6 = Vector { x: 0.1 + 0.1 + 0.25, y: 1.0, z: 1.0 };

    assert_eq!(v5, v6);
  }

  #[test]
  fn can_be_added() {
    let v1 = Vector { x: 1.0, y: 2.0, z: 3.0 };
    let v2 = Vector { x: 1.0, y: 2.0, z: 3.0 };

    assert_eq!(v1 + v2, Vector { x: 2.0, y: 4.0, z: 6.0 });
  }

  #[test]
  fn can_be_subtracted() {
    let v1 = Vector { x: 1.0, y: 2.0, z: 3.0 };
    let v2 = Vector { x: 1.0, y: 2.0, z: 3.0 };

    assert_eq!(v1 - v2, Vector { x: 0.0, y: 0.0, z: 0.0 });
  }
}
