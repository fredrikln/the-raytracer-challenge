use crate::point::Point;
use crate::ray::Ray;
use crate::intersection::Intersection;
use crate::matrix::Matrix;
use crate::vector::Vector;
use crate::material::Material;
use crate::object::{Object,Intersectable};
use crate::utils::EPSILON;

#[derive(PartialEq, Debug, Copy, Clone)]
pub struct Cube {
  pub transform: Matrix,
  pub material: Material,
  pub casts_shadow: bool,
}

impl Intersectable for Cube {
  fn normal(&self, p: Point) -> Vector {
    let object_point = self.transform.inverse().unwrap() * p;

    let maxc = object_point.x.abs().max(object_point.y.abs().max(object_point.z.abs()));
    let v: Vector;

    if maxc == object_point.x.abs() {
      v = Vector { x: object_point.x, y: 0.0, z: 0.0 }
    } else if maxc == object_point.y.abs() {
      v = Vector { x: 0.0, y: object_point.y, z: 0.0 }
    } else {
      v = Vector { x: 0.0, y: 0.0, z: object_point.z }
    }

    let world_normal = self.transform.inverse().unwrap().transpose() * v;

    world_normal.normalize()
  }

  fn intersect(&self, r: Ray) -> Vec<f64> {
    let ray2 = r * self.transform.inverse().unwrap();

    let [xtmin, xtmax] = Cube::check_axis(ray2.origin.x, ray2.direction.x);
    let [ytmin, ytmax] = Cube::check_axis(ray2.origin.y, ray2.direction.y);
    let [ztmin, ztmax] = Cube::check_axis(ray2.origin.z, ray2.direction.z);

    let tmin = xtmin.max(ytmin.max(ztmin));
    let tmax = xtmax.min(ytmax.min(ztmax));

    if tmin > tmax {
      return vec![];
    }

    vec![tmin, tmax]
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

impl Cube {
  pub fn new() -> Cube {
    Cube {
      transform: Matrix::identity(),
      material: Material::new(),
      casts_shadow: true,
    }
  }

  pub fn set_transform(&mut self, transform: Matrix) {
    self.transform = transform;
  }

  pub fn check_axis(origin: f64, direction: f64) -> [f64; 2] {
    let tmin_numerator = -1.0 - origin;
    let tmax_numerator = 1.0 - origin;

    let mut tmin: f64;
    let mut tmax: f64;

    if direction.abs() >= EPSILON {
      tmin = tmin_numerator / direction;
      tmax = tmax_numerator / direction;
    } else {
      tmin = tmin_numerator * std::f64::INFINITY;
      tmax = tmax_numerator * std::f64::INFINITY;
    }

    if tmin > tmax {
      let temp = tmin;
      tmin = tmax;
      tmax = temp;
    }

    return [tmin, tmax];
  }
}

#[cfg(test)]
mod tests {
  // use crate::ray::Ray;
  // use crate::sphere::Sphere;
  // use crate::vector::Vector;
  // use crate::point::Point;
  // use crate::matrix::Matrix;
  // use crate::material::Material;
  // use crate::canvas::Color;
  // use crate::object::{Object, Intersectable};

  // #[test]
  // fn insersects_sphere_at_two_points() {
  //   let ray = Ray { origin: Point { x: 0.0, y: 0.0, z: -5.0 }, direction: Vector { x: 0.0, y: 0.0, z: 1.0 } };
  //   let sphere = Sphere::new();

  //   let intersections = sphere.intersect(ray);

  //   assert_eq!(intersections.len(), 2);
  //   assert_eq!(intersections[0], 4.0);
  //   assert_eq!(intersections[1], 6.0);
  // }

  // #[test]
  // fn insersects_sphere_at_tangent() {
  //   let ray = Ray { origin: Point { x: 0.0, y: 1.0, z: -5.0 }, direction: Vector { x: 0.0, y: 0.0, z: 1.0 } };
  //   let sphere = Sphere::new();

  //   let intersections = sphere.intersect(ray);

  //   assert_eq!(intersections.len(), 2);
  //   assert_eq!(intersections[0], 5.0);
  //   assert_eq!(intersections[1], 5.0);
  // }

  // #[test]
  // fn ray_originates_inside_sphere() {
  //   let ray = Ray { origin: Point { x: 0.0, y: 0.0, z: 0.0 }, direction: Vector { x: 0.0, y: 0.0, z: 1.0 } };
  //   let sphere = Sphere::new();

  //   let intersections = sphere.intersect(ray);

  //   assert_eq!(intersections.len(), 2);
  //   assert_eq!(intersections[0], -1.0);
  //   assert_eq!(intersections[1], 1.0);
  // }

  // #[test]
  // fn ray_is_behind_sphere() {
  //   let ray = Ray { origin: Point { x: 0.0, y: 0.0, z: 5.0 }, direction: Vector { x: 0.0, y: 0.0, z: 1.0 } };
  //   let sphere = Sphere::new();

  //   let intersections = sphere.intersect(ray);

  //   assert_eq!(intersections.len(), 2);
  //   assert_eq!(intersections[0], -6.0);
  //   assert_eq!(intersections[1], -4.0);
  // }

  // // #[test]
  // // fn intersect_sets_the_object_on_intersection() {
  // //   let ray = Ray { origin: Point { x: 0.0, y: 0.0, z: -5.0 }, direction: Vector { x: 0.0, y: 0.0, z: 1.0 } };
  // //   let sphere = Sphere::new();

  // //   let intersections = sphere.intersect(ray);

  // //   assert_eq!(intersections.len(), 2);
  // //   assert_eq!(*intersections[0].object, Object::Sphere(sphere));
  // //   assert_eq!(*intersections[1].object, Object::Sphere(sphere));
  // // }

  // #[test]
  // fn a_spheres_default_transformation() {
  //   let s = Sphere::new();

  //   assert_eq!(s.transform, Matrix::identity())
  // }

  // #[test]
  // fn changing_a_spheres_transformation() {
  //   let mut s = Sphere::new();
  //   assert_eq!(s.transform, Matrix::identity());

  //   let t = Matrix::translate(2.0, 3.0, 4.0);
  //   s.set_transform(t);

  //   assert_eq!(s.transform, t)
  // }

  // #[test]
  // fn intersecting_a_scaled_sphere_with_a_ray() {
  //   let ray = Ray { origin: Point { x: 0.0, y: 0.0, z: -5.0 }, direction: Vector { x: 0.0, y: 0.0, z: 1.0 } };
  //   let mut sphere = Sphere::new();
  //   sphere.set_transform(Matrix::scale(2.0, 2.0, 2.0));

  //   let intersections = sphere.intersect(ray);

  //   assert_eq!(intersections[0], 3.0);
  //   assert_eq!(intersections[1], 7.0);
  // }

  // #[test]
  // fn intersecting_a_translated_sphere_with_a_ray() {
  //   let ray = Ray { origin: Point { x: 0.0, y: 0.0, z: -5.0 }, direction: Vector { x: 0.0, y: 0.0, z: 1.0 } };
  //   let mut sphere = Sphere::new();
  //   sphere.set_transform(Matrix::translate(5.0, 0.0, 0.0));

  //   let intersections = sphere.intersect(ray);

  //   assert_eq!(intersections.len(), 0);
  // }

  // #[test]
  // fn normal_at_a_point_on_x_axis() {
  //   let s = Sphere::new();
  //   let n = s.normal(Point { x: 1.0, y: 0.0, z: 0.0 });

  //   assert_eq!(n, Vector { x: 1.0, y: 0.0, z: 0.0 });
  // }

  // #[test]
  // fn normal_at_a_point_on_y_axis() {
  //   let s = Sphere::new();
  //   let n = s.normal(Point { x: 0.0, y: 1.0, z: 0.0 });

  //   assert_eq!(n, Vector { x: 0.0, y: 1.0, z: 0.0 });
  // }

  // #[test]
  // fn normal_at_a_point_on_z_axis() {
  //   let s = Sphere::new();
  //   let n = s.normal(Point { x: 0.0, y: 0.0, z: 1.0 });

  //   assert_eq!(n, Vector { x: 0.0, y: 0.0, z: 1.0 });
  // }

  // #[test]
  // fn normal_at_a_nonaxial_point() {
  //   let s = Sphere::new();
  //   let n = s.normal(Point { x: (3.0 as f64).sqrt() / 3.0, y: (3.0 as f64).sqrt() / 3.0, z: (3.0 as f64).sqrt() / 3.0 });

  //   assert_eq!(n, Vector { x: (3.0 as f64).sqrt() / 3.0, y: (3.0 as f64).sqrt() / 3.0, z: (3.0 as f64).sqrt() / 3.0 });
  // }

  // #[test]
  // fn normal_is_normalized() {
  //   let s = Sphere::new();
  //   let n = s.normal(Point { x: (3.0 as f64).sqrt() / 3.0, y: (3.0 as f64).sqrt() / 3.0, z: (3.0 as f64).sqrt() / 3.0 });

  //   assert_eq!(n, n.normalize());
  // }

  // #[test]
  // fn computing_normal_of_translated_sphere() {
  //   let mut s = Sphere::new();
  //   s.set_transform(Matrix::translate(0.0, 1.0, 0.0));

  //   let n = s.normal(Point { x: 0.0, y: 1.70711, z: -0.70711 });

  //   assert_eq!(n, Vector { x: 0.0, y: 0.70711, z: -0.70711 });
  // }

  // #[test]
  // fn computing_normal_of_transformed_sphere() {
  //   let mut s = Sphere::new();
  //   let transform = Matrix::scale(1.0, 0.5, 1.0) * Matrix::rotate_z(std::f64::consts::PI / 5.0);
  //   s.set_transform(transform);

  //   let n = s.normal(Point { x: 0.0, y: (2.0 as f64).sqrt() / 2.0, z: -((2.0 as f64).sqrt() / 2.0) });

  //   assert_eq!(n, Vector { x: 0.0, y: 0.97014, z: -0.24254 });
  // }

  // #[test]
  // fn a_sphere_has_a_default_material() {
  //   let s = Sphere::new();

  //   assert_eq!(s.material, Material::new());
  // }

  // #[test]
  // fn a_sphere_can_be_assigned_a_material() {
  //   let mut s = Sphere::new();
  //   let mut m = Material::new();
  //   m.ambient = 1.0;
  //   s.material = m.clone();

  //   let cm = Material { color: Color { r: 1.0, g: 1.0, b: 1.0 }, ambient: 1.0, diffuse: 0.9, specular: 0.9, shininess: 200.0, pattern: None, reflective: 0.0, refractive_index: 1.0, transparency: 0.0 };

  //   assert_eq!(s.material, cm);
  // }
}
