use crate::ray::Ray;
use crate::intersection::Intersection;
use crate::point::Point;
use crate::vector::Vector;
use crate::material::Material;
use crate::matrix::Matrix;
use crate::sphere::Sphere;

pub trait Intersectable {
  fn intersect(&self, r: Ray) -> Vec<f64>;
  fn normal(&self, p: Point) -> Vector;
  fn material(&self) -> Material;
  fn transform(&self) -> Matrix;
}

#[derive(PartialEq, Debug)]
pub enum Object {
  Sphere(Sphere)
}

impl Intersectable for Object {
  fn intersect(&self, r: Ray) -> Vec<f64> {
    match *self {
      Object::Sphere(ref s) => s.intersect(r)
    }
  }

  fn normal(&self, p: Point) -> Vector {
    match *self {
      Object::Sphere(ref s) => s.normal(p)
    }
  }

  fn transform(&self) -> Matrix {
    match *self {
      Object::Sphere(ref s) => s.transform()
    }
  }

  fn material(&self) -> Material {
    match *self {
      Object::Sphere(ref s) => s.material()
    }
  }
}

