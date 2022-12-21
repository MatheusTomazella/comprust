type RamAddress = usize;
type RamValue = i32;

pub const RAM_SIZE: usize = 100;

#[derive(Debug)]
pub struct Ram {
  tape: Vec<RamValue>,
}

impl Ram {
  pub fn read (&self, address: RamAddress) -> RamValue {
    if address >= RAM_SIZE { panic!(); }
    self.tape[address]
  }

  pub fn write (&mut self, address: RamAddress, value: RamValue) {
    if address >= RAM_SIZE { panic!(); }
    self.tape[address] = value;
  }
}

pub fn init_ram () -> Ram {
  Ram {
    tape: vec![0; RAM_SIZE]
  }
}

#[cfg(test)]
mod tests {
  use super::{init_ram, RAM_SIZE};

  #[test]
  fn should_start_all_0 () {
    let ram = init_ram();
    for i in ram.tape {
      if i != 0 { panic!() }
    }
  }
  #[test]
  fn should_read_0_at_0 () {
    let ram = init_ram();
    assert_eq!(ram.read(0), 0);
  }
  #[test]
  fn should_read_all_0 () {
    let ram = init_ram();
    for i in 0..RAM_SIZE {
      assert_eq!(ram.read(i), 0);
    }
  }
  #[test]
  fn should_write_to_0 () {
    let mut ram = init_ram();
    ram.write(0, 43);
    assert_eq!(ram.tape[0], 0);
  }
}
