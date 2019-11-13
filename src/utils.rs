pub fn equal(a: f32, b: f32) -> bool {
  let epsilon = 0.000001;

  if (a - b).abs() < epsilon { return true; }

  false
}
