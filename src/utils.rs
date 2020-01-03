pub const EPSILON: f64 = 0.00001;
pub const MAX_STEPS: u8 = 8;

pub fn equal(a: f64, b: f64) -> bool {
  if (a - b).abs() < EPSILON { return true; }

  false
}
