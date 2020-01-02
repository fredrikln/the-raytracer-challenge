use crate::sphere::Sphere;
use crate::ray::Ray;
use crate::vector::Vector;
use crate::point::Point;
use crate::utils::EPSILON;

use std::cmp::Ordering::Equal;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Intersection<'a> {
  pub time: f32,
  pub object: &'a Sphere
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

  pub fn prepare_computations(&self, ray: Ray) -> Computations {
    let point = ray.position(self.time);

    let mut normal = self.object.normal(point);
    let eye_vector = -ray.direction;

    let inside: bool;
    if normal.dot(&eye_vector) < 0.0 {
      inside = true;
      normal = -normal;
    } else {
      inside = false
    }

    let over_point = point + normal * EPSILON * 150.0;

    Computations {
      time: self.time,
      object: self.object,
      point,
      eye_vector,
      normal,
      inside,
      over_point,
    }
  }
}

pub struct Computations<'a> {
  pub time: f32,
  pub object: &'a Sphere,
  pub point: Point,
  pub eye_vector: Vector,
  pub normal: Vector,
  pub inside: bool,
  pub over_point: Point,
}

#[cfg(test)]
mod tests {
  use crate::point::Point;
  use crate::sphere::Sphere;
  use crate::intersection::Intersection;
  use crate::ray::Ray;
  use crate::vector::Vector;

  #[test]
  fn an_intersection_encapsulates_time_and_object() {
    let s = Sphere::new();
    let i = Intersection { time: 3.5, object: &s };

    assert_eq!(i.time, 3.5);
    assert_eq!(i.object, &s);
  }

  #[test]
  fn the_hit_when_all_intersections_positive() {
    let s = Sphere::new();
    let i1 = Intersection { time: 1.0, object: &s };
    let i2 = Intersection { time: 2.0, object: &s };
    let intersections = vec![i1, i2];

    assert_eq!(Intersection::hit(intersections).unwrap(), i1);
  }

  #[test]
  fn the_hit_when_some_intersections_negative() {
    let s = Sphere::new();
    let i1 = Intersection { time: -1.0, object: &s };
    let i2 = Intersection { time: 1.0, object: &s };
    let intersections = vec![i1, i2];

    assert_eq!(Intersection::hit(intersections).unwrap(), i2);
  }

  #[test]
  fn the_hit_when_all_intersections_negative() {
    let s = Sphere::new();
    let i1 = Intersection { time: -2.0, object: &s };
    let i2 = Intersection { time: -1.0, object: &s };
    let intersections = vec![i1, i2];

    assert_eq!(Intersection::hit(intersections), None);
  }

  #[test]
  fn the_lowest_non_negative_intersection() {
    let s = Sphere::new();
    let i1 = Intersection { time: 5.0, object: &s };
    let i2 = Intersection { time: 7.0, object: &s };
    let i3 = Intersection { time: -3.0, object: &s };
    let i4 = Intersection { time: 2.0, object: &s };
    let intersections = vec![i1, i2, i3, i4];

    assert_eq!(Intersection::hit(intersections).unwrap(), i4);
  }

  #[test]
  fn precomputing_the_state_of_an_intersection() {
    let r = Ray { origin: Point { x: 0.0, y: 0.0, z: -5.0 }, direction: Vector { x: 0.0, y: 0.0, z: 1.0 } };
    let s = Sphere::new();
    let i = Intersection { time: 4.0, object: &s };

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
    let i = Intersection { time: 4.0, object: &s };

    let comps = i.prepare_computations(r);

    assert_eq!(comps.inside, false);
  }

  #[test]
  fn the_hit_when_intersection_occurs_inside() {
    let r = Ray { origin: Point { x: 0.0, y: 0.0, z: 0.0 }, direction: Vector { x: 0.0, y: 0.0, z: 1.0 } };
    let s = Sphere::new();
    let i = Intersection { time: 1.0, object: &s };

    let comps = i.prepare_computations(r);

    assert_eq!(comps.inside, true);
    assert_eq!(comps.point, Point { x: 0.0, y: 0.0, z: 1.0 });
    assert_eq!(comps.eye_vector, Vector { x: 0.0, y: 0.0, z: -1.0 });
    assert_eq!(comps.normal, Vector { x: 0.0, y: 0.0, z: -1.0 });
  }
}
