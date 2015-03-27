use std::iter::IteratorExt;
use std::collections::HashMap;
use PhysicsDimension::{Impulse, PhysicsDecision, PhysicsWorld, Pos};

// Describing full workflow
// 1) Subscribe with task delivery method (method taking a state and returning a decision) and an
//    interval
// 2) Thread pool (run by main) will queue up tasks
// 3) Decisions will be aggregated by DecisionDimension and then compiled into a consensus
// 4) World will apply the consensus


// We know that entites will be ids associated with any number of components
// State will live in component registries
// Component registries live inside the World

/*
struct StaticWarudo {
  logicalState: i32,
  environment: i32,
}
*/
struct PositionComponent {
  x: f32,
  y: f32,
  z: f32,
}

struct VelocityComponent {
  x_v: f32,
  y_v: f32,
  z_v: f32,
}

struct MassComponent {
  x_v: f32,
}

struct DynamicWarudo {
  pub entities:         Vec<u32>,
  pub positionRegistry: HashMap<u32, &PositionComponent>,
  pub velocityRegistry: HashMap<u32, &VelocityComponent>,
  pub massRegistry:     HashMap<u32, &MassComponent>,
  //gfx3dRegistry:    HashMap<u32, &Gfx3dComponent>,
  //inputRegistry:    HashMap<u32, &InputComponent>,
  //aiRegistry:       HashMap<u32, &AiComponent>,
  //auraRegistry:     HashMap<u32, &AuraComponent>,
  //healthRegistry:   HashMap<u32, &HealthComponent>
}

/*
struct ZaWarudo {
  pub statics:  StaticWarudo,
  pub dynamics: DynamicWarudo,
}
*/

type VelocityDecision = HashMap<u32, Vec3<f32>>

impl VelocityDecision {
  pub fn consuming_plus(self, other: &VelocityDecision) {
    for (k, v) in other.iter() {
      
    }
  }

  pub fn plus(&self, other: &VelocityDecision) {
    
  }
}

fn gravityDecision(dynamicWorld: &DynamicWarudo) -> PhysicsDecision {
  let velocitied: BitSet = dynamicWorld.velocityRegistry.keys.iter().collect()
  let massed:     BitSet = dynamicWorld.massRegistry.keys.iter().collect()
  let afflicted          = velocitied.intersection(massed)
  VelocityDecision::new(afflicted.iter()
                                 .map(|id| DeltaV::new(id, Vec3::(0.0, 0.0, -9.81))
                                 .collect())
}

mod AbstractDimension {
  pub trait AbstractWorld<T> where T: AbstractDecision {
    fn apply(self, decision: &T) -> Self;
  }
  pub trait AbstractDecision {
    fn null_decision() -> Self;
    fn add(self, other: &Self) -> Self;
  }
}

mod PhysicsDimension {
  use AbstractDimension::{AbstractWorld, AbstractDecision};
  use std::collections::HashMap;

  pub type Impulse = (f32, f32);
  pub type Pos = (f32, f32);
  pub type Vel = (f32, f32);
  pub type EntityFields = (Pos, Vel);

  #[derive(Debug)]
  pub struct PhysicsDecision {
    impulses: HashMap<i32, Impulse>
  }

  #[derive(Debug)]
  pub struct PhysicsWorld {
    pub entities: HashMap<i32, EntityFields>
  }

  impl PhysicsWorld {
    pub fn new(entities: HashMap<i32, EntityFields>) -> PhysicsWorld {
      PhysicsWorld { entities: entities }
    }
  }

  impl PhysicsDecision {
    pub fn new(impulses: HashMap<i32, EntityFields>) -> PhysicsDecision {
      PhysicsDecision { impulses: impulses }
    }
  }

  impl AbstractWorld<PhysicsDecision> for PhysicsWorld {
    fn apply(self, decision: &PhysicsDecision) -> PhysicsWorld {
      decision.impulses.iter().fold(self, |mut acc, entry| {
        let (id, impulse) = entry;
        let (x_v, y_v) = *impulse;
        let past_value = acc.entities.remove(id);
        match past_value {
          Some(pos) => {
            let (old_x, old_y) = pos;
            acc.entities.insert(*id, (old_x + x_v, old_y + y_v));
            acc
          },
          None => acc
        }
      })
    }
  }

  impl AbstractDecision for PhysicsDecision {
    fn null_decision() -> PhysicsDecision {
      PhysicsDecision{ impulses: [].iter().map(|i| *i).collect() }
    }

    fn add(mut self, other: &PhysicsDecision) -> PhysicsDecision {
      other.impulses.iter().fold((), |_, pair| {
        let (k, v) = pair;
        let (x, y) = *v;
        let past_value = self.impulses.remove(k).unwrap_or((0.0, 0.0));
        let (old_x, old_y) = past_value;
        self.impulses.insert(*k, (x + old_x, y + old_y));
      });
      self
    }
  }
}

fn main() {
  use AbstractDimension::{AbstractWorld, AbstractDecision};
  let mut entities: HashMap<i32, Pos> = HashMap::new();
  entities.insert(5, ((9.0, 8.0),(0.0, 0.0)));
  entities.insert(1, ((3.0, 2.0),(0.0, 0.0)));
  let world = PhysicsWorld::new(entities);

  let decision = gravity_decision(&world);
  //let final_d = decisions.iter().fold(PhysicsDecision::null_decision(), |acc, decision| acc.add(decision));
  let new_world = world.apply(&decision);
  println!("{:?}", new_world);
}

fn gravity_decision(world: &PhysicsWorld) -> PhysicsDecision {
  PhysicsDecision::new(world.entities.iter().map(|pair| {
    let (id, pos) = pair;
    (*id, (0.0, -9.8))
  }).collect())
}
