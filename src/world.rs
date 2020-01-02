use crate::sphere::Sphere;
use crate::point_light::PointLight;
use crate::canvas::Color;
use crate::material::Material;
use crate::matrix::Matrix;
use crate::point::Point;
use crate::ray::Ray;
use crate::intersection::Intersection;
use crate::intersection::Computations;
use crate::vector::Vector;

use std::cmp::Ordering::Equal;

#[derive(Debug)]
pub struct World {
  pub objects: Vec<Sphere>,
  pub lights: Vec<PointLight>,
}

impl World {
  pub fn new() -> World {
    World {
      objects: vec![],
      lights: vec![],
    }
  }

  pub fn default() -> World {
    let mut s1_material = Material::new();
    s1_material.color = Color { r: 0.8, g: 1.0, b: 0.6 };
    s1_material.diffuse = 0.7;
    s1_material.specular = 0.2;
    let mut s1 = Sphere::new();
    s1.material = s1_material;

    let mut s2 = Sphere::new();
    s2.transform = Matrix::scale(0.5, 0.5, 0.5);

    let light = PointLight { position: Point { x: -10.0, y: 10.0, z: -10.0 }, intensity: Color { r: 1.0, g: 1.0, b: 1.0 } };

    World {
      objects: vec![s1, s2],
      lights: vec![light]
    }
  }

  pub fn intersect(&self, r: Ray) -> Vec<Intersection> {
    let mut intersections: Vec<Intersection> = vec![];

    for (i, object) in self.objects.iter().enumerate() {
      let mut object_intersections = object.intersect(r);
      intersections.append(&mut object_intersections);
    }

    intersections.sort_by(|a, b| a.time.partial_cmp(&b.time).unwrap_or(Equal));

    intersections
  }

  pub fn shade_hit(&self, comps: Computations) -> Color {
    let mut color = Color { r: 0.0, g: 0.0, b: 0.0 };

    for (i, light) in self.lights.iter().enumerate() {
      color = color + comps.object.material.lighting(*light, comps.point, comps.eye_vector, comps.normal);
    }

    color
  }

  pub fn color_at(&self, r: Ray) -> Color {
    let intersections = self.intersect(r);
    let hit = Intersection::hit(intersections);

    if hit.is_none() {
      return Color { r: 0.0, g: 0.0, b: 0.0 }
    }

    let unwrapped_hit = hit.unwrap();
    let comps = unwrapped_hit.prepare_computations(r);

    self.shade_hit(comps)
  }
}

// fn prepare_computations()

#[cfg(test)]
mod tests {
  use crate::world::World;
  use crate::sphere::Sphere;
  use crate::point_light::PointLight;
  use crate::canvas::Color;
  use crate::material::Material;
  use crate::point::Point;
  use crate::matrix::Matrix;
  use crate::vector::Vector;
  use crate::ray::Ray;
  use crate::intersection::Intersection;

  #[test]
  fn empty_world() {
    let w = World::new();

    assert_eq!(w.objects.len(), 0);
    assert_eq!(w.lights.len(), 0);
  }

  #[test]
  fn default_world() {
    let w = World::default();

    let mut s1_material = Material::new();
    s1_material.color = Color { r: 0.8, g: 1.0, b: 0.6 };
    s1_material.diffuse = 0.7;
    s1_material.specular = 0.2;
    let mut s1 = Sphere::new();
    s1.material = s1_material;

    let mut s2 = Sphere::new();
    s2.transform = Matrix::scale(0.5, 0.5, 0.5);

    let light = PointLight { position: Point { x: -10.0, y: 10.0, z: -10.0 }, intensity: Color { r: 1.0, g: 1.0, b: 1.0 } };

    assert_eq!(w.objects.len(), 2);
    assert_eq!(w.lights.len(), 1);
    assert_eq!(w.objects[0], s1);
    assert_eq!(w.objects[1], s2);
    assert_eq!(w.lights[0], light);
  }

  #[test]
  fn intersect_a_world_with_a_ray() {
    let w = World::default();
    let r = Ray { origin: Point { x: 0.0, y: 0.0, z: -5.0 }, direction: Vector { x: 0.0, y: 0.0, z: 1.0 } };

    let intersections = w.intersect(r);

    assert_eq!(intersections.len(), 4);
    assert_eq!(intersections[0].time, 4.0);
    assert_eq!(intersections[1].time, 4.5);
    assert_eq!(intersections[2].time, 5.5);
    assert_eq!(intersections[3].time, 6.0);
  }

  #[test]
  fn shading_an_intersection() {
    let w = World::default();
    let r = Ray { origin: Point { x: 0.0, y: 0.0, z: -5.0 }, direction: Vector { x: 0.0, y: 0.0, z: 1.0 } };

    let shape = w.objects[0];
    let i = Intersection { time: 4.0, object: &shape };

    let comps = i.prepare_computations(r);

    let c = w.shade_hit(comps);

    assert_eq!(c, Color { r: 0.38066, g: 0.47583, b: 0.2855 });
  }

  #[test]
  fn shading_an_intersection_from_the_inside() {
    let mut w = World::default();

    let light = PointLight { position: Point { x: 0.0, y: 0.25, z: 0.0 }, intensity: Color { r: 1.0, g: 1.0, b: 1.0 } };
    w.lights = vec![light];

    let r = Ray { origin: Point { x: 0.0, y: 0.0, z: 0.0 }, direction: Vector { x: 0.0, y: 0.0, z: 1.0 } };

    let shape = w.objects[1];
    let i = Intersection { time: 0.5, object: &shape };

    let comps = i.prepare_computations(r);

    let c = w.shade_hit(comps);

    assert_eq!(c, Color { r: 0.90498, g: 0.90498, b: 0.90498 });
  }

  #[test]
  fn color_when_ray_misses() {
    let w = World::default();
    let r = Ray { origin: Point { x: 0.0, y: 0.0, z: -5.0 }, direction: Vector { x: 0.0, y: 1.0, z: 0.0 } };

    let c = w.color_at(r);

    assert_eq!(c, Color { r: 0.0, g: 0.0, b: 0.0 });
  }

  #[test]
  fn color_when_ray_hits() {
    let w = World::default();
    let r = Ray { origin: Point { x: 0.0, y: 0.0, z: -5.0 }, direction: Vector { x: 0.0, y: 0.0, z: 1.0 } };

    let c = w.color_at(r);

    assert_eq!(c, Color { r: 0.38066, g: 0.47583, b: 0.2855 });
  }

  #[test]
  fn color_when_intersection_behind_ray() {
    let mut w = World::default();
    let r = Ray { origin: Point { x: 0.0, y: 0.0, z: 0.75 }, direction: Vector { x: 0.0, y: 0.0, z: -1.0 } };

    let mut s1_material = Material::new();
    s1_material.color = Color { r: 0.8, g: 1.0, b: 0.6 };
    s1_material.diffuse = 0.7;
    s1_material.specular = 0.2;
    s1_material.ambient = 1.0;
    let mut s1 = Sphere::new();
    s1.material = s1_material;

    let mut s2_material = Material::new();
    s2_material.ambient = 1.0;
    let mut s2 = Sphere::new();
    s2.transform = Matrix::scale(0.5, 0.5, 0.5);
    s2.material = s2_material;

    w.objects = vec![s1, s2];

    let c = w.color_at(r);

    assert_eq!(c, s2.material.color);
  }
}
