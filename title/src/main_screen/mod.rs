use std::collections::HashMap;
use std::old_io::stdin;

#[derive(Copy)]
pub enum ScreenResult {
  Play,
  Options,
  Exit
}

pub fn launch_screen() -> ScreenResult {
  println!("launch screen");
  println!("-------------");
  println!("press p to play");
  println!("press o to go to options");
  println!("press q to quit");
  println!("-------------");
  wait_for_selection()
}

pub fn wait_for_selection() -> ScreenResult {
  let user_input = stdin().read_line();

  match user_input {
    Ok(input) => {
      let screen_result = menu_opt_from(input);
      match screen_result {
        Some(result) => result,
        None => wait_for_selection()
      }
    },
    _ => ScreenResult::Exit
  }
}

fn menu_opt_from(input: String) -> Option<ScreenResult> {
  let option_map: HashMap<&str, ScreenResult> = vec![
    ("p", ScreenResult::Play),
    ("o", ScreenResult::Options),
    ("q", ScreenResult::Exit),
  ].into_iter().collect();

  let trimmed_input = &*(input.trim());

  option_map.get(trimmed_input).map(|v| *v)
}
