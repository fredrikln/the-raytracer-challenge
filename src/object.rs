use crate::ray::Ray;
use crate::point::Point;
use crate::vector::Vector;
use crate::material::Material;
use crate::matrix::Matrix;
use crate::sphere::Sphere;
use crate::plane::Plane;
use crate::cube::Cube;

pub trait Intersectable {
  fn intersect(&self, r: Ray) -> Vec<f64>;
  fn normal(&self, p: Point) -> Vector;
  fn material(&self) -> Material;
  fn transform(&self) -> Matrix;
  fn casts_shadow(&self) -> bool;
}

#[derive(PartialEq, Debug, Copy, Clone)]
pub enum Object {
  Sphere(Sphere),
  Plane(Plane),
  Cube(Cube),
}

impl Intersectable for Object {
  fn intersect(&self, r: Ray) -> Vec<f64> {
    match *self {
      Object::Sphere(ref s) => s.intersect(r),
      Object::Plane(ref p) => p.intersect(r),
      Object::Cube(ref c) => c.intersect(r),
    }
  }

  fn normal(&self, p: Point) -> Vector {
    match *self {
      Object::Sphere(ref s) => s.normal(p),
      Object::Plane(ref pl) => pl.normal(p),
      Object::Cube(ref c) => c.normal(p),
    }
  }

  fn transform(&self) -> Matrix {
    match *self {
      Object::Sphere(ref s) => s.transform(),
      Object::Plane(ref p) => p.transform(),
      Object::Cube(ref c) => c.transform(),
    }
  }

  fn material(&self) -> Material {
    match *self {
      Object::Sphere(ref s) => s.material(),
      Object::Plane(ref p) => p.material(),
      Object::Cube(ref c) => c.material(),
    }
  }

  fn casts_shadow(&self) -> bool {
    match *self {
      Object::Sphere(ref s) => s.casts_shadow(),
      Object::Plane(ref p) => p.casts_shadow(),
      Object::Cube(ref c) => c.casts_shadow(),
    }
  }
}
