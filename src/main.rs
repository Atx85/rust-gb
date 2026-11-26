#![allow(unused_variables)]
#![allow(dead_code)]

mod cpu;
mod instruction_set; 


struct Machine {
  cpu :cpu::Cpu,
  bus: Bus
}
pub struct Bus {
  memory: [u8; 65536],
}
impl Bus {
  pub fn new() -> Self {
    let memory = [0 as u8; 65536];
    Self {memory}
  }
  pub fn load_program(&mut self, program: &[u8], start: usize) {
    self.memory[start..start + program.len()]
      .copy_from_slice(program);
  }
  pub fn write_byte(&mut self, addr: u16, val: u8) {
    println!("Bus: write_byte {:X} {:X}", addr, val);
    self.memory[addr as usize] = val;
  }
  pub fn read_byte(&mut self, addr: u16) -> u8 {
    println!("Bus: read {:X}", addr); 
    self.memory[addr as usize]
  }
}


/*
   fn main() {
   let mut  cpu = cpu::Cpu::init();
// let machine = Machine { cpu: cpu, bus: Bus {} };
cpu.step();
println!("Hello, world!");


}
*/


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

    runtime.run();

    Ok(())
}
