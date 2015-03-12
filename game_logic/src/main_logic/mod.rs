use std::old_io::stdin;
use std::sync::mpsc::channel;
use std::thread;
use std::num::ToPrimitive;
use time;

use menus::pause_screen;
use shared_types::MenuResult;

pub fn start_game() {
  let mut continuing = true;
  while continuing {
    let (tx, rx) = channel();
    thread::spawn(move|| {
      tx.send(stdin().read_char()).unwrap();
    });

    let mut waiting = true;
    let mut last_len = 0usize;
    let mut last_time = time::now();

    while waiting {
      let this_time = time::now();
      last_len = print_stuff(last_len, last_time, this_time);
      last_time = this_time;
      match rx.try_recv() {
        Ok(user_input) =>  {
          print!("\n");
          waiting = false;
          match user_input {
            Ok(input) => {
              if input == 'q' {
                continuing = pausing()
              }
            },
            _ => ()
          }

        },
        _ => ()
      }
    }
  }
}

pub fn print_stuff(last_len: usize, last_time: time::Tm, this_time: time::Tm) -> usize {
  let dt = this_time.tm_nsec - last_time.tm_nsec;
  let freq = 1.0 / dt.to_f32().unwrap();
  for _ in 0..last_len {
    print!("\x08");
  }
  let res_str = format!("{} ghz", freq);
  print!("{}", res_str);
  res_str.len()
}

pub fn pausing() -> bool {
  match pause_screen::launch_screen() {
    MenuResult::Options => {
      println!("No options");
      pausing()
    },
    MenuResult::Play => true,
    MenuResult::Exit => false
  }
}
