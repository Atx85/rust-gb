use crate::instruction_set::*;
use crate::Bus;

#[derive(Clone, Debug)]
pub enum RegPair {
  AF, BC, DE, HL}
#[derive(Clone)]
pub enum Reg {
  A, F, B, C, D, E, H, L
}
#[derive(Debug)]
pub enum Flag {
  Z, N, H, C
}
pub enum RegVal {
  Byte(u8),
  Word(u16),
}
pub struct Cpu {
    a:u8, f:u8, b:u8, c: u8, d:u8,
    e:u8, h:u8, l:u8,
    pub pc:u16, pub sp:u16, bus: Option<crate::Bus>
}
impl Cpu {
  pub fn init() -> Self {
    Self {
      a: 0x01,
      f: 0xB0,
      b: 0x00,
      c: 0x13,
      d: 0x00,
      e: 0xD8,
      h: 0x01,
      l: 0x4D,
      sp: 0xFFFE,
      pc: 0x0100,
      bus: None
    }
  }

  pub fn attach_bus(&mut self, bus: Bus) {
    self.bus = Some(bus);
  }

  pub fn fetch_word(&mut self) -> u16 {
    let h = self.fetch_byte();
    let l = self.fetch_byte();
    u16::from_be_bytes([h, l])
  }

  pub fn fetch_byte(&mut self) -> u8 {
    match self.bus {
      Some(ref mut  bus) => {
        let value = bus.read_byte(self.pc);
        self.pc = self.pc.wrapping_add(1);
        value
      },
      _ => panic!("Bus is not set on Cpu!\n"),
    }
  }
  pub fn write_word(&mut self, addr: u16, val: u16) {
    let [h, l] = val.to_be_bytes();  
    match self.bus {
      Some(ref mut bus) => {
        bus.write_byte(addr, h);
        bus.write_byte(addr + 1, l);
      },
      _ => panic!("Bus is not se on Cpu!\n"),
    }
  }
  pub fn step(&mut self) {
    let inst = Inst::decode(self);
    self.execute(inst);
  }

  pub fn try_write_byte(&mut self, addr: u16, val: u8) {
    match self.bus {
      Some(ref mut bus) => bus.write_byte(addr, val),
      _ => panic!("Bus is not set on Cpu!\n"),
    }
  }

  fn execute(&mut self, inst: InstRes) {
    match inst.inst {
      Inst::Nop => {},
      Inst::LD_R_D8(reg, val) => {
        self.set_reg(reg, RegVal::Byte(val));
      },
      Inst::LD_HL_D8(val) => { 
        let h= self.get_reg(Reg::H);
        let l= self.get_reg(Reg::L);
        self.try_write_byte(u16::from_be_bytes([h, l]), val);
      },
      Inst::LD_RR_D16(reg, val) => {
        self.set_reg_pair_word(reg, val);
      },
      Inst::LD_SP_D16(val) => {
        self.sp = val;
      },
      Inst::LD_SP_HL() => {
        self.sp = self.get_reg_pair(RegPair::HL);
      },
      Inst::LD_HL_SP(offset) => {
        // LD HL, SP+r8:
        // The 8-bit immediate is sign-extended and added to SP to calculate HL.
        // H and C flags are calculated using unsigned addition of the low 4 bits
        // and low 8 bits of SP and r8, respectively.
        // Z and N are always reset.
        let val = self.sp.wrapping_add((offset as i8 as i16) as u16);
        let sp = self.sp;
        self.set_reg_pair_word(RegPair::HL, val);
        self.set_flag(Flag::Z, false);
        self.set_flag(Flag::N, false);
        let h = (sp & 0x0f) + (offset as u16 & 0x0f) > 0x0f;
        let c = ((sp & 0xff) + (offset as u16)) > 0xff;
        self.set_flag(Flag::H, h) ;
        self.set_flag(Flag::C, c);
        // sp = 0x001, r8=0x0f HL = 0x0010 h:1, c 0
        // sp=0x00f0, r8 = 0x20, HL = 0x0110 h:0 c:1
      },
      Inst::LD_A16_SP(addr) => {
        self.write_word(addr, self.sp);
      }

    }
  }
  fn get_flag(&mut self, flag: Flag) -> bool {
    let bit_index;  
    match flag {
      Flag::C => { bit_index = 4; }
      Flag::H => { bit_index = 5; }   
      Flag::N => { bit_index = 6; }
      Flag::Z => { bit_index = 7; }
    }
    ((self.f >> bit_index) & 0x01) == 1
  }

  fn set_flag(
    &mut self, 
    flag: Flag, 
    val: bool
  ) {
    // z, n ,h, c
    // 7, 6, 5, 4
    // 3-0 always 0, enforce it with & 0xF0;

    let bit_index;  
    match flag {
      Flag::C => { bit_index = 4; }
      Flag::H => { bit_index = 5; }   
      Flag::N => { bit_index = 6; }
      Flag::Z => { bit_index = 7; }
    }
    if val {
     self.f = (self.f | 1 << bit_index) & 0xF0;
    } else {
     self.f = (self.f & !(1 << bit_index)) & 0xF0;
    }
    println!("Setting {:?} to be {}", bit_index, val);
  }
  fn get_reg(&mut self, reg: Reg) -> u8 {
         let val: u8;
         match reg {
          Reg::A => { val = self.a; },
          Reg::F => { val = self.f; },
          Reg::B => { val = self.b; },
          Reg::C => { val = self.c; },
          Reg::D => { val = self.d; },
          Reg::E => { val = self.e; },
          Reg::H => { val = self.h; },
          Reg::L => { val = self.l; },
        }
         val
  }
  fn set_reg(&mut self, reg: Reg, val: RegVal) {
    let value: u8;
    match val {
      RegVal::Word(_) => println!("Expected Byte, got Word"),
      RegVal::Byte(value) => {
        match reg {
          Reg::A => { self.a = value; },
          Reg::F => { self.f = value; },
          Reg::B => { self.b = value; },
          Reg::C => { self.c = value; },
          Reg::D => { self.d = value; },
          Reg::E => { self.e = value; },
          Reg::H => { self.h = value; },
          Reg::L => { self.l = value; },
        }
      }
    }
  }
  fn get_reg_pair(&mut self, reg: RegPair) -> u16 {
  let low: u8; 
  let high: u8;
   match reg {
      RegPair::AF => { high = self.a; low = self.f;  }
      RegPair::BC => { high = self.b; low = self.c;  }
      RegPair::DE => { high = self.d; low = self.e;  }
      RegPair::HL => { high = self.h; low = self.l;  }
    }
   u16::from_be_bytes([high, low])
  }
  fn set_reg_pair_word(&mut self,reg: RegPair, val:u16) {
    let [h, l] = val.to_be_bytes();
    self.set_reg_pair(reg, RegVal::Byte(h), Some(RegVal::Byte(l)));
  }
  // af, bc, de, hl
  // set_reg_pair(RegPair::AF, RegVal::Byte(0x12), Some(RegVal::Byte(0x34)));
  // set_reg_pair(RegPair::AF, RegVal::Word(0x1234), None);
  fn set_reg_pair(&mut self, reg: RegPair, high_val: RegVal, low_val: Option<RegVal>){
    let mut high: u8 = 0; 
    let mut low: u8 = 0;

    if matches!(high_val, RegVal::Byte(_)) {
      if let RegVal::Byte(v) = high_val { high = v; }      
      let low_val = low_val.unwrap();
      if let RegVal::Byte(v) = low_val { low = v; }
    } else {
      if let RegVal::Word(v) = high_val { [high, low] = v.to_be_bytes(); }
    }
    match reg {
      RegPair::AF => { self.a = high; self.f = low;  }
      RegPair::BC => { self.b = high; self.c = low;  }
      RegPair::DE => { self.d = high; self.e = low;  }
      RegPair::HL => { self.h = high; self.l = low;  }
    }
  }
}




#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_set_reg_pairs() {
    let mut  cpu = Cpu::init();
    cpu.set_reg_pair(RegPair::AF, RegVal::Byte(0x12), Some(RegVal::Byte(0x34)));
    assert_eq!(cpu.a, 0x12);
    assert_eq!(cpu.f, 0x34);
    cpu.set_reg_pair(RegPair::AF, RegVal::Word(0x0000), None);
    assert_eq!(cpu.a, 0x00);
    assert_eq!(cpu.f, 0x00);
    cpu.set_reg_pair(RegPair::AF, RegVal::Word(0x1234), None);
    assert_eq!(cpu.a, 0x12);
    assert_eq!(cpu.f, 0x34);
  }
  #[test]
  fn test_set_flag() {
    let mut  cpu = Cpu {
      sp: 0x0000, pc: 0x0000, bus: None,
      a: 0, f: 0, b: 0, c: 0, d: 0, e: 0, h: 0, l: 0
      };
    cpu.set_flag(Flag::C, true);
    assert_eq!(cpu.f, 0x10);
    cpu.f = 0;
    cpu.set_flag(Flag::H, true);
    assert_eq!(cpu.f, 0x20);

    cpu.f = 0;
    cpu.set_flag(Flag::N, true);
    assert_eq!(cpu.f, 0x40);

    cpu.f = 0;
    cpu.set_flag(Flag::Z, true);
    assert_eq!(cpu.f, 0x80);
  }

  #[test]
  fn test_get_pair () {
    let mut cpu = Cpu::init();
    let regs = vec![
      RegPair::AF,
      RegPair::BC,
      RegPair::DE,
      RegPair::HL,
    ]; 
    for reg in &regs {
      cpu.set_reg_pair(reg.clone() , RegVal::Word(0x1234), None);
      let val = cpu.get_reg_pair(reg.clone());
      assert_eq!(val, 0x1234);
    }
  }

  #[test]
  fn test_execute_ld_hl_d8() {
    let mut cpu = Cpu::init();
    let mut bus = Bus::new();
    let program = [0x36, 0x42];
    bus.load_program(&program, 0x0100);
    cpu.set_reg(Reg::H, RegVal::Byte(0x12));
    cpu.set_reg(Reg::L, RegVal::Byte(0x34));
    cpu.attach_bus(bus);
    cpu.step();
    assert_eq!(cpu.bus.expect("Bus not attached!").read_byte(0x1234), 0x42);
  }

  #[test]
  fn test_execute_ld_r_d8() {
    let mut cpu = Cpu::init();
    let mut regs: Vec<(Reg, u8)> = Vec::new();
    let mut bus = Bus::new();
    let program = [
      0x06,0x42,
      0x0E,0x42,
      0x16,0x42,
      0x1E,0x42,
      0x26,0x42,
      0x2E,0x42,
      0x3E,0x42,
   ];
    bus.load_program(&program, 0x0100);
    regs.push((Reg::B, 0x06));
    regs.push((Reg::C, 0x0E));
    regs.push((Reg::D, 0x16));
    regs.push((Reg::E, 0x1E));
    regs.push((Reg::H, 0x26));
    regs.push((Reg::L, 0x2E));
    regs.push((Reg::A, 0x3E));
    cpu.attach_bus(bus);
    for reg in &regs {
      cpu.set_reg(reg.clone().0, RegVal::Byte(0x42));
      cpu.step();
      assert_eq!(cpu.get_reg(reg.clone().0), 0x42);
    }
  }

  #[test]
  fn test_execute_ld_hl_sp() {
    #[derive(Debug)]
    struct TestCase {
      sp: u16,
      r8: u8,
      hl: u16,
      z: bool,
      n: bool,
      h: bool,
      c: bool
    }
    let tests = vec![
      TestCase { sp: 0x0000, r8: 0x00, hl: 0x0000, z: false, n: false, h: false, c: false },
      TestCase { sp: 0x0001, r8: 0x01, hl: 0x0002, z: false, n: false, h: false, c: false },
      TestCase { sp: 0x0001, r8: 0x0F, hl: 0x0010, z: false, n: false, h: true,  c: false },
      TestCase { sp: 0x0001, r8: 0xFF, hl: 0x0000, z: false, n: false, h: true,  c: true  },
      TestCase { sp: 0x00F0, r8: 0x20, hl: 0x0110, z: false, n: false, h: false, c: true  },
      TestCase { sp: 0x00FF, r8: 0x01, hl: 0x0100, z: false, n: false, h: true,  c: true  },
      TestCase { sp: 0x0002, r8: 0xFE, hl: 0x0000, z: false, n: false, h: true, c: true },
      TestCase { sp: 0x0002, r8: 0xFC, hl: 0xFFFE, z: false, n: false, h: false, c: false },
      TestCase { sp: 0xFFF0, r8: 0x10, hl: 0x0000, z: false, n: false, h: false, c: true  },
      TestCase { sp: 0xFFFF, r8: 0x01, hl: 0x0000, z: false, n: false, h: true,  c: true  },
      TestCase { sp: 0x1234, r8: 0x7F, hl: 0x12B3, z: false, n: false, h: true, c: false },
      TestCase { sp: 0x1234, r8: 0x80, hl: 0x11B4, z: false, n: false, h: false, c: false },
    ];
    let mut  cpu = Cpu::init();
    let mut bus = Bus::new();
    let program = [
      0xf8,0x00,
      0xf8,0x01,
      0xf8,0x0F,
      0xf8,0xFF,
      0xf8,0x20,
      0xf8,0x01,
      0xf8,0xFE,
      0xf8,0xFC,
      0xf8,0x10,
      0xf8,0x01,
      0xf8,0x7F,
      0xf8,0x80,
   ];
    bus.load_program(&program, 0x0100);
    cpu.attach_bus(bus);
    for test in &tests {
      cpu.sp = test.sp;
      assert_eq!(cpu.sp, test.sp);
      cpu.step();
      println!(
        "H: {:?} C: {:?} HL: {:04X} | H: {:?} C: {:?} HL: {:04X} ", 
        cpu.get_flag(Flag::H),
        cpu.get_flag(Flag::C),
        cpu.get_reg_pair(RegPair::HL),
        test.h, test.c, test.hl);
      assert_eq!(cpu.get_reg_pair(RegPair::HL), test.hl);
      assert_eq!(cpu.get_flag(Flag::Z), test.z);
      assert_eq!(cpu.get_flag(Flag::N), test.n);
      assert_eq!(cpu.get_flag(Flag::H), test.h);
      assert_eq!(cpu.get_flag(Flag::C), test.c);
    }
  }

}



/*
 * LD HL, SP+r8 (0xF8)
 *
 * SP     r8    HL      Z N H C
 * --------------------------------
 * 0000   00    0000    0 0 0 0
 * 0001   01    0002    0 0 0 0
 * 0001   0F    0010    0 0 1 0
 * 0001   FF    0000    0 0 1 1
 * 00F0   20    0110    0 0 0 1
 * 00FF   01    0100    0 0 1 1
 * 0002   FE    0000    0 0 0 0
 * 0002   FC    FFFE    0 0 0 0
 * FFF0   10    0000    0 0 0 1
 * FFFF   01    0000    0 0 1 1
 * 1234   7F    12B3    0 0 0 0
 * 1234   80    11B4    0 0 0 0
 *
 * r8 is interpreted as signed for the result,
 * but unsigned for H and C flag calculations.
 */

