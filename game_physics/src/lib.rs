extern crate game_logic;

use game_logic::{World, WorldDecision, WorldTransformer};

pub fn physics_task() -> WorldTransformer {
  |world: &World| {
    WorldDecision::null_decision()
  }
}

