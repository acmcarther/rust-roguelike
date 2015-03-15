#[derive(Copy)]
pub struct LogicState {
  good: bool,
  edge_count: usize,
  x: f32,
  y: f32
}

impl LogicState {
  pub fn new(good: bool) -> LogicState {
    LogicState {
      good: good,
      edge_count: 3usize,
      x: 0.0,
      y: 0.0
    }
  }

  pub fn is_good(&self) -> bool {
    self.good
  }

  pub fn as_closed(&self) -> LogicState {
    let mut new_state = *self;
    new_state.good = false;
    new_state
  }

  pub fn inc_edges(&self) -> LogicState {
    let mut new_state = *self;
    new_state.edge_count += 1;
    new_state
  }

  pub fn dec_edges(&self) -> LogicState {
    let mut new_state = *self;
    if self.edge_count == 0usize {
      new_state
    } else {
      new_state.edge_count -= 1;
      new_state
    }
  }

  pub fn edge_count(&self) -> usize {
    self.edge_count
  }
}

pub fn init_state() -> LogicState {
  LogicState::new(true)
}

pub fn state_good(state: LogicState) -> bool {
  state.good
}
