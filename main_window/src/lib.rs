extern crate glfw;

use std::sync::mpsc::Receiver;
use glfw::Context;

pub use glfw::{WindowEvent, Key, Action};
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
}

pub fn init_dependencies() -> WindowDependencies {
  let glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();
  let (mut window, events) = glfw.create_window(300, 300, "Hello, this is window", glfw::WindowMode::Windowed)
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
