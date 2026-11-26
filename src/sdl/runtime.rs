use sdl3::{
    event::Event,
    keyboard::Keycode,
    EventPump,
    Sdl,
    video::Window,
};

use crate::{
    graphics::renderer::Renderer,
};

pub struct Runtime {
    _sdl: Sdl,
    window: Window,
    event_pump: EventPump,
    renderer: Renderer,
    running: bool,
    frame: f32,
}

impl Runtime {
    pub fn new(
        sdl: Sdl,
        window: Window,
        renderer: Renderer
    ) -> Self {
        let event_pump = sdl.event_pump().unwrap();

        Self {
            _sdl: sdl,
            window,
            event_pump,
            renderer,
            running: true,
            frame: 0.0,
        }
    }

    pub fn run(&mut self) {
        while self.running {
            self.handle_events();
            self.update();
            self.render();
        }
    }

    fn handle_events(&mut self) {
        for event in self.event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => {
                    self.running = false;
                }

                Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => {
                    self.running = false;
                }

                _ => {}
            }
        }
    }

    fn update(&mut self) {
        // Run emulator
    }

   fn render(&mut self) {
    self.frame += 0.01;

    let red = self.frame.sin() * 0.5 + 0.5;
    let green = (self.frame + 2.0).sin() * 0.5 + 0.5;
    let blue = (self.frame + 4.0).sin() * 0.5 + 0.5;

    self.renderer.render(red, green, blue);

    self.window.gl_swap_window();
   }
} 
