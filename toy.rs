
enum DecisionSpace {
  // All possible decision dimensions,
  // generated dynamically
  PhysicsDimension,
  //AiDimension,
  //CharacterDimension,
  //SoundDimension
}

mod PhysicsDimension {
  type Impulse = (f32, f32);
  type Pos = (f32, f32);
  type Entity = (i32, Pos);

  struct PhysicsDecision {
    entities: Map<i32, Impulse>
  }

  struct PhysicsWorld {
    entities: Map<i32, Pos>
  }

  impl AbstractWorld for PhysicsWorld {
    fn apply(self, decision: PhysicsDecision) -> {
      PhysicsWorld { entities:
        self.entities.iter().map(|entity| {
          let (x, y) = entity.second();
          Entity { entity.first, (x+5, y-1) }
        })
      }
    }
  }

  impl AbstractDecision for PhysicsDecision {
    fn null_decision() -> PhysicsDecision {
      PhysicsDecision{ impulses: [].iter().collect() }
    }

    fn add(mut self, other: PhysicsDecision) -> PhysicsDecision {
      other.iter().fold((), |_, pair| {
        let (k, v) = pair;
        let (x, y) = v;
        let past_value = self.remove(k).unwrap_pr((0.0, 0.0));
        let (old_x, old_y) = past_value;
        self.insert(k, (x + old_x, y + old_y));
      });
      self
    }
  }
}

mod AbstractDimension {
  trait AbstractWorld {
    fn apply(self, decision: AbstractDecision) -> AbstractWorld;
  }
  trait AbstractDecision {
    fn null_decision() -> AbstractDecision;
    fn add(self, other: AbstractDecision) -> AbstractDecision;
  }
}

trait DecisionDimension<T> where T: DecisionWorld {
  fn null_decision() -> Decision;
  fn add(this: Decision, other: Decision) -> Decision;
  fn apply(this: Decision, world: T) -> DecisionWorld;
}
type Consensus = Vec<Decision<T>>

let deciders = [];
let delta = inputs.aggregate;
let consensus = deciders.iter()
                        .map(|decider| decider.decide(delta, world))
                        .fold(NULL_CONSENSUS, build_consensus);

let new_world = world.reconcile(consensus);  // Consuming function (self)

renderer.render(world)

fn build_consensus(acc_decisions: Consensus, new_decision: Decision) {
  // if decisions in same dimension, do work lossy
  // if decisions in different dimensions, combine losslessly
}

