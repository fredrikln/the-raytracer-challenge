pub const EPSILON: f32 = 0.00001;

pub fn equal(a: f32, b: f32) -> bool {
  if (a - b).abs() < EPSILON { return true; }

  false
}
