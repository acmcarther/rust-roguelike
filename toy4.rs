pub trait Decision {
  pub fn concat(self, &decision: Self) -> Self;
  pub fn apply(self, world: World) -> World;
}

pub struct ForceDecision {
  forces: HashMap<usize, Force>;
}

impl Decision for ForceDecision {
  pub fn concat(self, &decision: ForceDecision) -> ForceDecision {
    
  }

  pub fn apply(self, world: mut World, dt: usize) -> World {
    forces.iter().fold(world, |accWorld, forceSet| {
      let (ent_id, force) = forceSet;
      let current_force =
      accWorld.setForce 
    })
  }
}
