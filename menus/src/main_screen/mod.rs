use std::collections::HashMap;
use std::old_io::stdin;
use shared_types::MenuResult;

pub fn launch_screen() -> MenuResult {
  println!("launch screen");
  println!("-------------");
  println!("press p to play");
  println!("press o to go to options");
  println!("press q to quit");
  println!("-------------");
  wait_for_selection()
}

pub fn wait_for_selection() -> MenuResult {
  let user_input = stdin().read_line();

  match user_input {
    Ok(input) => {
      let screen_result = menu_opt_from(input);
      match screen_result {
        Some(result) => result,
        None => wait_for_selection()
      }
    },
    _ => MenuResult::Exit
  }
}

fn menu_opt_from(input: String) -> Option<MenuResult> {
  let option_map: HashMap<&str, MenuResult> = vec![
    ("p", MenuResult::Play),
    ("o", MenuResult::Options),
    ("q", MenuResult::Exit),
  ].into_iter().collect();

  let trimmed_input = &*(input.trim());

  option_map.get(trimmed_input).map(|v| *v)
}
