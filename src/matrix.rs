use std::ops::{Mul, Index, IndexMut};
use float_cmp::approx_eq;
use crate::point::Point;
use crate::vector::Vector;

#[derive(Debug, Copy, Clone)]
pub struct Matrix {
  data: [[f32; 4]; 4],
}

impl Matrix {
  pub fn new() -> Matrix {
    Matrix {
      data: [
        [0.0, 0.0, 0.0, 0.0],
        [0.0, 0.0, 0.0, 0.0],
        [0.0, 0.0, 0.0, 0.0],
        [0.0, 0.0, 0.0, 0.0],
      ]
    }
  }

  pub fn identity() -> Matrix {
    Matrix {
      data: [
        [1.0, 0.0, 0.0, 0.0],
        [0.0, 1.0, 0.0, 0.0],
        [0.0, 0.0, 1.0, 0.0],
        [0.0, 0.0, 0.0, 1.0]
      ]
    }
  }

  pub fn transpose(&self) -> Matrix {
    let mut m = Matrix::new();

    for row in 0..4 {
      for col in 0..4 {
        m[row][col] = self[col][row];
      }
    }

    m
  }
}

impl PartialEq for Matrix {
  fn eq(&self, other: &Self) -> bool {
    for i in 0..4 {
      for j in 0..4 {
        if !approx_eq!(f32, self[i][j], other[i][j]) {
          return false;
        }
      }
    }

    true
  }
}

impl Mul<Matrix> for Matrix {
  type Output = Matrix;

  fn mul(self, other: Self) -> Matrix {
    let mut m = Matrix::new();

    for row in 0..4 {
      for col in 0..4 {
        m[row][col] = self[row][0] * other[0][col]
          + self[row][1] * other[1][col]
          + self[row][2] * other[2][col]
          + self[row][3] * other[3][col];
      }
    }

    m
  }
}

impl Mul<Point> for Matrix {
  type Output = Point;

  fn mul(self, other: Point) -> Point {
    Point {
      x: self[0][0] * other.x + self[0][1] * other.y + self[0][2] * other.z + self[0][3],
      y: self[1][0] * other.x + self[1][1] * other.y + self[1][2] * other.z + self[1][3],
      z: self[2][0] * other.x + self[2][1] * other.y + self[2][2] * other.z + self[2][3],
    }
  }
}

impl Mul<Matrix> for Point {
  type Output = Point;

  fn mul(self, other: Matrix) -> Point {
    other * self
  }
}

impl Mul<Vector> for Matrix {
  type Output = Vector;

  fn mul(self, other: Vector) -> Vector {
    Vector {
      x: self[0][0] * other.x + self[0][1] * other.y + self[0][2] * other.z,
      y: self[1][0] * other.x + self[1][1] * other.y + self[1][2] * other.z,
      z: self[2][0] * other.x + self[2][1] * other.y + self[2][2] * other.z,
    }
  }
}

impl Mul<Matrix> for Vector {
  type Output = Vector;

  fn mul(self, other: Matrix) -> Vector {
    other * self
  }
}

impl Index<usize> for Matrix {
  type Output = [f32; 4];

  fn index(&self, idx: usize) -> &[f32; 4] {
    &self.data[idx]
  }
}

impl IndexMut<usize> for Matrix {
  fn index_mut(&mut self, idx: usize) -> &mut [f32; 4] {
    &mut self.data[idx]
  }
}

#[cfg(test)]
mod tests {
  use crate::matrix::Matrix;
  use crate::point::Point;
  use crate::vector::Vector;

  #[test]
  fn can_create_new_matrix() {
    let m = Matrix {
      data: [
        [1.0, 2.0, 3.0, 4.0],
        [5.5, 6.5, 7.5, 8.5],
        [9.0, 10.0, 11.0, 12.0],
        [13.5, 14.5, 15.5, 16.5],
      ]
    };

    assert_eq!(m[0][0], 1.0);
    assert_eq!(m[0][3], 4.0);
    assert_eq!(m[1][0], 5.5);
    assert_eq!(m[1][2], 7.5);
    assert_eq!(m[2][2], 11.0);
    assert_eq!(m[3][0], 13.5);
    assert_eq!(m[3][2], 15.5);
  }

  #[test]
  fn can_compare_matrices() {
    let m1 = Matrix {
      data: [
        [1.0, 2.0, 3.0, 4.0],
        [5.5, 6.5, 7.5, 8.5],
        [9.0, 10.0, 11.0, 12.0],
        [13.5, 14.5, 15.5, 16.5],
      ]
    };

    let m2 = Matrix {
      data: [
        [1.0, 2.0, 3.0, 4.0],
        [5.5, 6.5, 7.5, 8.5],
        [9.0, 10.0, 11.0, 12.0],
        [13.5, 14.5, 15.5, 16.5],
      ]
    };

    let m3 = Matrix {
      data: [
        [1.0, 5.0, 3.0, 4.0],
        [5.5, 6.5, 7.5, 8.5],
        [9.0, 10.0, 5.0, 12.0],
        [13.5, 5.5, 15.5, 16.5],
      ]
    };

    assert_eq!(m1, m2);
    assert_ne!(m1, m3);
    assert_ne!(m2, m3);
  }

  #[test]
  fn can_multiply_matrices() {
    let m1 = Matrix {
      data: [
        [1.0, 2.0, 3.0, 4.0],
        [5.0, 6.0, 7.0, 8.0],
        [9.0, 8.0, 7.0, 6.0],
        [5.0, 4.0, 3.0, 2.0],
      ]
    };

    let m2 = Matrix {
      data: [
        [-2.0, 1.0, 2.0, 3.0],
        [3.0, 2.0, 1.0, -1.0],
        [4.0, 3.0, 6.0, 5.0],
        [1.0, 2.0, 7.0, 8.0],
      ]
    };

    let result = Matrix {
      data: [
        [20.0, 22.0, 50.0, 48.0],
        [44.0, 54.0, 114.0, 108.0],
        [40.0, 58.0, 110.0, 102.0],
        [16.0, 26.0, 46.0, 42.0],
      ]
    };

    assert_eq!(m1 * m2, result);
  }

  #[test]
  fn can_multiply_with_point() {
    let p = Point {
      x: 1.0,
      y: 2.0,
      z: 3.0
    };

    let m = Matrix {
      data: [
        [1.0, 2.0, 3.0, 4.0],
        [2.0, 4.0, 4.0, 2.0],
        [8.0, 6.0, 4.0, 1.0],
        [0.0, 0.0, 0.0, 1.0],
      ]
    };

    assert_eq!(p * m, Point { x: 18.0, y: 24.0, z: 33.0 });
    assert_eq!(m * p, Point { x: 18.0, y: 24.0, z: 33.0 });
  }

  #[test]
  fn can_multiply_with_vector() {
    let v = Vector {
      x: 1.0,
      y: 2.0,
      z: 3.0
    };

    let m = Matrix {
      data: [
        [1.0, 2.0, 3.0, 4.0],
        [2.0, 4.0, 4.0, 2.0],
        [8.0, 6.0, 4.0, 1.0],
        [0.0, 0.0, 0.0, 1.0],
      ]
    };

    assert_eq!(v * m, Vector { x: 14.0, y: 22.0, z: 32.0 });
    assert_eq!(m * v, Vector { x: 14.0, y: 22.0, z: 32.0 });
  }

  #[test]
  fn multiplying_by_identy_returns_itself() {
    let m = Matrix {
      data: [
        [1.0, 2.0, 3.0, 4.0],
        [2.0, 4.0, 4.0, 2.0],
        [8.0, 6.0, 4.0, 1.0],
        [0.0, 0.0, 0.0, 1.0],
      ]
    };

    let id = Matrix::identity();

    assert_eq!(m * id, m);
  }

  #[test]
  fn can_transpose_matrix() {
    let m1 = Matrix {
      data: [
        [1.0, 2.0, 3.0, 4.0],
        [2.0, 4.0, 4.0, 2.0],
        [8.0, 6.0, 4.0, 1.0],
        [0.0, 0.0, 0.0, 1.0],
      ]
    };

    let m2 = Matrix {
      data: [
        [1.0, 2.0, 8.0, 0.0],
        [2.0, 4.0, 6.0, 0.0],
        [3.0, 4.0, 4.0, 0.0],
        [4.0, 2.0, 1.0, 1.0],
      ]
    };

    assert_eq!(m1.transpose(), m2);
  }
}
