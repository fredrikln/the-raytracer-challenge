use crate::sphere::Sphere;
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
}

#[cfg(test)]
mod tests {
  use crate::sphere::Sphere;
  use crate::intersection::Intersection;

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
}
