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


use glow::HasContext;
use sdl3::{
  event::Event,
  keyboard::Keycode,
  video::GLProfile,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
  // Initialize SDL3
  let sdl = sdl3::init()?;
  let video = sdl.video()?;

  // Configure OpenGL
  {
    let gl_attr = video.gl_attr();

    gl_attr.set_context_profile(GLProfile::Core);
    gl_attr.set_context_version(3, 3);
    gl_attr.set_double_buffer(true);
  }

  // Create window
  let window = video
    .window("SDL3 + OpenGL + glow test", 800, 600)
    .position_centered()
    .resizable()
    .opengl()
    .build()?;

  // Create OpenGL context
  let _gl_context = window.gl_create_context()?;

  // Load OpenGL functions
  let gl = unsafe {
    glow::Context::from_loader_function(|name| {
      video.gl_get_proc_address(name).map_or(std::ptr::null(), |f| f as *const () as *const _)
    })
  };

  // Create OpenGL context
  let _gl_context = window.gl_create_context()?;

  // Load OpenGL functions
  let gl = unsafe {
    glow::Context::from_loader_function(|name| {
      video.gl_get_proc_address(name)
        .map_or(std::ptr::null(), |f| {
          f as *const () as *const _
        })
    })
  };

  // Enable VSync
  video.gl_set_swap_interval(
    sdl3::video::SwapInterval::VSync
  )?;    println!("OpenGL initialized!");
  println!(
    "Vendor: {}",
    unsafe { gl.get_parameter_string(glow::VENDOR) }
  );
  println!(
    "Renderer: {}",
    unsafe { gl.get_parameter_string(glow::RENDERER) }
  );
  println!(
    "Version: {}",
    unsafe { gl.get_parameter_string(glow::VERSION) }
  );

  let mut event_pump = sdl.event_pump()?;

  let mut frame: f32 = 0.0;

  'running: loop {
    // Handle events
    for event in event_pump.poll_iter() {
      match event {
        Event::Quit { .. } => break 'running,

        Event::KeyDown {
          keycode: Some(Keycode::Escape),
          ..
        } => break 'running,

        _ => {}
      }
    }

    // Animate background colour
    frame += 0.01;

    let red = frame.sin() * 0.5 + 0.5;
    let green = (frame + 2.0).sin() * 0.5 + 0.5;
    let blue = (frame + 4.0).sin() * 0.5 + 0.5;

    // Render
    unsafe {
      gl.clear_color(red, green, blue, 1.0);
      gl.clear(glow::COLOR_BUFFER_BIT);
    }

    // Present frame
    window.gl_swap_window();
  }

  println!("Application closed.");

  Ok(())
}




