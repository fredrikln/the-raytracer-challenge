use crate::sphere::Sphere;
use crate::ray::Ray;
use crate::vector::Vector;
use crate::point::Point;
use crate::utils::EPSILON;
use crate::object::{Object,Intersectable};

use std::cmp::Ordering::Equal;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Intersection<'a> {
  pub time: f64,
  pub object: &'a Object
}

impl Intersection<'_> {
  pub fn hit(intersections: Vec<Intersection>) -> Option<Intersection> {
    let mut copy = intersections.clone();
    copy.retain(|a| a.time > 0.0);
    copy.sort_by(|a, b| a.time.partial_cmp(&b.time).unwrap_or(Equal));

    if copy.len() == 0 {
      return None
    }

    Some(copy[0])
  }

  pub fn shadow_hit(intersections: Vec<Intersection>) -> Option<Intersection> {
    let mut copy = intersections.clone();
    copy.retain(|i| i.object.casts_shadow());
    copy.retain(|i| i.time > 0.0);
    copy.sort_by(|a, b| a.time.partial_cmp(&b.time).unwrap_or(Equal));

    if copy.len() == 0 {
      return None
    }

    Some(copy[0])
  }

  pub fn prepare_computations_with_intersections(&self, r: Ray, intersections: Vec<Intersection>) -> Computations {
    let mut comps = self.prepare_computations(r);
    let hit = Intersection::hit(intersections.clone());

    let mut containers: Vec<Object> = vec![];

    let mut n1: f64 = 1.0;
    let mut n2: f64 = 1.0;

    for i in intersections {
      if i == hit.unwrap() {
        if containers.len() == 0 {
          n1 = 1.0;
        } else {
          n1 = containers.last().unwrap().material().refractive_index;
        }
      }

      let contains = containers.iter().position(|&o| o == *i.object);
      if contains.is_some() {
        containers.remove(contains.unwrap());
      } else {
        containers.push(*i.object);
      }

      if i == hit.unwrap() {
        if containers.len() == 0 {
          n2 = 1.0;
        } else {
          n2 = containers.last().unwrap().material().refractive_index;
        }
      }
    }

    comps.n1 = n1;
    comps.n2 = n2;

    comps
  }

  pub fn prepare_computations(&self, ray: Ray) -> Computations {
    let point = ray.position(self.time);

    let mut normal = self.object.normal(point);
    let eye_vector = -ray.direction;

    let reflect_vector = ray.direction.reflect(normal);

    let inside: bool;
    if normal.dot(&eye_vector) < 0.0 {
      inside = true;
      normal = -normal;
    } else {
      inside = false
    }

    let over_point = point + normal * EPSILON;
    let under_point = point - normal * EPSILON;

    Computations {
      time: self.time,
      object: self.object,
      point,
      eye_vector,
      normal,
      reflect_vector,
      inside,
      over_point,
      under_point,
      n1: 1.0,
      n2: 1.0,
    }
  }
}

#[derive(Debug)]
pub struct Computations<'a> {
  pub time: f64,
  pub object: &'a Object,
  pub point: Point,
  pub eye_vector: Vector,
  pub normal: Vector,
  pub reflect_vector: Vector,
  pub inside: bool,
  pub over_point: Point,
  pub under_point: Point,
  pub n1: f64,
  pub n2: f64,
}

#[cfg(test)]
mod tests {
  use crate::point::Point;
  use crate::sphere::Sphere;
  use crate::intersection::Intersection;
  use crate::ray::Ray;
  use crate::vector::Vector;
  use crate::object::Object;

  #[test]
  fn an_intersection_encapsulates_time_and_object() {
    let s = Sphere::new();
    let i = Intersection { time: 3.5, object: &Object::Sphere(s) };

    assert_eq!(i.time, 3.5);
    assert_eq!(i.object, &Object::Sphere(s));
  }

  #[test]
  fn the_hit_when_all_intersections_positive() {
    let s = Sphere::new();
    let i1 = Intersection { time: 1.0, object: &Object::Sphere(s) };
    let i2 = Intersection { time: 2.0, object: &Object::Sphere(s) };
    let intersections = vec![i1, i2];

    assert_eq!(Intersection::hit(intersections).unwrap(), i1);
  }

  #[test]
  fn the_hit_when_some_intersections_negative() {
    let s = Sphere::new();
    let i1 = Intersection { time: -1.0, object: &Object::Sphere(s) };
    let i2 = Intersection { time: 1.0, object: &Object::Sphere(s) };
    let intersections = vec![i1, i2];

    assert_eq!(Intersection::hit(intersections).unwrap(), i2);
  }

  #[test]
  fn the_hit_when_all_intersections_negative() {
    let s = Sphere::new();
    let i1 = Intersection { time: -2.0, object: &Object::Sphere(s) };
    let i2 = Intersection { time: -1.0, object: &Object::Sphere(s) };
    let intersections = vec![i1, i2];

    assert_eq!(Intersection::hit(intersections), None);
  }

  #[test]
  fn the_lowest_non_negative_intersection() {
    let s = Sphere::new();
    let i1 = Intersection { time: 5.0, object: &Object::Sphere(s) };
    let i2 = Intersection { time: 7.0, object: &Object::Sphere(s) };
    let i3 = Intersection { time: -3.0, object: &Object::Sphere(s) };
    let i4 = Intersection { time: 2.0, object: &Object::Sphere(s) };
    let intersections = vec![i1, i2, i3, i4];

    assert_eq!(Intersection::hit(intersections).unwrap(), i4);
  }

  #[test]
  fn precomputing_the_state_of_an_intersection() {
    let r = Ray { origin: Point { x: 0.0, y: 0.0, z: -5.0 }, direction: Vector { x: 0.0, y: 0.0, z: 1.0 } };
    let s = Sphere::new();
    let i = Intersection { time: 4.0, object: &Object::Sphere(s) };

    let comps = i.prepare_computations(r);

    assert_eq!(comps.time, i.time);
    assert_eq!(comps.object, i.object);
    assert_eq!(comps.point, Point { x: 0.0, y: 0.0, z: -1.0 });
    assert_eq!(comps.eye_vector, Vector { x: 0.0, y: 0.0, z: -1.0 });
    assert_eq!(comps.normal, Vector { x: 0.0, y: 0.0, z: -1.0 });
  }

  #[test]
  fn the_hit_when_intersection_occurs_outside() {
    let r = Ray { origin: Point { x: 0.0, y: 0.0, z: -5.0 }, direction: Vector { x: 0.0, y: 0.0, z: 1.0 } };
    let s = Sphere::new();
    let i = Intersection { time: 4.0, object: &Object::Sphere(s) };

    let comps = i.prepare_computations(r);

    assert_eq!(comps.inside, false);
  }

  #[test]
  fn the_hit_when_intersection_occurs_inside() {
    let r = Ray { origin: Point { x: 0.0, y: 0.0, z: 0.0 }, direction: Vector { x: 0.0, y: 0.0, z: 1.0 } };
    let s = Sphere::new();
    let i = Intersection { time: 1.0, object: &Object::Sphere(s) };

    let comps = i.prepare_computations(r);

    assert_eq!(comps.inside, true);
    assert_eq!(comps.point, Point { x: 0.0, y: 0.0, z: 1.0 });
    assert_eq!(comps.eye_vector, Vector { x: 0.0, y: 0.0, z: -1.0 });
    assert_eq!(comps.normal, Vector { x: 0.0, y: 0.0, z: -1.0 });
  }
}
