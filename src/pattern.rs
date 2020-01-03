use crate::point::Point;
use crate::matrix::Matrix;
use crate::canvas::Color;
use crate::object::{Object,Intersectable};

pub trait PatternTrait {
  fn color_at(&self, p: Point) -> Color;
  fn color_at_object(&self, o: &Object, p: Point) -> Color;
}

// STRIPES
#[derive(PartialEq, Debug, Copy, Clone)]
pub struct StripedPattern {
  pub a: Color,
  pub b: Color,
  pub transform: Matrix,
}

impl StripedPattern {
  pub fn new(a: Color, b: Color) -> StripedPattern {
    StripedPattern {
      a,
      b,
      transform: Matrix::identity(),
    }
  }

  pub fn default() -> StripedPattern {
    StripedPattern::new(Color { r: 1.0, g: 1.0, b: 1.0 }, Color { r: 0.0, g: 0.0, b: 0.0 })
  }
}

impl PatternTrait for StripedPattern {
  fn color_at(&self, p: Point) -> Color {
    if p.x.floor() % 2.0 == 0.0 {
      self.a
    } else {
      self.b
    }
  }

  fn color_at_object(&self, o: &Object, p: Point) -> Color {
    let object_point = o.transform().inverse().unwrap() * p;
    let pattern_point = self.transform.inverse().unwrap() * object_point;

    self.color_at(pattern_point)
  }
}

// GRADIENT
#[derive(PartialEq, Debug, Copy, Clone)]
pub struct GradientPattern {
  pub a: Color,
  pub b: Color,
  pub transform: Matrix,
}

impl GradientPattern {
  pub fn new(a: Color, b: Color) -> GradientPattern {
    GradientPattern {
      a,
      b,
      transform: Matrix::identity(),
    }
  }

  pub fn default() -> GradientPattern {
    GradientPattern::new(Color { r: 1.0, g: 1.0, b: 1.0 }, Color { r: 0.0, g: 0.0, b: 0.0 })
  }
}

impl PatternTrait for GradientPattern {
  fn color_at(&self, p: Point) -> Color {
    let distance = self.b - self.a;
    let fraction = p.x - p.x.floor();

    self.a + distance * fraction
  }

  fn color_at_object(&self, o: &Object, p: Point) -> Color {
    let object_point = o.transform().inverse().unwrap() * p;
    let pattern_point = self.transform.inverse().unwrap() * object_point;

    self.color_at(pattern_point)
  }
}

// RING

// CHECKER

// ENUM
#[derive(PartialEq, Debug, Copy, Clone)]
pub enum Pattern {
  Stripe(StripedPattern),
  Gradient(GradientPattern),
}

impl PatternTrait for Pattern {
  fn color_at(&self, p: Point) -> Color {
    match *self {
      Pattern::Stripe(ref s) => s.color_at(p),
      Pattern::Gradient(ref g) => g.color_at(p),
    }
  }

  fn color_at_object(&self, o: &Object, p: Point) -> Color {
    match *self {
      Pattern::Stripe(ref s) => s.color_at_object(o, p),
      Pattern::Gradient(ref g) => g.color_at_object(o, p),
    }
  }
}

#[cfg(test)]
mod tests {
  use crate::pattern::{PatternTrait,StripedPattern,GradientPattern};
  use crate::canvas::Color;
  use crate::sphere::Sphere;
  use crate::object::Object;
  use crate::point::Point;
  use crate::matrix::Matrix;

  #[test]
  fn creating_a_striped_pattern() {
    let black = Color { r: 0.0, g: 0.0, b: 0.0 };
    let white = Color { r: 1.0, g: 1.0, b: 1.0 };
    let mut pa = StripedPattern::new(white, black);

    assert_eq!(pa.a, white);
    assert_eq!(pa.b, black);
  }

  #[test]
  fn stripe_is_contant_in_y() {
    let black = Color { r: 0.0, g: 0.0, b: 0.0 };
    let white = Color { r: 1.0, g: 1.0, b: 1.0 };

    let mut pa = StripedPattern::new(white, black);

    assert_eq!(pa.color_at(Point { x: 0.0, y: 0.0, z: 0.0 }), white);
    assert_eq!(pa.color_at(Point { x: 0.0, y: 1.0, z: 0.0 }), white);
    assert_eq!(pa.color_at(Point { x: 0.0, y: 2.0, z: 0.0 }), white);
  }

  #[test]
  fn stripe_is_contant_in_z() {
    let black = Color { r: 0.0, g: 0.0, b: 0.0 };
    let white = Color { r: 1.0, g: 1.0, b: 1.0 };

    let mut pa = StripedPattern::new(white, black);

    assert_eq!(pa.color_at(Point { x: 0.0, y: 0.0, z: 0.0 }), white);
    assert_eq!(pa.color_at(Point { x: 0.0, y: 0.0, z: 1.0 }), white);
    assert_eq!(pa.color_at(Point { x: 0.0, y: 0.0, z: 2.0 }), white);
  }
  #[test]
  fn stripe_alternates_in_x() {
    let black = Color { r: 0.0, g: 0.0, b: 0.0 };
    let white = Color { r: 1.0, g: 1.0, b: 1.0 };

    let mut pa = StripedPattern::new(white, black);

    assert_eq!(pa.color_at(Point { x: 0.0, y: 0.0, z: 0.0 }), white);
    assert_eq!(pa.color_at(Point { x: 0.0, y: 0.0, z: 0.0 }), white);
    assert_eq!(pa.color_at(Point { x: 1.0, y: 0.0, z: 0.0 }), black);
    assert_eq!(pa.color_at(Point { x: -0.1, y: 0.0, z: 0.0 }), black);
    assert_eq!(pa.color_at(Point { x: -1.0, y: 0.0, z: 0.0 }), black);
    assert_eq!(pa.color_at(Point { x: -1.1, y: 0.0, z: 0.0 }), white);
  }


  #[test]
  fn stripes_with_object_transformation() {
    let mut sp = Sphere::new();
    sp.transform = Matrix::scale(2.0, 2.0, 2.0);
    let s = Object::Sphere(sp);

    let pa = StripedPattern::default();

    let p = Point { x: 1.5, y: 0.0, z: 0.0 };

    assert_eq!(pa.color_at_object(&s, p), Color { r: 1.0, g: 1.0, b: 1.0 });
  }

  #[test]
  fn stripes_with_pattern_transformation() {
    let sp = Sphere::new();
    let s = Object::Sphere(sp);

    let mut pa = StripedPattern::default();
    pa.transform = Matrix::scale(2.0, 2.0, 2.0);

    let p = Point { x: 1.5, y: 0.0, z: 0.0 };

    assert_eq!(pa.color_at_object(&s, p), Color { r: 1.0, g: 1.0, b: 1.0 });
  }

  #[test]
  fn stripes_with_object_and_pattern_transformation() {
    let mut sp = Sphere::new();
    sp.transform = Matrix::scale(2.0, 2.0, 2.0);
    let s = Object::Sphere(sp);

    let mut pa = StripedPattern::default();
    pa.transform = Matrix::translate(0.5, 0.0, 0.0);

    let p = Point { x: 2.5, y: 0.0, z: 0.0 };

    assert_eq!(pa.color_at_object(&s, p), Color { r: 1.0, g: 1.0, b: 1.0 });
  }

  #[test]
  fn gradient_pattern_linearly_interpolates_between_black_and_white() {
    let black = Color { r: 0.0, g: 0.0, b: 0.0 };
    let white = Color { r: 1.0, g: 1.0, b: 1.0 };

    let mut pa = GradientPattern::new(white, black);

    assert_eq!(pa.color_at(Point { x: 0.0, y: 0.0, z: 0.0 }), white);
    assert_eq!(pa.color_at(Point { x: 0.25, y: 0.0, z: 0.0 }), Color { r: 0.75, g: 0.75, b: 0.75 });
    assert_eq!(pa.color_at(Point { x: 0.50, y: 0.0, z: 0.0 }), Color { r: 0.50, g: 0.50, b: 0.50 });
    assert_eq!(pa.color_at(Point { x: 0.75, y: 0.0, z: 0.0 }), Color { r: 0.25, g: 0.25, b: 0.25 });
  }
}
