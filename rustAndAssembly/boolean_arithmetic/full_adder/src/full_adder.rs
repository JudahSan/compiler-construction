pub struct FullAdder {
  pub a: bool,
  pub b: bool,
  pub c: bool,
  pub sum: bool,
  pub carry: bool,
}

impl FullAdder {
  pub fn new() -> FullAdder {
      FullAdder {
          a: false,
          b: false,
          c: false,
          sum: false,
          carry: false,
      }
  }

  pub fn compute(&mut self) {
      let mut half_adder_1 = HalfAdder::new(self.a, self.b);
      half_adder_1.compute();

      let mut half_adder_2 = HalfAdder::new(half_adder_1.sum, self.c);
      half_adder_2.compute();

      self.sum = half_adder_2.sum;
      self.carry = half_adder_1.carry || half_adder_2.carry;
  }
}

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

  pub fn compute(&mut self) {
      self.sum = self.a ^ self.b;
      self.carry = self.a && self.b;
  }
}
