use crate::sphere::Sphere;

#[derive(Debug)]
pub struct Intersection<'a> {
  pub time: f32,
  pub object: &'a Sphere
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
}
