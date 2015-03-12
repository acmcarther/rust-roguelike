extern crate game_logic;
extern crate game_input;
extern crate game_window;

pub struct GameDependencies {
  window_deps: game_window::WindowDependencies,
  state: game_logic::LogicState
}

impl GameDependencies {
  pub fn new(window_deps: game_window::WindowDependencies, state: game_logic::LogicState)
    -> GameDependencies {
    GameDependencies {window_deps: window_deps, state: state}
  }

  pub fn should_stay_open(&self) -> bool {
    self.state.is_good()
  }
}

fn main() {
  let mut game_deps = init();
  run(&mut game_deps);
}

fn init() -> GameDependencies {
  let window_deps = game_window::init_dependencies();
  let state = game_logic::init_state();
  GameDependencies::new(window_deps, state)
}

fn run(game_deps: &mut GameDependencies) {
  while game_deps.should_stay_open() {
    game_deps.state = game_input::handle_events(
      game_deps.state,
      game_window::get_events(&mut game_deps.window_deps)
    );
  }
}
