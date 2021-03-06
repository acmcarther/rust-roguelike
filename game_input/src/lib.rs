extern crate game_window;
extern crate game_logic;

use std::iter::IteratorExt;
use game_logic::LogicState;
use game_window::{Action, Key, WindowEvent};

pub type InputHandler = fn(LogicState, WindowEvent) -> LogicState;

pub fn handle_events(logic_state: LogicState, events: Vec<WindowEvent>) -> LogicState {
  events.iter().fold(logic_state, check_keys)
}

fn check_keys(logic_state: LogicState, event: &WindowEvent) -> LogicState {
  match *event {
    WindowEvent::Key(Key::Escape, _, Action::Press, _) => logic_state.as_closed(),
    WindowEvent::Key(Key::A, _, Action::Press, _) => logic_state.inc_edges(),
    WindowEvent::Key(Key::D, _, Action::Press, _) => logic_state.dec_edges(),
    _ => logic_state
  }
}

