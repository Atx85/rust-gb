
use glow::HasContext;

use super::gl_context::GlContext;

pub struct Renderer {
    gl: GlContext,
}

impl Renderer {
    pub fn new(
        gl: GlContext,
    ) -> Self {
        Self { gl }
    }

    pub fn print_info(&self) {
        unsafe {
            println!(
                "Vendor: {}",
                self.gl.gl.get_parameter_string(glow::VENDOR)
            );

            println!(
                "Renderer: {}",
                self.gl.gl.get_parameter_string(glow::RENDERER)
            );

            println!(
                "Version: {}",
                self.gl.gl.get_parameter_string(glow::VERSION)
            );
        }
    }

    pub fn render(&self, red: f32, green: f32, blue: f32) {
        unsafe {
            self.gl.gl.clear_color(red, green, blue, 1.0);
            self.gl.gl.clear(glow::COLOR_BUFFER_BIT);
        }
    }
}

