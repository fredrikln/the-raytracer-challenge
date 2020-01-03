use crate::point::Point;
use crate::ray::Ray;
use crate::intersection::Intersection;
use crate::matrix::Matrix;
use crate::vector::Vector;
use crate::material::Material;
use crate::object::{Object,Intersectable};
use crate::utils::EPSILON;

#[derive(PartialEq, Debug, Copy, Clone)]
pub struct Plane {
  pub transform: Matrix,
  pub material: Material,
  pub casts_shadow: bool,
}

impl Intersectable for Plane {
  fn normal(&self, p: Point) -> Vector {
    (self.transform.inverse().unwrap().transpose() * Vector { x: 0.0, y: 1.0, z: 0.0 }).normalize()
  }

  fn intersect(&self, r: Ray) -> Vec<f64> {
    let ray2 = r * self.transform.inverse().unwrap();

    if (ray2.direction.y).abs() < EPSILON {
      return vec![];
    }

    let t = (-ray2.origin.y) / ray2.direction.y;

    vec![t]
  }

  fn material(&self) -> Material {
    self.material
  }

  fn transform(&self) -> Matrix {
    self.transform
  }

  fn casts_shadow(&self) -> bool {
    self.casts_shadow
  }
}

impl Plane {
  pub fn new() -> Plane {
    Plane {
      transform: Matrix::identity(),
      material: Material::new(),
      casts_shadow: true
    }
  }

  pub fn set_transform(&mut self, transform: Matrix) {
    self.transform = transform;
  }
}

#[cfg(test)]
mod tests {
  use crate::ray::Ray;
  use crate::plane::Plane;
  use crate::vector::Vector;
  use crate::point::Point;
  use crate::matrix::Matrix;
  use crate::material::Material;
  use crate::canvas::Color;
  use crate::object::{Object, Intersectable};

  #[test]
  fn the_normal_of_a_plane_is_constant_everywhere() {
    let p = Plane::new();

    let n1 = p.normal(Point { x: 0.0, y: 0.0, z: 0.0 });
    let n2 = p.normal(Point { x: 10.0, y: 0.0, z: -10.0 });
    let n3 = p.normal(Point { x: -5.0, y: 0.0, z: 150.0 });

    assert_eq!(n1, Vector { x: 0.0, y: 1.0, z: 0.0 });
    assert_eq!(n2, Vector { x: 0.0, y: 1.0, z: 0.0 });
    assert_eq!(n3, Vector { x: 0.0, y: 1.0, z: 0.0 });
  }

  #[test]
  fn intersect_with_a_ray_parallell_to_the_plane() {
    let p = Plane::new();
    let r = Ray { origin: Point { x: 0.0, y: 10.0, z: 0.0 }, direction: Vector { x: 0.0, y: 0.0, z: 1.0 } };

    let intersections = p.intersect(r);

    assert_eq!(intersections.len(), 0);
  }

  #[test]
  fn intersect_with_a_ray_coplanar_to_the_plane() {
    let p = Plane::new();
    let r = Ray { origin: Point { x: 0.0, y: 0.0, z: 0.0 }, direction: Vector { x: 0.0, y: 0.0, z: 1.0 } };

    let intersections = p.intersect(r);

    assert_eq!(intersections.len(), 0);
  }

  #[test]
  fn intersect_with_a_ray_from_above() {
    let p = Plane::new();
    let r = Ray { origin: Point { x: 0.0, y: 1.0, z: 0.0 }, direction: Vector { x: 0.0, y: -1.0, z: 0.0 } };

    let intersections = p.intersect(r);

    assert_eq!(intersections.len(), 1);
    assert_eq!(intersections[0], 1.0);
  }

  #[test]
  fn intersect_with_a_ray_from_below() {
    let p = Plane::new();
    let r = Ray { origin: Point { x: 0.0, y: -1.0, z: 0.0 }, direction: Vector { x: 0.0, y: 1.0, z: 0.0 } };

    let intersections = p.intersect(r);

    assert_eq!(intersections.len(), 1);
    assert_eq!(intersections[0], 1.0);
  }
}
