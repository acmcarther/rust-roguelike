extern crate game_window;
extern crate game_logic;

use game_logic::LogicState;
use game_window::{Action, Key, WindowEvent};

pub type InputHandler = fn(LogicState, WindowEvent) -> LogicState;

pub fn handle_events(logic_state: &mut LogicState, events: Vec<WindowEvent>) {
  for event in events.iter() {
    match *event {
      WindowEvent::Key(Key::Escape, _, Action::Press, _) => logic_state.closed(),
      WindowEvent::Key(Key::A, _, Action::Press, _) => logic_state.inc_edges(),
      WindowEvent::Key(Key::D, _, Action::Press, _) => logic_state.dec_edges(),
      WindowEvent::Key(Key::A, _, Action::Repeat, _) => logic_state.inc_edges(),
      WindowEvent::Key(Key::D, _, Action::Repeat, _) => logic_state.dec_edges(),
      _ => ()
    }
  }
}

