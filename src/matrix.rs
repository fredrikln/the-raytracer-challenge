use std::ops::{Mul, Index, IndexMut};
use crate::point::Point;
use crate::vector::Vector;
use crate::utils::equal;

#[derive(Debug, Copy, Clone)]
pub struct Matrix {
  pub data: [[f64; 4]; 4],
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

  pub fn cofactor(&self, row: usize, col: usize) -> f64 {
    cofactor4(self.data, row, col)
  }

  pub fn determinant(&self) -> f64 {
    determinant4(self.data)
  }

  pub fn invertible(&self) -> bool {
    invertible(self.data)
  }

  pub fn inverse(&self) -> Option<Matrix> {
    if !self.invertible() { return None }

    Some(Matrix { data: inverse(self.data).unwrap() })
  }

  pub fn translate(x: f64, y: f64, z: f64) -> Matrix {
    Matrix {
      data: [
        [1.0, 0.0, 0.0,   x],
        [0.0, 1.0, 0.0,   y],
        [0.0, 0.0, 1.0,   z],
        [0.0, 0.0, 0.0, 1.0],
      ]
    }
  }

  pub fn scale(x: f64, y: f64, z: f64) -> Matrix {
    Matrix {
      data: [
        [  x, 0.0, 0.0, 0.0],
        [0.0,   y, 0.0, 0.0],
        [0.0, 0.0,   z, 0.0],
        [0.0, 0.0, 0.0, 1.0],
      ]
    }
  }

  pub fn scale_linear(i: f64) -> Matrix {
    Matrix::scale(i, i, i)
  }

  pub fn rotate_x(r: f64) -> Matrix {
    Matrix {
      data: [
        [1.0,     0.0,      0.0, 0.0],
        [0.0, r.cos(), -r.sin(), 0.0],
        [0.0, r.sin(),  r.cos(), 0.0],
        [0.0,     0.0,      0.0, 1.0]
      ]
    }
  }

  pub fn rotate_y(r: f64) -> Matrix {
    Matrix {
      data: [
        [ r.cos(), 0.0, r.sin(), 0.0],
        [    0.0,  1.0,     0.0, 0.0],
        [-r.sin(), 0.0, r.cos(), 0.0],
        [     0.0, 0.0,     0.0, 1.0]
      ]
    }
  }

  pub fn rotate_z(r: f64) -> Matrix {
    Matrix {
      data: [
        [ r.cos(), -r.sin(), 0.0, 0.0],
        [ r.sin(),  r.cos(), 0.0, 0.0],
        [     0.0,      0.0, 1.0, 0.0],
        [     0.0,      0.0, 0.0, 1.0]
      ]
    }
  }

  pub fn shear(xy: f64, xz: f64, yx: f64, yz: f64, zx: f64, zy: f64) -> Matrix {
    Matrix {
      data: [
        [1.0,  xy,  xz, 0.0],
        [ yx, 1.0,  yz, 0.0],
        [ zx,  zy, 1.0, 0.0],
        [0.0, 0.0, 0.0, 1.0]
      ]
    }
  }
}

impl PartialEq for Matrix {
  fn eq(&self, other: &Self) -> bool {
    for i in 0..4 {
      for j in 0..4 {
        if !equal(self[i][j], other[i][j]) {
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
  type Output = [f64; 4];

  fn index(&self, idx: usize) -> &[f64; 4] {
    &self.data[idx]
  }
}

impl IndexMut<usize> for Matrix {
  fn index_mut(&mut self, idx: usize) -> &mut [f64; 4] {
    &mut self.data[idx]
  }
}

fn determinant(matrix: [[f64; 2]; 2]) -> f64 {
  return (matrix[0][0] * matrix[1][1]) - (matrix[0][1] * matrix[1][0]);
}

fn submatrix3(matrix: [[f64; 3]; 3], row: usize, col: usize) -> [[f64; 2]; 2] {
  let mut out: [[f64; 2]; 2] = [[0.0; 2]; 2];

  let mut row_counter = 0;
  for i in 0..3 {
    if i == row { continue; }

    let mut col_counter = 0;
    for j in 0..3 {
      if j == col { continue; }

      out[row_counter][col_counter] = matrix[i][j];

      col_counter += 1;
    }

    row_counter += 1;
  }

  return out;
}

fn submatrix4(matrix: [[f64; 4]; 4], row: usize, col: usize) -> [[f64; 3]; 3] {
  let mut out: [[f64; 3]; 3] = [[0.0; 3]; 3];

  let mut row_counter = 0;
  for i in 0..4 {
    if i == row { continue; }

    let mut col_counter = 0;
    for j in 0..4 {
      if j == col { continue; }

      out[row_counter][col_counter] = matrix[i][j];

      col_counter += 1;
    }

    row_counter += 1;
  }

  return out;
}

fn minor3(matrix: [[f64; 3]; 3], row: usize, col: usize) -> f64 {
  determinant(submatrix3(matrix, row, col))
}

fn cofactor3(matrix: [[f64; 3]; 3], row: usize, col: usize) -> f64 {
  let mut minor = minor3(matrix, row, col);

  if (row+col) % 2 == 1 { minor *= -1.0; }

  minor
}

fn determinant3(matrix: [[f64; 3]; 3]) -> f64 {
  let mut out = 0.0;
  for i in 0..3 {
    out += cofactor3(matrix, 0, i) * matrix[0][i];
  }

  out
}

fn minor4(matrix: [[f64; 4]; 4], row: usize, col: usize) -> f64 {
  determinant3(submatrix4(matrix, row, col))
}

fn cofactor4(matrix: [[f64; 4]; 4], row: usize, col: usize) -> f64 {
  let mut minor = minor4(matrix, row, col);

  if (row+col) % 2 == 1 { minor *= -1.0; }

  minor
}

fn determinant4(matrix: [[f64; 4]; 4]) -> f64 {
  let mut out = 0.0;
  for i in 0..4 {
    out += cofactor4(matrix, 0, i) * matrix[0][i];
  }

  out
}

fn invertible(matrix: [[f64; 4]; 4]) -> bool {
  determinant4(matrix) != 0.0
}

fn inverse(matrix: [[f64; 4]; 4]) -> Option<[[f64; 4]; 4]> {
  if !invertible(matrix) { return None; }

  let mut out: [[f64; 4]; 4] = [[0.0; 4]; 4];
  let determinant = determinant4(matrix);

  for i in 0..4 {
    for j in 0..4 {
      out[i][j] = cofactor4(matrix, i, j);
    }
  }

  out = transpose(out);

  for i in 0..4 {
    for j in 0..4 {
      out[i][j] = out[i][j] / determinant;
    }
  }

  Some(out)
}

fn transpose(matrix: [[f64; 4]; 4]) -> [[f64; 4]; 4] {
  let mut out: [[f64; 4]; 4] = [[0.0; 4]; 4];

  for row in 0..4 {
    for col in 0..4 {
      out[row][col] = matrix[col][row];
    }
  }

  out
}

#[cfg(test)]
mod tests {
  use crate::matrix::Matrix;
  use crate::point::Point;
  use crate::vector::Vector;
  use crate::utils::equal;

  #[test]
  fn can_create_new_matrix() {
    let m = Matrix {
      data: [
        [ 1.0,  2.0,  3.0,  4.0],
        [ 5.5,  6.5,  7.5,  8.5],
        [ 9.0, 10.0, 11.0, 12.0],
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
        [ 1.0,  2.0,  3.0,  4.0],
        [ 5.5,  6.5,  7.5,  8.5],
        [ 9.0, 10.0, 11.0, 12.0],
        [13.5, 14.5, 15.5, 16.5],
      ]
    };

    let m2 = Matrix {
      data: [
        [ 1.0,  2.0,  3.0,  4.0],
        [ 5.5,  6.5,  7.5,  8.5],
        [ 9.0, 10.0, 11.0, 12.0],
        [13.5, 14.5, 15.5, 16.5],
      ]
    };

    let m3 = Matrix {
      data: [
        [ 1.0,  5.0,  3.0,  4.0],
        [ 5.5,  6.5,  7.5,  8.5],
        [ 9.0, 10.0,  5.0, 12.0],
        [13.5,  5.5, 15.5, 16.5],
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
        [-2.0,  1.0,  2.0,  3.0],
        [ 3.0,  2.0,  1.0, -1.0],
        [ 4.0,  3.0,  6.0,  5.0],
        [ 1.0,  2.0,  7.0,  8.0],
      ]
    };

    let result = Matrix {
      data: [
        [20.0, 22.0,  50.0,  48.0],
        [44.0, 54.0, 114.0, 108.0],
        [40.0, 58.0, 110.0, 102.0],
        [16.0, 26.0,  46.0,  42.0],
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

  #[test]
  fn can_calculate_determinant_of_2x2() {
    let a = [
      [ 1.0,  5.0],
      [-3.0,  2.0],
    ];

    assert_eq!(super::determinant(a), 17.0);
  }

  #[test]
  fn can_calculate_3x3_submatrix() {
    let a = [
      [ 1.0,  5.0,  0.0],
      [-3.0,  2.0,  7.0],
      [ 0.0,  6.0, -3.0]
    ];

    let b = [
      [-3.0,  2.0],
      [ 0.0,  6.0]
    ];

    assert_eq!(super::submatrix3(a, 0, 2), b);
  }

  #[test]
  fn can_calculate_4x4_submatrix() {
    let a = [
      [-6.0,  1.0,  1.0,  6.0],
      [-8.0,  5.0,  8.0,  6.0],
      [-1.0,  0.0,  8.0,  2.0],
      [-7.0,  1.0, -1.0,  1.0],
    ];

    let b = [
      [-6.0,  1.0,  6.0],
      [-8.0,  8.0,  6.0],
      [-7.0, -1.0,  1.0],
    ];

    assert_eq!(super::submatrix4(a, 2, 1), b);
  }

  #[test]
  fn can_calculate_minor_of_3x3() {
    let a = [
      [ 3.0,  5.0,  0.0],
      [ 2.0, -1.0, -7.0],
      [ 6.0, -1.0,  5.0]
    ];

    let b = super::submatrix3(a, 1, 0);

    assert_eq!(super::determinant(b), 25.0);
    assert_eq!(super::minor3(a, 1, 0), 25.0);
  }

  #[test]
  fn can_calculate_cofactor_of_3x3() {
    let a = [
      [ 3.0,  5.0,  0.0],
      [ 2.0, -1.0, -7.0],
      [ 6.0, -1.0,  5.0],
    ];

    assert_eq!(super::minor3(a, 0, 0), -12.0);
    assert_eq!(super::cofactor3(a, 0, 0), -12.0);
    assert_eq!(super::minor3(a, 1, 0), 25.0);
    assert_eq!(super::cofactor3(a, 1, 0), -25.0);
  }

  #[test]
  fn can_calculate_determinant_of_3x3() {
    let a = [
      [ 1.0,  2.0,  6.0],
      [-5.0,  8.0, -4.0],
      [ 2.0,  6.0,  4.0],
    ];

    assert_eq!(super::cofactor3(a, 0, 0), 56.0);
    assert_eq!(super::cofactor3(a, 0, 1), 12.0);
    assert_eq!(super::cofactor3(a, 0, 2), -46.0);
    assert_eq!(super::determinant3(a), -196.0);
  }

  #[test]
  fn can_calculate_determinant_of_4x4() {
    let a = [
      [-2.0, -8.0,  3.0,  5.0],
      [-3.0,  1.0,  7.0,  3.0],
      [ 1.0,  2.0, -9.0,  6.0],
      [-6.0,  7.0,  7.0, -9.0],
    ];

    assert_eq!(super::cofactor4(a, 0, 0), 690.0);
    assert_eq!(super::cofactor4(a, 0, 1), 447.0);
    assert_eq!(super::cofactor4(a, 0, 2), 210.0);
    assert_eq!(super::cofactor4(a, 0, 3), 51.0);
    assert_eq!(super::determinant4(a), -4071.0);
  }

  #[test]
  fn matrix_is_invertible() {
    let a = [
      [ 6.0,  4.0,  4.0,  4.0],
      [ 5.0,  5.0,  7.0,  6.0],
      [ 4.0, -9.0,  3.0, -7.0],
      [ 9.0,  1.0,  7.0, -6.0],
    ];

    assert_eq!(super::determinant4(a), -2120.0);
    assert_eq!(super::invertible(a), true);
  }

  #[test]
  fn matrix_is_not_invertible() {
    let a = [
      [-4.0,  2.0, -2.0, -3.0],
      [ 0.0,  6.0,  2.0,  6.0],
      [ 0.0, -5.0,  1.0, -5.0],
      [ 0.0,  0.0,  0.0,  0.0]
    ];

    assert_eq!(super::determinant4(a), 0.0);
    assert_eq!(super::invertible(a), false);
  }

  #[test]
  fn inverse_of_matrix() {
    let a = [
      [-5.0,  2.0,  6.0, -8.0],
      [ 1.0, -5.0,  1.0,  8.0],
      [ 7.0,  7.0, -6.0, -7.0],
      [ 1.0, -3.0,  7.0,  4.0],
    ];

    let b = super::inverse(a).unwrap();

    let b_should_equal = [
      [ 0.21805,  0.45113,  0.24060, -0.04511],
      [-0.80827, -1.45677, -0.44361,  0.52068],
      [-0.07895, -0.22368, -0.05263,  0.19737],
      [-0.52256, -0.81391, -0.30075,  0.30639]
    ];

    assert_eq!(super::determinant4(a), 532.0);
    assert_eq!(super::cofactor4(a, 2, 3), -160.0);
    assert_eq!(b[3][2], -160.0/532.0);
    assert_eq!(super::cofactor4(a, 3, 2), 105.0);
    assert_eq!(b[2][3], 105.0/532.0);

    for i in 0..4 {
      for j in 0..4 {
        assert!(equal(b[i][j], b_should_equal[i][j]));
      }
    }
  }

  #[test]
  fn matrix_test() {
    let a = Matrix {
      data: [
        [ 8.0, -5.0,  9.0,  2.0],
        [ 7.0,  5.0,  6.0,  1.0],
        [-6.0,  0.0,  9.0,  6.0],
        [-3.0,  0.0, -9.0, -4.0]
      ]
    };

    let b = Matrix {
      data: [
        [-0.15385, -0.15385, -0.28205, -0.53846],
        [-0.07692,  0.12308,  0.02564,  0.03077],
        [ 0.35897,  0.35897,  0.43590,  0.92308],
        [-0.69231, -0.69231, -0.76923, -1.92308]
      ]
    };

    assert_eq!(a.inverse().unwrap(), b);

    let c = Matrix {
      data: [
        [ 9.0,  3.0,  0.0,  9.0],
        [-5.0, -2.0, -6.0, -3.0],
        [-4.0,  9.0,  6.0,  4.0],
        [-7.0,  6.0,  6.0,  2.0]
      ]
    };

    let d = Matrix {
      data: [
        [-0.04074, -0.07778,  0.14444, -0.22222],
        [-0.07778,  0.03333,  0.36667, -0.33333],
        [-0.02901, -0.14630, -0.10926,  0.12963],
        [ 0.17778,  0.06667, -0.26667,  0.33333]
      ]
    };

    assert_eq!(c.inverse().unwrap(), d);
  }

  #[test]
  fn multiplying_by_its_inverse() {
    let a = Matrix {
      data: [
        [ 3.0, -9.0,  7.0,  3.0],
        [ 3.0, -8.0,  2.0, -9.0],
        [-4.0,  4.0,  4.0,  1.0],
        [-6.0,  5.0, -1.0,  1.0]
      ]
    };

    let b = Matrix {
      data: [
        [ 8.0,  2.0,  2.0,  2.0],
        [ 3.0, -1.0,  7.0,  0.0],
        [ 7.0,  0.0,  5.0,  4.0],
        [ 6.0, -2.0,  0.0,  5.0]
      ]
    };

    let c = a * b;

    assert_eq!(c * b.inverse().unwrap(), a);
  }

  #[test]
  fn multiply_by_translation_matrix() {
    let transform = Matrix::translate(5.0, -3.0, 2.0);
    let point = Point { x: -3.0, y: 4.0, z: 5.0 };

    assert_eq!(transform * point, Point { x: 2.0, y: 1.0, z: 7.0 });
  }

  #[test]
  fn multiply_by_inverse_of_translation_matrix() {
    let transform = Matrix::translate(5.0, -3.0, 2.0);
    let inverse = transform.inverse().unwrap();
    let point = Point { x: -3.0, y: 4.0, z: 5.0 };

    assert_eq!(inverse * point, Point { x: -8.0, y: 7.0, z: 3.0 });
  }

  #[test]
  fn translation_does_not_affect_vectors() {
    let transform = Matrix::translate(5.0, -3.0, 2.0);
    let vector = Vector { x: -3.0, y: 4.0, z: 5.0 };

    assert_eq!(transform * vector, Vector { x: -3.0, y: 4.0, z: 5.0 });
  }

  #[test]
  fn scaling_applied_to_point() {
    let transform = Matrix::scale(2.0, 3.0, 4.0);
    let point = Point { x: -4.0, y: 6.0, z: 8.0 };

    assert_eq!(transform * point, Point { x: -8.0, y: 18.0, z: 32.0 });
  }

  #[test]
  fn scaling_applied_to_vector() {
    let transform = Matrix::scale(2.0, 3.0, 4.0);
    let vector = Vector { x: -4.0, y: 6.0, z: 8.0 };

    assert_eq!(transform * vector, Vector { x: -8.0, y: 18.0, z: 32.0 });
  }

  #[test]
  fn multiply_by_inverse_of_scaling_matrix() {
    let transform = Matrix::scale(2.0, 3.0, 4.0);
    let inverse = transform.inverse().unwrap();
    let vector = Vector { x: -4.0, y: 6.0, z: 8.0 };

    assert_eq!(inverse * vector, Vector { x: -2.0, y: 2.0, z: 2.0 });
  }

  #[test]
  fn reflection_is_scaling_by_negative_value() {
    let transform = Matrix::scale(-1.0, 1.0, 1.0);
    let point = Point { x: 2.0, y: 3.0, z: 4.0 };

    assert_eq!(transform * point, Point { x: -2.0, y: 3.0, z: 4.0 });
  }

  #[test]
  fn rotating_around_x() {
    let half_quarter = Matrix::rotate_x(std::f64::consts::PI / 4.0);
    let full_quarter = Matrix::rotate_x(std::f64::consts::PI / 2.0);
    let point = Point { x: 0.0, y: 1.0, z: 0.0 };

    let two: f64 = 2.0;

    assert_eq!(half_quarter * point, Point { x: 0.0, y: two.sqrt() / 2.0, z: two.sqrt() / 2.0 });
    assert_eq!(full_quarter * point, Point { x: 0.0, y: 0.0, z: 1.0 });
  }

  #[test]
  fn rotating_inverse_around_x() {
    let half_quarter = Matrix::rotate_x(std::f64::consts::PI / 4.0);
    let inverse = half_quarter.inverse().unwrap();
    let point = Point { x: 0.0, y: 1.0, z: 0.0 };

    let two: f64 = 2.0;

    assert_eq!(inverse * point, Point { x: 0.0, y: two.sqrt() / 2.0, z: -(two.sqrt() / 2.0) });
  }

  #[test]
  fn rotating_around_y() {
    let half_quarter = Matrix::rotate_y(std::f64::consts::PI / 4.0);
    let full_quarter = Matrix::rotate_y(std::f64::consts::PI / 2.0);
    let point = Point { x: 0.0, y: 0.0, z: 1.0 };

    let two: f64 = 2.0;

    assert_eq!(half_quarter * point, Point { x: two.sqrt() / 2.0, y: 0.0, z: two.sqrt() / 2.0 });
    assert_eq!(full_quarter * point, Point { x: 1.0, y: 0.0, z: 0.0 });
  }

  #[test]
  fn rotating_around_z() {
    let half_quarter = Matrix::rotate_z(std::f64::consts::PI / 4.0);
    let full_quarter = Matrix::rotate_z(std::f64::consts::PI / 2.0);
    let point = Point { x: 0.0, y: 1.0, z: 0.0 };

    let two: f64 = 2.0;

    assert_eq!(half_quarter * point, Point { x: -(two.sqrt() / 2.0), y: two.sqrt() / 2.0, z: 0.0 });
    assert_eq!(full_quarter * point, Point { x: -1.0, y: 0.0, z: 0.0 });
  }

  #[test]
  fn shearing_xyz() {
    let xy = Matrix::shear(1.0, 0.0, 0.0, 0.0, 0.0, 0.0);
    let xz = Matrix::shear(0.0, 1.0, 0.0, 0.0, 0.0, 0.0);
    let yx = Matrix::shear(0.0, 0.0, 1.0, 0.0, 0.0, 0.0);
    let yz = Matrix::shear(0.0, 0.0, 0.0, 1.0, 0.0, 0.0);
    let zx = Matrix::shear(0.0, 0.0, 0.0, 0.0, 1.0, 0.0);
    let zy = Matrix::shear(0.0, 0.0, 0.0, 0.0, 0.0, 1.0);

    let point = Point { x: 2.0, y: 3.0, z: 4.0 };

    assert_eq!(xy * point, Point { x: 5.0, y: 3.0, z: 4.0 });
    assert_eq!(xz * point, Point { x: 6.0, y: 3.0, z: 4.0 });
    assert_eq!(yx * point, Point { x: 2.0, y: 5.0, z: 4.0 });
    assert_eq!(yz * point, Point { x: 2.0, y: 7.0, z: 4.0 });
    assert_eq!(zx * point, Point { x: 2.0, y: 3.0, z: 6.0 });
    assert_eq!(zy * point, Point { x: 2.0, y: 3.0, z: 7.0 });
  }

  #[test]
  fn chaining_transforms() {
    let p = Point { x: 1.0, y: 0.0, z: 1.0 };
    let a = Matrix::rotate_x(std::f64::consts::PI / 2.0);
    let b = Matrix::scale(5.0, 5.0, 5.0);
    let c = Matrix::translate(10.0, 5.0, 7.0);

    let p2 = a * p;
    assert_eq!(p2, Point { x: 1.0, y: -1.0, z: 0.0 });

    let p3 = b * p2;
    assert_eq!(p3, Point { x: 5.0, y: -5.0, z: 0.0 });

    let p4 = c * p3;
    assert_eq!(p4, Point { x: 15.0, y: 0.0, z: 7.0 });
  }

  #[test]
  fn chaining_must_be_applied_in_reverse_order() {
    let p = Point { x: 1.0, y: 0.0, z: 1.0 };
    let a = Matrix::rotate_x(std::f64::consts::PI / 2.0);
    let b = Matrix::scale(5.0, 5.0, 5.0);
    let c = Matrix::translate(10.0, 5.0, 7.0);

    let t = c * b * a;

    assert_eq!(t * p, Point { x: 15.0, y: 0.0, z: 7.0 });
  }
}
