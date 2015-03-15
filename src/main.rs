extern crate game_gfx;
extern crate game_logic;
extern crate game_input;
extern crate game_window;

pub struct GameDependencies<'a> {
  window_deps: game_window::WindowDependencies,
  gfx_deps: game_gfx::GfxDependencies,
  state: game_logic::LogicState<'a>
}

impl <'a> GameDependencies<'a> {
  pub fn new(window_deps: game_window::WindowDependencies,
             gfx_deps: game_gfx::GfxDependencies,
             state: game_logic::LogicState)
    -> GameDependencies {
    GameDependencies {window_deps: window_deps, gfx_deps: gfx_deps, state: state}
  }

  pub fn should_stay_open(&self) -> bool {
    self.state.is_good()
  }
}

fn main() {
  let mut game_deps = init();
  run(&mut game_deps);
}

fn init<'a>() -> GameDependencies<'a> {
  let mut window_deps = game_window::init_dependencies();
  let gfx_deps = game_gfx::init_dependencies(&mut window_deps);
  let state = game_logic::init_state();
  GameDependencies::new(window_deps, gfx_deps, state)
}

fn run(game_deps: &mut GameDependencies) {
  while game_deps.should_stay_open() {
    game_input::handle_events(
      &mut game_deps.state,
      game_window::get_events(&mut game_deps.window_deps)
    );

    game_gfx::render(&game_deps.state, &mut game_deps.gfx_deps, &mut game_deps.window_deps)
  }
}
