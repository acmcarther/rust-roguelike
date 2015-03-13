#[derive(Copy)]
pub struct LogicState {
  good: bool
}

impl LogicState {
  pub fn new(good: bool) -> LogicState {
    LogicState { good: good }
  }

  pub fn is_good(&self) -> bool {
    self.good
  }

  pub fn as_closed(&self) -> LogicState {
    let mut new_state = *self;
    new_state.good = false;
    new_state
  }
}

pub fn init_state() -> LogicState {
  LogicState::new(true)
}

pub fn state_good(state: LogicState) -> bool {
  state.good
}
