
use sdl3::video::{GLProfile, SwapInterval, Window};

pub struct GlContext {
    pub gl: glow::Context,
    _context: sdl3::video::GLContext,
}

impl GlContext {
    pub fn new(
        window: &Window,
        video: &sdl3::VideoSubsystem,
    ) -> Result<Self, Box<dyn std::error::Error>> {

        // Configure OpenGL
        {
            let gl_attr = video.gl_attr();

            gl_attr.set_context_profile(GLProfile::Core);
            gl_attr.set_context_version(3, 3);
            gl_attr.set_double_buffer(true);
        }

        // Create SDL OpenGL context
        let context = window.gl_create_context()?;

        // Load glow
        let gl = unsafe {
            glow::Context::from_loader_function(|name| {
                video.gl_get_proc_address(name)
                    .map_or(std::ptr::null(), |f| {
                        f as *const () as *const _
                    })
            })
        };

        video.gl_set_swap_interval(SwapInterval::VSync)?;

        Ok(Self {
            gl,
            _context: context,
        })
    }
}

