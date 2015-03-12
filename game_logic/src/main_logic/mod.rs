use std::old_io::{stdin, IoResult};
use std::sync::mpsc::channel;
use std::thread;
use std::num::ToPrimitive;
use time;

use menus::pause_screen;
use shared_types::MenuResult;

#[derive(PartialEq, Eq)]
enum GameState {
  RecvInput,
  Waiting,
  Finished
}

pub fn start_game() {
  let mut state = GameState::RecvInput;
  while state == GameState::RecvInput {
    let (tx, rx) = channel();
    thread::spawn(move|| {
      tx.send(stdin().read_char()).unwrap();
    });

    let mut last_len = 0usize;
    let mut last_time = time::now();
    state = GameState::Waiting;

    while state == GameState::Waiting {
      let this_time = time::now();
      last_len = print_stuff(last_len, last_time, this_time);
      last_time = this_time;
      state = rx.try_recv().map(check_pausing).unwrap_or_else(|_| GameState::Waiting);
    }
  }

}

fn check_pausing(user_input: IoResult<char>) -> GameState {
  user_input.map(check_pause_game_state)
            .unwrap_or_else(|_| GameState::RecvInput)
}

fn check_pause_game_state(input: char) -> GameState {
  if input == 'q' {
    pausing()
  } else {
    GameState::RecvInput
  }
}


fn print_stuff(last_len: usize, last_time: time::Tm, this_time: time::Tm) -> usize {
  let dt = this_time.tm_nsec - last_time.tm_nsec;
  let freq = 1.0 / dt.to_f32().unwrap();
  for _ in 0..last_len {
    print!("\x08");
  }
  let res_str = format!("{} ghz", freq);
  print!("{}", res_str);
  res_str.len()
}

fn pausing() -> GameState {
  match pause_screen::launch_screen() {
    MenuResult::Options => {
      println!("No options");
      pausing()
    },
    MenuResult::Play => GameState::RecvInput,
    MenuResult::Exit => GameState::Finished
  }
}
