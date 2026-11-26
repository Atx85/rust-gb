use sdl3::video::{Window};
use sdl3::VideoSubsystem;

pub fn create(
  video: &VideoSubsystem) -> Result<Window, Box<dyn std::error::Error>> {
  let window = video
      .window("GB Emulator", 960, 864)
      .position_centered()
      .resizable()
      .opengl()
      .build()?;

      Ok(window)
}
