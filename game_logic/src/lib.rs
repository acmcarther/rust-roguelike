pub trait GameEvent {
  fn works(&self) -> bool;
}

pub struct LogicState<'a> {
  good: bool,
  edge_count: usize,
  events: Vec<&'a GameEvent>
}

impl <'a>LogicState <'a> {
  pub fn new(good: bool) -> LogicState<'a> {
    LogicState {
      good: good,
      edge_count: 3usize,
      events: vec![]
    }
  }

  pub fn is_good(&self) -> bool {
    self.good
  }

  pub fn closed(&mut self) {
    self.good = false;
  }

  pub fn inc_edges(&mut self) {
    self.edge_count += 1;
  }

  pub fn dec_edges(&mut self) {
    if self.edge_count != 0usize {
      self.edge_count -= 1;
    }
  }

  pub fn edge_count(&self) -> usize {
    self.edge_count
  }

  pub fn clear_events(&mut self) {
    self.events = vec![];
  }
}

pub fn init_state<'a>() -> LogicState<'a> {
  LogicState::new(true)
}

pub fn state_good(state: LogicState) -> bool {
  state.good
}
