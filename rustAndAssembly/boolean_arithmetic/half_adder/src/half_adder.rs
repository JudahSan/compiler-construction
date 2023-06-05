pub struct HalfAdder {
  pub a: bool,
  pub b: bool,
  pub sum: bool,
  pub carry: bool,
}

impl HalfAdder {
  pub fn new(a: bool, b: bool) -> HalfAdder {
    HalfAdder {
      a,
      b,
      sum: false,
      carry: false,

    }
  }

  pub fn compute(&mut self ) {
    self.sum = self.a ^ self.b;
    self.carry = self.a && self.b;
  }
}