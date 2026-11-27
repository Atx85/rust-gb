#![allow(unused_variables)]
#![allow(dead_code)]

mod cpu;
mod ppu;
mod instruction_set; 


struct Machine {
  cpu :cpu::Cpu,
  bus: Bus
}
pub struct Bus {
  memory: [u8; 65536],
  ppu: Option<ppu::Ppu>,
}
impl Bus {
  pub fn new() -> Self {
    let memory = [0 as u8; 65536];
    Self {memory, ppu: None}
  }
  fn ppu(&mut self) -> &mut ppu::Ppu {
    match self.ppu.as_mut() {
      Some(ppu) => {
        return ppu;
      },
      _ => panic!("Ppu not attached to Bus!")
    }
  }
  pub fn attach_ppu(&mut self, ppu: ppu::Ppu) {
    self.ppu = Some(ppu); 
  }
  pub fn load_program(&mut self, program: &[u8], start: usize) {
    self.memory[start..start + program.len()]
      .copy_from_slice(program);
  }
  pub fn write_byte(&mut self, addr: u16, val: u8) {
    if (0xfe00..=0xfe9f).contains(&addr) {
      self.ppu().write_oam_byte(addr, val);
    } else if (0x8000..=0x9fff).contains(&addr) {
      self.ppu().write_vram_byte(addr, val);
    } else {
      self.memory[addr as usize] = val;
    }
  }
  pub fn read_byte(&mut self, addr: u16) -> u8 {
    if (0xfe00..=0xfe9f).contains(&addr) {
      return self.ppu().read_oam_byte(addr);
    } else if (0x8000..=0x9fff).contains(&addr) {
      return self.ppu().read_vram_byte(addr);
    } else {
      return self.memory[addr as usize]
    }
  }
}


mod sdl;
mod graphics;

use sdl::runtime::Runtime;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let sdl = sdl3::init()?;
    let video = sdl.video()?;

    let window = sdl::window::create(&video)?;

    let gl_context =
        graphics::gl_context::GlContext::new(&window, &video)?;

    let renderer = graphics::renderer::Renderer::new(gl_context);
    let mut runtime = Runtime::new(
        sdl,
        window,
        renderer
    );

    // runtime.run();
    let mut  cpu = cpu::Cpu::init();
    let mut bus = crate::Bus::new();
    cpu.attach_bus(bus);
    // let machine = Machine { cpu: cpu, bus: Bus {} };
    cpu.step();
    println!("Hello, world!");


    Ok(())
}
