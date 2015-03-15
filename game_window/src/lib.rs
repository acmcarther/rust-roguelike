extern crate glfw;

use std::sync::mpsc::Receiver;

pub use glfw::{Action, Context, Key, WindowEvent};
pub type Events = Receiver<(f64, glfw::WindowEvent)>;

#[allow(dead_code)]
pub struct DimensionsXY {
  x: i32,
  y: i32
}

impl DimensionsXY {
  pub fn new(x: i32, y:i32) -> DimensionsXY {
    DimensionsXY { x: x, y: y }
  }

  pub fn x(&self) -> i32 { self.x }
  pub fn y(&self) -> i32 { self.y }
}

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

  pub fn dimensions(&self) -> DimensionsXY {
    let (w, h) = self.window.get_framebuffer_size();
    DimensionsXY::new(w, h)
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
  let (mut window, events) = glfw.create_window(960, 1080, "Hello, this is window", glfw::WindowMode::Windowed)
                                 .expect("Failed to create GLFW window.");
  window.set_key_polling(true);
  window.make_current();

  WindowDependencies::new(glfw, window, events)
}

pub fn get_events(deps: &mut WindowDependencies) -> Vec<glfw::WindowEvent> {
  deps.glfw.poll_events();
  glfw::flush_messages(&deps.events)
       .map(|(_, event)| event ).collect()
}
