pub struct ALU {
  pub x: [bool; 16],
  pub y: [bool; 16],
  pub zx: bool,
  pub nx: bool,
  pub zy: bool,
  pub ny: bool,
  pub f: bool,
  pub no: bool,
  pub out: [bool; 16],
  pub zr: bool,
  pub ng: bool,
}

impl ALU {
  pub fn new() -> ALU {
      ALU {
          x: [false; 16],
          y: [false; 16],
          zx: false,
          nx: false,
          zy: false,
          ny: false,
          f: false,
          no: false,
          out: [false; 16],
          zr: false,
          ng: false,
      }
  }

  pub fn compute(&mut self) {
      // Apply input control signals
      if self.zx {
          self.x = [false; 16];
      }
      if self.nx {
          self.x = self.twos_complement(self.x);
      }
      if self.zy {
          self.y = [false; 16];
      }
      if self.ny {
          self.y = self.twos_complement(self.y);
      }

      // Perform addition or bitwise AND based on the control signal f
      if self.f {
          self.out = self.add(self.x, self.y);
      } else {
          self.out = self.bitwise_and(self.x, self.y);
      }

      // Apply output negation if necessary
      if self.no {
          self.out = self.twos_complement(self.out);
      }

      // Set zero and negative flags
      self.zr = self.is_zero(self.out);
      self.ng = self.is_negative(self.out);
  }

  fn add(&self, a: [bool; 16], b: [bool; 16]) -> [bool; 16] {
      let mut result = [false; 16];
      let mut carry = false;

      for i in (0..16).rev() {
          let sum = a[i] ^ b[i] ^ carry;
          carry = (a[i] && b[i]) || (a[i] && carry) || (b[i] && carry);
          result[i] = sum;
      }

      result
  }

  fn bitwise_and(&self, a: [bool; 16], b: [bool; 16]) -> [bool; 16] {
      let mut result = [false; 16];

      for i in 0..16 {
          result[i] = a[i] && b[i];
      }

      result
  }

  fn twos_complement(&self, value: [bool; 16]) -> [bool; 16] {
      let inverted = self.bitwise_not(value);
      let one = self.generate_one();

      self.add(inverted, one)
  }

  fn generate_one(&self) -> [bool; 16] {
    let mut one = [false; 16];
    one[15] = true;
    one
}

  fn bitwise_not(&self, value: [bool; 16]) -> [bool; 16] {
      let mut result = [false; 16];

      for i in 0..16 {
          result[i] = !value[i];
      }

      result
  }

  fn is_zero(&self, value: [bool; 16]) -> bool {
      for i in 0..16 {
          if value[i] {
              return false;
          }
      }
      true
  }

  fn is_negative(&self, value: [bool; 16]) -> bool {
      value[0]
  }
}
