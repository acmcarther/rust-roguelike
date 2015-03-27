extern crate game_window;
extern crate game_logic;

use std::iter::IteratorExt;
use game_window::{Action, Key, WindowEvent};
use game_logic::{World, WorldDecision, WorldTransformer};

enum Events {
  Close,
  IncrEdges,
  DecrEdges
}

pub fn input_task(events: Vec<WindowEvent>) -> WorldTransformer {
  |world: &World| {
    // TODO: Prepare this to change based on state
    let decisions = events.iter().map {
      match *event {
        WindowEvent::Key(Key::Escape, _, Action::Press, _) => game_logic::close_window_decision()
        WindowEvent::Key(Key::A, _, Action::Press, _) => game_geometry::increase_edges_decision(),
        WindowEvent::Key(Key::D, _, Action::Press, _) => game_geometry::decrease_edges_decision(),
        WindowEvent::Key(Key::A, _, Action::Repeat, _) => game_geometry::increase_edges_decision(),
        WindowEvent::Key(Key::D, _, Action::Repeat, _) => game_geometry::decrease_edges_decision(),
        _ => game_logic::null_decision()  // TODO: Optimize this out?
      }
      decisions.fold(WorldDecision::null_decision(), WorldDecision::arbitrate)
    };
  }
}

