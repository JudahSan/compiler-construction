pub struct Adder {
  pub a: bool,
  pub b: bool,
  pub sum: bool,
  pub carry: bool,
}

impl Adder {
  pub fn new() -> Adder {
    Adder {
      a: false,
      b: false,
      sum: false,
      carry: false,
    }
  }

  pub fn compute(&mut self) {
    self.sum = self.a ^ self.b;
    self.carry = self.a && self.b;
  }
}