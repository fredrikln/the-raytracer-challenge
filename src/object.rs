use crate::ray::Ray;
use crate::point::Point;
use crate::vector::Vector;
use crate::material::Material;
use crate::matrix::Matrix;
use crate::sphere::Sphere;
use crate::plane::Plane;

pub trait Intersectable {
  fn intersect(&self, r: Ray) -> Vec<f64>;
  fn normal(&self, p: Point) -> Vector;
  fn material(&self) -> Material;
  fn transform(&self) -> Matrix;
  fn casts_shadow(&self) -> bool;
}

#[derive(PartialEq, Debug)]
pub enum Object {
  Sphere(Sphere),
  Plane(Plane)
}

impl Intersectable for Object {
  fn intersect(&self, r: Ray) -> Vec<f64> {
    match *self {
      Object::Sphere(ref s) => s.intersect(r),
      Object::Plane(ref p) => p.intersect(r),
    }
  }

  fn normal(&self, p: Point) -> Vector {
    match *self {
      Object::Sphere(ref s) => s.normal(p),
      Object::Plane(ref pl) => pl.normal(p),
    }
  }

  fn transform(&self) -> Matrix {
    match *self {
      Object::Sphere(ref s) => s.transform(),
      Object::Plane(ref p) => p.transform(),
    }
  }

  fn material(&self) -> Material {
    match *self {
      Object::Sphere(ref s) => s.material(),
      Object::Plane(ref p) => p.material(),
    }
  }

  fn casts_shadow(&self) -> bool {
    match *self {
      Object::Sphere(ref s) => s.casts_shadow(),
      Object::Plane(ref p) => p.casts_shadow(),
    }
  }
}
