use std::ops::{Add, Sub, Neg, Mul, Div};
use crate::utils::equal;

#[derive(Debug, Copy, Clone)]
pub struct Vector {
  pub x: f64,
  pub y: f64,
  pub z: f64,
}

impl Vector {
  pub fn magnitude(&self) -> f64 {
    (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
  }

  pub fn normalize(&self) -> Vector {
    let magnitude = self.magnitude();
    Vector {
      x: self.x / magnitude,
      y: self.y / magnitude,
      z: self.z / magnitude,
    }
  }

  pub fn dot(&self, other: &Self) -> f64 {
    self.x * other.x + self.y * other.y + self.z * other.z
  }

  pub fn cross(&self, other: &Self) -> Vector {
    Vector {
      x: self.y * other.z - self.z * other.y,
      y: self.z * other.x - self.x * other.z,
      z: self.x * other.y - self.y * other.x,
    }
  }

  pub fn reflect(&self, normal: Vector) -> Vector {
    *self - normal * 2.0 * self.dot(&normal)
  }
}

impl PartialEq for Vector {
  fn eq(&self, other: &Self) -> bool {
    equal(self.x, other.x) && equal(self.y, other.y) && equal(self.z, other.z)
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

impl Neg for Vector {
  type Output = Vector;

  fn neg(self) -> Vector {
    Vector {
      x: -self.x,
      y: -self.y,
      z: -self.z,
    }
  }
}

impl Mul<f64> for Vector {
  type Output = Vector;

  fn mul(self, rhs: f64) -> Vector {
    Vector {
      x: self.x * rhs,
      y: self.y * rhs,
      z: self.z * rhs,
    }
  }
}

impl Mul<Vector> for Vector {
  type Output = Vector;

  fn mul(self, other: Self) -> Vector {
    Vector {
      x: self.x * other.x,
      y: self.y * other.y,
      z: self.z * other.z,
    }
  }
}

impl Div<f64> for Vector {
  type Output = Vector;

  fn div(self, rhs: f64) -> Vector {
    Vector {
      x: self.x / rhs,
      y: self.y / rhs,
      z: self.z / rhs,
    }
  }
}

#[cfg(test)]
mod tests {
  use crate::vector::Vector;
  use crate::utils::equal;

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

  #[test]
  fn can_be_negated() {
    let v1 = Vector { x: 1.0, y: 2.0, z: 3.0 };
    assert_eq!(-v1, Vector { x: -1.0, y: -2.0, z: -3.0 });
  }

  #[test]
  fn can_be_multiplied() {
    let v1 = Vector { x: 1.0, y: 2.0, z: 3.0 };
    assert_eq!(v1 * 3.0, Vector { x: 3.0, y: 6.0, z: 9.0 });

    let v2 = Vector { x: 1.0, y: 2.0, z: 3.0 };
    assert_eq!(v2 * 0.5, Vector { x: 0.5, y: 1.0, z: 1.5 });
  }

  #[test]
  fn can_be_divided() {
    let v1 = Vector { x: 3.0, y: 6.0, z: 9.0 };
    assert_eq!(v1 / 3.0, Vector { x: 1.0, y: 2.0, z: 3.0 });

    let v2 = Vector { x: 0.5, y: 1.0, z: 1.5 };
    assert_eq!(v2 / 0.5, Vector { x: 1.0, y: 2.0, z: 3.0 });
  }

  #[test]
  fn can_get_magnitude() {
    let v1 = Vector { x: 1.0, y: 0.0, z: 0.0 };
    assert_eq!(v1.magnitude(), 1.0);

    let v2 = Vector { x: 0.0, y: 1.0, z: 0.0 };
    assert_eq!(v2.magnitude(), 1.0);

    let v3 = Vector { x: 0.0, y: 0.0, z: 1.0 };
    assert_eq!(v3.magnitude(), 1.0);

    let v4 = Vector { x: 1.0, y: 2.0, z: 3.0 };
    let result: f64 = 14.0;
    assert_eq!(v4.magnitude(), result.sqrt());

    let v5 = Vector { x: -1.0, y: -2.0, z: -3.0 };
    let result2: f64 = 14.0;
    assert_eq!(v5.magnitude(), result2.sqrt());
  }

  #[test]
  fn can_be_normalized() {
    let v1 = Vector { x: 4.0, y: 0.0, z: 0.0 };
    assert_eq!(v1.normalize(), Vector { x: 1.0, y: 0.0, z: 0.0 });

    let v2 = Vector { x: 1.0, y: 2.0, z: 3.0 };
    assert_eq!(v2.normalize(), Vector { x: 0.26726, y: 0.53452, z: 0.80178 });

    let v3 = Vector { x: 1.0, y: 2.0, z: 3.0 };
    let result: f64 = v3.normalize().magnitude();

    assert!(equal(result, 1.0));
  }

  #[test]
  fn computes_dot_product() {
    let v1 = Vector { x: 1.0, y: 2.0, z: 3.0 };
    let v2 = Vector { x: 2.0, y: 3.0, z: 4.0 };

    assert_eq!(v1.dot(&v2), 20.0);
    assert_eq!(v2.dot(&v1), 20.0);
  }

  #[test]
  fn computes_cross_product() {
    let v1 = Vector { x: 1.0, y: 2.0, z: 3.0 };
    let v2 = Vector { x: 2.0, y: 3.0, z: 4.0 };

    assert_eq!(v1.cross(&v2), Vector { x: -1.0, y: 2.0, z: -1.0 });
    assert_eq!(v2.cross(&v1), Vector { x: 1.0, y: -2.0, z: 1.0 });

    let v3 = Vector { x: 1.0, y: 0.0, z: 0.0 };
    let v4 = Vector { x: 0.0, y: 1.0, z: 0.0 };

    assert_eq!(v3.cross(&v4), Vector { x: 0.0, y: 0.0, z: 1.0 });
  }

  #[test]
  fn reflect_a_vector_approaching_at_45_degrees() {
    let v = Vector { x: 1.0, y: -1.0, z: 0.0 };
    let n = Vector { x: 0.0, y: 1.0, z: 0.0 };

    let r = v.reflect(n);

    assert_eq!(r, Vector { x: 1.0, y: 1.0, z: 0.0 })
  }

  #[test]
  fn reflect_a_vector_on_slanted_surface() {
    let v = Vector { x: 0.0, y: -1.0, z: 0.0 };
    let n = Vector { x: (2.0 as f64).sqrt() / 2.0, y: (2.0 as f64).sqrt() / 2.0, z: 0.0 };

    let r = v.reflect(n);

    assert_eq!(r, Vector { x: 1.0, y: 0.0, z: 0.0 })
  }
}
