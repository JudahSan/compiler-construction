#[cfg(test)]
mod tests {
  use crate::half_adder::HalfAdder;

  #[test]
  fn test_half_adder() {
    // Test case 1: Inputs (a=0, b=0)
    let mut half_adder = HalfAdder::new(false, false);
    half_adder.compute();
    assert_eq!(half_adder.sum, false);
    assert_eq!(half_adder.carry, false);

    // Test case 2: Inputs (a=0, b=1)
    let mut half_adder = HalfAdder::new(false, true);
    half_adder.compute();
    assert_eq!(half_adder.sum, true);
    assert_eq!(half_adder.carry, false);

    // Test case 3: Inputs (a=1, b=0)
    let mut half_adder = HalfAdder::new(true, false);
    half_adder.compute();
    assert_eq!(half_adder.sum, true);
    assert_eq!(half_adder.carry, false);

    // Test case 4: Inputs (a=1, b=1)
    let mut half_adder = HalfAdder::new(true, true);
    half_adder.compute();
    assert_eq!(half_adder.sum, false);
    assert_eq!(half_adder.carry, true);
  }
}