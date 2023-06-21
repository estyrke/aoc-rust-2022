pub type Word = i64;

pub struct Memory {
  mem: Vec<Word>,
}

impl Memory {
  pub fn new(initial_mem: &[Word]) -> Self {
    Memory {
      mem: initial_mem.to_vec(),
    }
  }

  pub fn store(&mut self, addr: usize, value: Word) {
    if addr >= self.mem.len() {
      self.mem.resize(addr + 1, 0);
    }

    self.mem[addr] = value;
  }

  pub fn load(&self, addr: usize) -> Word {
    self.mem[addr as usize]
  }
}

pub struct Machine {
  ip: usize,
  bp: Word,
  pub mem: Memory,
  pub halted: bool,
}

impl Machine {
  fn load_next_op(&mut self) -> (Word, Word) {
    let op = self.mem.mem[self.ip];
    self.ip += 1;

    (op % 100, op / 100)
  }

  fn load_from_next(&mut self, param_modes: &mut Word) -> Word {
    let addr = self.mem.mem[self.ip];
    self.ip += 1;

    let value = match *param_modes % 10 {
      0 => self.mem.load(addr as usize),
      1 => addr,
      2 => self.mem.load((addr + self.bp) as usize),
      m => panic!("Invalid parameter mode {} at {}!", m, self.ip - 1),
    };
    *param_modes /= 10;

    return value;
  }

  fn store_next_value(&mut self, value: Word, param_modes: &mut Word) {
    let addr = self.mem.mem[self.ip];
    self.ip += 1;

    match *param_modes % 10 {
      0 => self.mem.store(addr as usize, value),
      2 => self.mem.store((addr + self.bp) as usize, value),
      m => panic!("Invalid parameter mode {} at {}!", m, self.ip - 1),
    };
    *param_modes /= 10;
  }

  pub fn run(&mut self, input: &Vec<Word>) -> Vec<Word> {
    let mut input_it = input.iter();
    let mut output = Vec::new();
    if self.halted {
      return [].to_vec();
    }
    loop {
      let saved_ip = self.ip;
      let (opcode, mut param_modes) = self.load_next_op();
      //println!("Opcode {} param_modes {} at {}", opcode, param_modes, self.ip - 1);
      match opcode {
        1 => {
          let param1 = self.load_from_next(&mut param_modes);
          let param2 = self.load_from_next(&mut param_modes);
          //println!("{:4} add {}, {}", saved_ip, param1, param2);
          self.store_next_value(param1 + param2, &mut param_modes);
        }
        2 => {
          let param1 = self.load_from_next(&mut param_modes);
          let param2 = self.load_from_next(&mut param_modes);
          //println!("{:4} mul {}, {}", saved_ip, param1, param2);
          self.store_next_value(param1 * param2, &mut param_modes);
        }
        3 => {
          if let Some(inp) = input_it.next() {
            //println!("{:4} inp {}", saved_ip, inp);
            self.store_next_value(*inp, &mut param_modes);
          } else {
            //println!("{:4} inp <paused waiting for input...>", saved_ip);
            self.ip = saved_ip;
            break;
          }
        }
        4 => {
          let param1 = self.load_from_next(&mut param_modes);
          //println!("{:4} outp, {}", saved_ip, param1);
          output.push(param1);
        }
        5 => {
          let cond = self.load_from_next(&mut param_modes);
          let dest = self.load_from_next(&mut param_modes);
          //println!("{:4} jnz {}, {}", saved_ip, cond, dest);
          if cond != 0 {
            self.ip = dest as usize;
          }
        }
        6 => {
          let cond = self.load_from_next(&mut param_modes);
          let dest = self.load_from_next(&mut param_modes);
          //println!("{:4} jz {}, {}", saved_ip, cond, dest);
          if cond == 0 {
            self.ip = dest as usize;
          }
        }
        7 => {
          let param1 = self.load_from_next(&mut param_modes);
          let param2 = self.load_from_next(&mut param_modes);
          //println!("{:4} cmplt {}, {}", saved_ip, param1, param2);
          self.store_next_value(if param1 < param2 { 1 } else { 0 }, &mut param_modes);
        }
        8 => {
          let param1 = self.load_from_next(&mut param_modes);
          let param2 = self.load_from_next(&mut param_modes);
          //println!("{:4} cmpeq {}, {}", saved_ip, param1, param2);
          self.store_next_value(if param1 == param2 { 1 } else { 0 }, &mut param_modes);
        }
        9 => {
          let param1 = self.load_from_next(&mut param_modes);
          //println!("{:4} base {}", saved_ip, param1);
          self.bp += param1;
        }
        99 => {
          //println!("{:4} stop", saved_ip);
          self.halted = true;
          break;
        }
        _ => {
          panic!("Illegal instruction {} at {}", opcode, saved_ip)
        }
      }
    }
    return output;
  }

  pub fn new(program: &str) -> Self {
    let initial_mem = &program
      .split(",")
      .map(|s| -> Word { s.trim().parse().unwrap() })
      .collect::<Vec<Word>>();

    return Machine {
      ip: 0,
      bp: 0,
      halted: false,
      mem: Memory::new(initial_mem),
    };
  }

  pub fn reset(&mut self) {
    self.ip = 0;
    self.bp = 0;
    self.halted = false;
  }
}
