extern crate glfw;
extern crate cgmath;

use std::sync::mpsc::Receiver;

pub use cgmath::Vector2;
pub use glfw::{Action, Context, Key, WindowEvent};
pub type Events = Receiver<(f64, glfw::WindowEvent)>;

#[allow(dead_code)]
pub struct WindowDependencies {
  glfw: glfw::Glfw,
  window: glfw::Window,
  events: Events
}

impl WindowDependencies {
  pub fn new(glfw: glfw::Glfw, window: glfw::Window, events: Events)
    -> WindowDependencies {
    WindowDependencies { glfw: glfw, window: window, events: events}
  }

  pub fn dimensions(&self) -> Vector2<u16> {
    let (w, h) = self.window.get_framebuffer_size();
    Vector2::new(w as u16, h as u16)
  }

  pub fn swap_buffers(&mut self) {
    self.window.swap_buffers()
  }

  pub fn window(&mut self) -> &mut glfw::Window {
    &mut self.window
  }
}

pub fn init_dependencies() -> WindowDependencies {
  let glfw = glfw::init(glfw::FAIL_ON_ERRORS)
                  .ok().expect("Failed to init GLFW.");
  let (mut window, events) = glfw.create_window(300, 300, "Hello, this is window", glfw::WindowMode::Windowed)
                                 .expect("Failed to create GLFW window.");
  window.set_key_polling(true);
  window.make_current();

  WindowDependencies::new(glfw, window, events)
}

pub fn get_events(deps: &mut WindowDependencies) -> Vec<glfw::WindowEvent> {
  deps.glfw.poll_events();
  glfw::flush_messages(&deps.events)
       .map(|(_, event)| event).collect()
  // Handle internal window events (ex: resize)
}
