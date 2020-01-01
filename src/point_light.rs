use crate::canvas::Color;
use crate::point::Point;

#[derive(PartialEq, Debug, Copy, Clone)]
pub struct PointLight {
  pub intensity: Color,
  pub position: Point,
}

#[cfg(test)]
mod tests {
  use crate::canvas::Color;
  use crate::point::Point;
  use crate::point_light::PointLight;

  #[test]
  fn a_point_light_has_position_and_intensity() {
    let intensity = Color { r: 1.0, g: 1.0, b: 1.0 };
    let position = Point { x: 0.0, y: 0.0, z: 0.0 };

    let light = PointLight { intensity, position };

    assert_eq!(light.position, position);
    assert_eq!(light.intensity, intensity);
  }
}
