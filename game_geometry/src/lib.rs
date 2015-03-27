extern crate game_logic;

use game_logic::{World, WorldDecision};
use game_registries::geometry_interface;

pub fn increase_edges_decision(entity_id: usize) -> WorldDecision {
  |world: &mut World| {
    let interface = world.geometry_interface()
    let component = interface.find_edge_component_for(entity_id);
    match entity {
      Some(component) => {
        component.vertex_count = component.vertex_count + 1usize;
      },
      None
    }
  }
}

pub fn decrease_edges_decision(entity_id: usize) -> WorldDecision {
  |world: &mut World| {
    let mut interface = world.geometry_interface()
    let mut component = interface.find_edge_component_for(entity_id);
    match entity {
      Some(component) => {
        if component.vertex_count != 0 {
          component.vertex_count = component.vertex_count - 1usize;
        }
      },
      None
    }
  }
}

pub fn set_as_ngon(entity_id: usize) -> WorldDecision {
  |world: &mut World| {
    let mut interface = world.geometry_interface()
    let mut component = interface.find_shape_component_for(entity_id);
    component.shape = ShapeType::NGon2D;
  }
}

