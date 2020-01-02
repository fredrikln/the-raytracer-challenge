use crate::canvas::Color;
use crate::vector::Vector;
use crate::point::Point;
use crate::point_light::PointLight;

#[derive(PartialEq, Debug, Copy, Clone)]
pub struct Material {
  pub color: Color,
  pub ambient: f64,
  pub diffuse: f64,
  pub specular: f64,
  pub shininess: f64,
}

impl Material {
  pub fn new() -> Material {
    Material {
      color: Color { r: 1.0, g: 1.0, b: 1.0 },
      ambient: 0.1,
      diffuse: 0.9,
      specular: 0.9,
      shininess: 200.0
    }
  }

  pub fn lighting(&self, light: PointLight, position: Point, eye_vector: Vector, normal: Vector, in_shadow: bool) -> Color {
    let effective_color = self.color * light.intensity;

    let lightv = (light.position - position).normalize();

    let ambient = effective_color * self.ambient;

    let light_dot_normal = lightv.dot(&normal);

    let mut diffuse: Color;
    let mut specular: Color;

    if light_dot_normal < 0.0 {
      diffuse = Color { r: 0.0, g: 0.0, b: 0.0 };
      specular = Color { r: 0.0, g: 0.0, b: 0.0 };
    } else {
      diffuse = effective_color * self.diffuse * light_dot_normal;

      let reflectv = (-lightv).reflect(normal);
      let reflect_dot_eye = reflectv.dot(&eye_vector);

      if reflect_dot_eye < 0.0 {
        specular = Color { r: 0.0, g: 0.0, b: 0.0 }
      } else {
        let factor = reflect_dot_eye.powf(self.shininess);
        specular = light.intensity * self.specular * factor;
      }
    }

    if in_shadow {
      specular = specular * 0.0;
      diffuse = diffuse * 0.0;
    }

    ambient + diffuse + specular
  }
}

#[cfg(test)]
mod tests {
  use crate::material::Material;
  use crate::canvas::Color;
  use crate::vector::Vector;
  use crate::point::Point;
  use crate::point_light::PointLight;

  #[test]
  fn the_default_material() {
    let m = Material::new();

    assert_eq!(m.color, Color { r: 1.0, g: 1.0, b: 1.0 });
    assert_eq!(m.ambient, 0.1);
    assert_eq!(m.diffuse, 0.9);
    assert_eq!(m.specular, 0.9);
    assert_eq!(m.shininess, 200.0);
  }

  #[test]
  fn eye_directly_between_light_and_surface() {
    let material = Material::new();
    let position = Point { x: 0.0, y: 0.0, z: 0.0 };
    let eye_vector = Vector { x: 0.0, y: 0.0, z: -1.0 };
    let normal = Vector { x: 0.0, y: 0.0, z: -1.0 };
    let light = PointLight { intensity: Color { r: 1.0, g: 1.0, b: 1.0 }, position: Point { x: 0.0, y: 0.0, z: -10.0 } };

    let light = material.lighting(light, position, eye_vector, normal, false);

    assert_eq!(light, Color { r: 1.9, g: 1.9, b: 1.9 });
  }

  #[test]
  fn eye_between_light_and_surface_eye_offset_45_degrees() {
    let material = Material::new();
    let position = Point { x: 0.0, y: 0.0, z: 0.0 };
    let eye_vector = Vector { x: 0.0, y: (2.0 as f64).sqrt() / 2.0, z: -((2.0 as f64).sqrt() / 2.0) };
    let normal = Vector { x: 0.0, y: 0.0, z: -1.0 };
    let light = PointLight { intensity: Color { r: 1.0, g: 1.0, b: 1.0 }, position: Point { x: 0.0, y: 0.0, z: -10.0 } };

    let light = material.lighting(light, position, eye_vector, normal, false);

    assert_eq!(light, Color { r: 1.0, g: 1.0, b: 1.0 });
  }

  #[test]
  fn eye_opposite_surface_light_offset_45_degrees() {
    let material = Material::new();
    let position = Point { x: 0.0, y: 0.0, z: 0.0 };
    let eye_vector = Vector { x: 0.0, y: 0.0, z: -1.0 };
    let normal = Vector { x: 0.0, y: 0.0, z: -1.0 };
    let light = PointLight { intensity: Color { r: 1.0, g: 1.0, b: 1.0 }, position: Point { x: 0.0, y: 10.0, z: -10.0 } };

    let light = material.lighting(light, position, eye_vector, normal, false);

    assert_eq!(light, Color { r: 0.7364, g: 0.7364, b: 0.7364 });
  }

  #[test]
  fn lighting_in_path_with_reflecting_vector() {
    let material = Material::new();
    let position = Point { x: 0.0, y: 0.0, z: 0.0 };
    let eye_vector = Vector { x: 0.0, y: -((2.0 as f64).sqrt() / 2.0), z: -((2.0 as f64).sqrt() / 2.0) };
    let normal = Vector { x: 0.0, y: 0.0, z: -1.0 };
    let light = PointLight { intensity: Color { r: 1.0, g: 1.0, b: 1.0 }, position: Point { x: 0.0, y: 10.0, z: -10.0 } };

    let light = material.lighting(light, position, eye_vector, normal, false);

    assert_eq!(light, Color { r: 1.6363961, g: 1.6363961, b: 1.6363961 });
  }

  #[test]
  fn light_behind_surface() {
    let material = Material::new();
    let position = Point { x: 0.0, y: 0.0, z: 0.0 };
    let eye_vector = Vector { x: 0.0, y: 0.0, z: -1.0 };
    let normal = Vector { x: 0.0, y: 0.0, z: -1.0 };
    let light = PointLight { intensity: Color { r: 1.0, g: 1.0, b: 1.0 }, position: Point { x: 0.0, y: 0.0, z: 10.0 } };

    let light = material.lighting(light, position, eye_vector, normal, false);

    assert_eq!(light, Color { r: 0.1, g: 0.1, b: 0.1 });
  }

  #[test]
  fn lighting_with_the_surface_in_shadow() {
    let material = Material::new();
    let position = Point { x: 0.0, y: 0.0, z: 0.0 };
    let eye_vector = Vector { x: 0.0, y: 0.0, z: -1.0 };
    let normal = Vector { x: 0.0, y: 0.0, z: -1.0 };
    let light = PointLight { intensity: Color { r: 1.0, g: 1.0, b: 1.0 }, position: Point { x: 0.0, y: 0.0, z: -10.0 } };
    let in_shadow = true;
    let light = material.lighting(light, position, eye_vector, normal, in_shadow);

    assert_eq!(light, Color { r: 0.1, g: 0.1, b: 0.1 });
  }
}
