extern crate game_gfx;
extern crate game_logic;
extern crate game_input;
extern crate game_window;
extern crate task_parallelism;

use task_parallelism::{PoolToken, Task};
use game_logic::World;
use threadpool::ScopedPool;
use std::sync::mpsc::{Sender, channel};
use std::sync::Arc;

use game_window::WindowDependencies;
use game_gfx::GfxDependencies;
use game_logic::{ World, WorldDecision, WorldTransformer };

pub struct GameDependencies {
  window_deps: WindowDependencies,
  gfx_deps: GfxDependencies,
  state: World
}

impl GameDependencies {
  pub fn new(window_deps: WindowDependencies,
             gfx_deps: GfxDependencies,
             state: World)
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

fn init() -> GameDependencies {
  let mut window_deps = game_window::init_dependencies();
  let gfx_deps = game_gfx::init_dependencies(&mut window_deps);
  let state = World::new();
  GameDependencies::new(window_deps, gfx_deps, state)
}

fn run(game_deps: &mut GameDependencies) {
  let mut pool_token = PoolToken::new(4);

  while game_deps.should_stay_open() {
    let tasks = get_tasks(&mut gfx_deps, &mut window_deps);
    let decisions = pool.in_parallel(&game_deps.state, tasks);
    let consensus = decisions.fold(WorldDecision::null_decision(), WorldDecision::arbitrate);
    world.apply(consensus);
  }
}

fn get_tasks(gfx_deps: GfxDependencies, window_deps: WindowDependencies) -> Vec<WorldTransformer>{
  let events = game_window::get_events(&mut window_deps);

  vec![
    game_input::input_task(events),
    game_physics::physics_task(),
    game_gfx::render_task(gfx_deps, window_deps)
  ]

  // TODO Move this to render task
  //game_gfx::render(&game_deps.state, &mut game_deps.gfx_deps, &mut game_deps.window_deps);
}

