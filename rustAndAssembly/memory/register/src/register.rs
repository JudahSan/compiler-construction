pub struct Bit {
  in_value: u8,
  load: u8,
  out_value: u8,
}

impl Bit {
  pub fn new() -> Bit {
    Bit {
      in_value: 0,
      load: 0,
      out_value: 0,
    }
  }

  pub fn set_input(&mut self, value: u8) {
    self.in_value = value;
  }

  pub fn set_load(&mut self, value: u8) {
    self.load = value;
  }

  pub fn update(&mut self) {
    if self.load == 1{
      self.out_value = self.in_value;
    }
  }

  pub fn get_output(&self) -> u8 {
    self.out_value
  }
}