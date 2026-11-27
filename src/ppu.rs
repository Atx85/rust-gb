#![allow(unused_variables)]
#![allow(dead_code)]

pub struct Ppu {
  vram: [u8; 8192],
  oam: [u8; 1280],

}

impl Ppu {
  fn new() -> Self {
    let vram = [0 as u8; 8192];
    let oam = [0 as u8; 1280];
    Self {vram, oam}
  }
  pub fn write_vram_byte(&mut self, addr: u16, val: u8) {
    self.vram[addr as usize] = val;
  }
  pub fn read_vram_byte(&mut self, addr: u16) -> u8 {
    self.vram[addr as usize]
  }
  pub fn write_oam_byte(&mut self, addr: u16, val: u8) {
    self.vram[addr as usize] = val;
  }
  pub fn read_oam_byte(&mut self, addr: u16) -> u8 {
    self.vram[addr as usize]
  }
}


/*
Test this:
8000: FF FF FF FF FF FF FF FF FF FF FF FF FF FF FF FF
FE00: 10 08 00 00
FF48: E4
FF40: 92
*/
