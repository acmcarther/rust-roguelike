extern crate game_input;
extern crate game_physics;
extern crate core;

use game_input::ControlRegistry;
use game_physics::*;
use std::collections::BitSet;
use std::collections::HashMap;
use std::iter::FromIterator;
use core::iter::IntoIterator;

// TODO: Make this more complicated
pub type WorldTransformer = Task<&World, WorldDecision>;
pub type WorldDecision = fn(&mut world) -> ();

// TODO: Generate these structures with macros
pub struct StaticWorld {
  // TODO: More Interesting logical state
  logical_state: bool,
}

pub struct DynamicWorld {
  entities: BitSet,

  /* Physics Components */
  position_registry: PositionRegistry,
  velocity_registry: VelocityRegistry,
  mass_registry:     MassRegistry,
  edge_registry:     EdgeRegistry,
  control_registry:  ControlRegistry,

  /* Graphics Components */
  //renderable_registry: HashMap<u32, &renderableComponent>
}

pub struct World {
  statics: StaticWorld,
  dynamics: DynamicWorld
}

type Registry<T> = HashMap<usize, T>;

impl World {
  pub fn new() -> World {
    World {
      statics: StaticWorld::new(true),
      dynamics: DynamicWorld::new(
        vec![1usize].into_iter().collect(),
        vec![(1usize, PositionComponent::new(0.0,0.0,0.0))].into_iter().collect(),
        vec![(1usize, VelocityComponent::new(0.0,0.0,0.0))].into_iter().collect(),
        vec![].into_iter().collect(),
        vec![(1usize, EdgeComponent::new(6usize))].into_iter().collect(),
        vec![(1usize, ControlComponent::new())].into_iter().collect()
      )
    }
  }

  pub fn is_good(&self) -> bool {
    self.statics.is_good()
  }

  pub fn closed(&mut self) {
    self.statics.closed()
  }

  pub fn inc_edges(&mut self) {
    // Update default entity
    self.dynamics.inc_edges(1usize);
  }

  pub fn dec_edges(&mut self) {
    // Update default entity
    self.dynamics.dec_edges(1usize);
  }

  pub fn edge_count(&self) -> usize {
    self.dynamics.edge_count(1usize)
  }
}

pub fn init_state() -> LogicState {
  LogicState::new(true)

impl StaticWorld {
  pub fn new(good: bool) -> StaticWorld {
    StaticWorld {
      logical_state: good,
    }
  }

  pub fn is_good(&self) -> bool {
    self.logical_state
  }

  pub fn closed(&mut self) {
    self.logical_state = false;
  }
}

// TODO: This needs to be generated with a macro
impl DynamicWorld {
  pub fn new(entities: BitSet, position_registry: PositionRegistry, velocity_registry: VelocityRegistry, mass_registry: MassRegistry, edge_registry: EdgeRegistry, control_registry: ControlRegistry) -> DynamicWorld {
    DynamicWorld {
      entities: entities,
      position_registry: position_registry,
      velocity_registry: velocity_registry,
      mass_registry: mass_registry,
      edge_registry: edge_registry,
      control_registry: control_registry,
    }
  }

  pub fn inc_edges(&mut self, ent_id: usize) {
    match self.edge_registry.get_mut(&ent_id) {
      Some(x) => *x = EdgeComponent::new(x.edge_count + 1),
      None => (),
    }
  }

  pub fn dec_edges(&mut self, ent_id: usize) {
    match self.edge_registry.get_mut(&ent_id) {
      Some(x) => {
        if x.edge_count > 6usize {
          *x = EdgeComponent::new(x.edge_count - 1)
        }
      },
      None => (),
    }
  }

  pub fn edge_count(&self, ent_id: usize) -> usize {
    self.edge_registry.get(&ent_id).unwrap().edge_count
  }
}
