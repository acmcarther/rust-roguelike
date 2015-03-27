extern crate game_logic;

use game_logic::{World, WorldDecision};

pub fn increase_edges_decision() -> WorldDecision {
  |world: &mut World| {
    world.inc_edges();
  }
}

pub fn decrease_edges_decision() -> WorldDecision {
  |world: &mut World| {
    world.dec_edges();
  }
}
