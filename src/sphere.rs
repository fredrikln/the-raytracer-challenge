use crate::point::Point;
use crate::ray::Ray;
use crate::intersection::Intersection;
use crate::matrix::Matrix;

#[derive(PartialEq, Debug)]
pub struct Sphere {
  pub transform: Matrix
}

impl Sphere {
  pub fn new() -> Sphere {
    Sphere {
      transform: Matrix::identity()
    }
  }

  pub fn set_transform(&mut self, transform: Matrix) {
    self.transform = transform;
  }

  pub fn intersect(&self, ray: Ray) -> Vec<Intersection> {
    let ray2 = ray * self.transform.inverse().unwrap();
    let sphere_to_ray = ray2.origin - Point { x: 0.0, y: 0.0, z: 0.0 };

    let a = ray2.direction.dot(&ray2.direction);
    let b = 2.0 * ray2.direction.dot(&sphere_to_ray);
    let c = sphere_to_ray.dot(&sphere_to_ray) - 1.0;

    let discriminant = b.powf(2.0) - 4.0 * a * c;

    if discriminant < 0.0 {
      return vec![];
    }

    let t1 = (-b - discriminant.sqrt()) / (2.0 * a);
    let t2 = (-b + discriminant.sqrt()) / (2.0 * a);

    vec![Intersection { time: t1, object: self }, Intersection { time: t2, object: self }]
  }
}

#[cfg(test)]
mod tests {
  use crate::ray::Ray;
  use crate::sphere::Sphere;
  use crate::vector::Vector;
  use crate::point::Point;
  use crate::matrix::Matrix;

  #[test]
  fn insersects_sphere_at_two_points() {
    let ray = Ray { origin: Point { x: 0.0, y: 0.0, z: -5.0 }, direction: Vector { x: 0.0, y: 0.0, z: 1.0 } };
    let sphere = Sphere::new();

    let intersections = sphere.intersect(ray);

    assert_eq!(intersections.len(), 2);
    assert_eq!(intersections[0].time, 4.0);
    assert_eq!(intersections[1].time, 6.0);
  }

  #[test]
  fn insersects_sphere_at_tangent() {
    let ray = Ray { origin: Point { x: 0.0, y: 1.0, z: -5.0 }, direction: Vector { x: 0.0, y: 0.0, z: 1.0 } };
    let sphere = Sphere::new();

    let intersections = sphere.intersect(ray);

    assert_eq!(intersections.len(), 2);
    assert_eq!(intersections[0].time, 5.0);
    assert_eq!(intersections[1].time, 5.0);
  }

  #[test]
  fn ray_originates_inside_sphere() {
    let ray = Ray { origin: Point { x: 0.0, y: 0.0, z: 0.0 }, direction: Vector { x: 0.0, y: 0.0, z: 1.0 } };
    let sphere = Sphere::new();

    let intersections = sphere.intersect(ray);

    assert_eq!(intersections.len(), 2);
    assert_eq!(intersections[0].time, -1.0);
    assert_eq!(intersections[1].time, 1.0);
  }

  #[test]
  fn ray_is_behind_sphere() {
    let ray = Ray { origin: Point { x: 0.0, y: 0.0, z: 5.0 }, direction: Vector { x: 0.0, y: 0.0, z: 1.0 } };
    let sphere = Sphere::new();

    let intersections = sphere.intersect(ray);

    assert_eq!(intersections.len(), 2);
    assert_eq!(intersections[0].time, -6.0);
    assert_eq!(intersections[1].time, -4.0);
  }

  #[test]
  fn intersect_sets_the_object_on_intersection() {
    let ray = Ray { origin: Point { x: 0.0, y: 0.0, z: -5.0 }, direction: Vector { x: 0.0, y: 0.0, z: 1.0 } };
    let sphere = Sphere::new();

    let intersections = sphere.intersect(ray);

    assert_eq!(intersections.len(), 2);
    assert_eq!(intersections[0].object, &sphere);
    assert_eq!(intersections[1].object, &sphere);
  }

  #[test]
  fn a_spheres_default_transformation() {
    let s = Sphere::new();

    assert_eq!(s.transform, Matrix::identity())
  }

  #[test]
  fn changing_a_spheres_transformation() {
    let mut s = Sphere::new();
    assert_eq!(s.transform, Matrix::identity());

    let t = Matrix::translate(2.0, 3.0, 4.0);
    s.set_transform(t);

    assert_eq!(s.transform, t)
  }

  #[test]
  fn intersecting_a_scaled_sphere_with_a_ray() {
    let ray = Ray { origin: Point { x: 0.0, y: 0.0, z: -5.0 }, direction: Vector { x: 0.0, y: 0.0, z: 1.0 } };
    let mut sphere = Sphere::new();
    sphere.set_transform(Matrix::scale(2.0, 2.0, 2.0));

    let intersections = sphere.intersect(ray);

    assert_eq!(intersections[0].time, 3.0);
    assert_eq!(intersections[1].time, 7.0);
  }

  #[test]
  fn intersecting_a_translated_sphere_with_a_ray() {
    let ray = Ray { origin: Point { x: 0.0, y: 0.0, z: -5.0 }, direction: Vector { x: 0.0, y: 0.0, z: 1.0 } };
    let mut sphere = Sphere::new();
    sphere.set_transform(Matrix::translate(5.0, 0.0, 0.0));

    let intersections = sphere.intersect(ray);

    assert_eq!(intersections.len(), 0);
  }
}
