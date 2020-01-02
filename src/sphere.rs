use crate::point::Point;
use crate::ray::Ray;
use crate::intersection::Intersection;
use crate::matrix::Matrix;
use crate::vector::Vector;
use crate::material::Material;

#[derive(PartialEq, Debug, Copy, Clone)]
pub struct Sphere {
  pub transform: Matrix,
  pub material: Material,
}

impl Sphere {
  pub fn new() -> Sphere {
    Sphere {
      transform: Matrix::identity(),
      material: Material::new(),
    }
  }

  pub fn normal(&self, world_point: Point) -> Vector {
    let object_point = self.transform.inverse().unwrap() * world_point;
    let object_normal = object_point - Point { x: 0.0, y: 0.0, z: 0.0 };
    let world_normal = self.transform.inverse().unwrap().transpose() * object_normal;

    world_normal.normalize()
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
  use crate::material::Material;
  use crate::canvas::Color;

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

  #[test]
  fn normal_at_a_point_on_x_axis() {
    let s = Sphere::new();
    let n = s.normal(Point { x: 1.0, y: 0.0, z: 0.0 });

    assert_eq!(n, Vector { x: 1.0, y: 0.0, z: 0.0 });
  }

  #[test]
  fn normal_at_a_point_on_y_axis() {
    let s = Sphere::new();
    let n = s.normal(Point { x: 0.0, y: 1.0, z: 0.0 });

    assert_eq!(n, Vector { x: 0.0, y: 1.0, z: 0.0 });
  }

  #[test]
  fn normal_at_a_point_on_z_axis() {
    let s = Sphere::new();
    let n = s.normal(Point { x: 0.0, y: 0.0, z: 1.0 });

    assert_eq!(n, Vector { x: 0.0, y: 0.0, z: 1.0 });
  }

  #[test]
  fn normal_at_a_nonaxial_point() {
    let s = Sphere::new();
    let n = s.normal(Point { x: (3.0 as f64).sqrt() / 3.0, y: (3.0 as f64).sqrt() / 3.0, z: (3.0 as f64).sqrt() / 3.0 });

    assert_eq!(n, Vector { x: (3.0 as f64).sqrt() / 3.0, y: (3.0 as f64).sqrt() / 3.0, z: (3.0 as f64).sqrt() / 3.0 });
  }

  #[test]
  fn normal_is_normalized() {
    let s = Sphere::new();
    let n = s.normal(Point { x: (3.0 as f64).sqrt() / 3.0, y: (3.0 as f64).sqrt() / 3.0, z: (3.0 as f64).sqrt() / 3.0 });

    assert_eq!(n, n.normalize());
  }

  #[test]
  fn computing_normal_of_translated_sphere() {
    let mut s = Sphere::new();
    s.set_transform(Matrix::translate(0.0, 1.0, 0.0));

    let n = s.normal(Point { x: 0.0, y: 1.70711, z: -0.70711 });

    assert_eq!(n, Vector { x: 0.0, y: 0.70711, z: -0.70711 });
  }

  #[test]
  fn computing_normal_of_transformed_sphere() {
    let mut s = Sphere::new();
    let transform = Matrix::scale(1.0, 0.5, 1.0) * Matrix::rotate_z(std::f64::consts::PI / 5.0);
    s.set_transform(transform);

    let n = s.normal(Point { x: 0.0, y: (2.0 as f64).sqrt() / 2.0, z: -((2.0 as f64).sqrt() / 2.0) });

    assert_eq!(n, Vector { x: 0.0, y: 0.97014, z: -0.24254 });
  }

  #[test]
  fn a_sphere_has_a_default_material() {
    let s = Sphere::new();

    assert_eq!(s.material, Material::new());
  }

  #[test]
  fn a_sphere_can_be_assigned_a_material() {
    let mut s = Sphere::new();
    let mut m = Material::new();
    m.ambient = 1.0;
    s.material = m.clone();

    let cm = Material { color: Color { r: 1.0, g: 1.0, b: 1.0 }, ambient: 1.0, diffuse: 0.9, specular: 0.9, shininess: 200.0 };

    assert_eq!(s.material, cm);
  }
}
