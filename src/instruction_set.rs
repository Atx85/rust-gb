use crate::cpu::Cpu;
use crate::cpu::{Reg, RegPair};


#[allow(non_camel_case_types)]
pub enum Inst {
  Nop, 
//  LD_R_R,
  LD_R_D8(Reg,u8),
  LD_HL_D8(u8),
  LD_RR_D16(RegPair, u16),
  LD_SP_D16(u16),
  LD_SP_HL(),
  LD_HL_SP(u8),
  LD_A16_SP(u16),
}
pub struct InstRes {
  pub inst: Inst,
  pub opcode: u8,
  pub tcycles: u8,
}
impl Inst {
  pub fn decode(cpu: &mut Cpu) -> InstRes {
    // Read opcode and operands
    let op_code = cpu.fetch_byte();
    // Return the appropriate instruction variant
    let (inst, tcycles) = match op_code {
      0x00 => (Inst::Nop, 4),
      0x01 => (Inst::LD_RR_D16( RegPair::BC, cpu.fetch_word() ), 12),
      0x06 => (Inst::LD_R_D8( Reg::B, cpu.fetch_byte() ), 8),
      0x0E => (Inst::LD_R_D8( Reg::C, cpu.fetch_byte() ), 8),
      0x11 => (Inst::LD_RR_D16( RegPair::DE, cpu.fetch_word() ), 12),
      0x16 => (Inst::LD_R_D8( Reg::D, cpu.fetch_byte() ), 8),
      0x1E => (Inst::LD_R_D8( Reg::E, cpu.fetch_byte() ),8),
      0x21 => (Inst::LD_RR_D16( RegPair::HL, cpu.fetch_word() ),12),
      0x26 => (Inst::LD_R_D8( Reg::H, cpu.fetch_byte() ), 8),
      0x2E => (Inst::LD_R_D8( Reg::L, cpu.fetch_byte() ), 8),
      0x31 => (Inst::LD_SP_D16(cpu.fetch_word() ), 12),
      0x3E => (Inst::LD_R_D8( Reg::A, cpu.fetch_byte() ), 8),
      0x36 => (Inst::LD_HL_D8( cpu.fetch_byte() ),12), // puts imm byte in HL
      0xF8 => (Inst::LD_HL_SP( cpu.fetch_byte()), 12),
      0xF9 => (Inst::LD_SP_HL(), 8),
      _ => panic!("Unknown op code {:?} \n", op_code)
    };
    InstRes{ inst: inst, opcode: op_code, tcycles, }
  }
}


