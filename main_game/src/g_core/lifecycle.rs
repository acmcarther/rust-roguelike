use title::main_screen;
use title::main_screen::ScreenResult::{ Play, Options, Exit };

use game_logic::main_logic;

pub fn start() {
  println!("Lifecycle start!");

  load_init()
}

fn load_init() {
  let screen_result = main_screen::launch_screen();

  match screen_result {
    Play => {
      main_logic::start_game();
      load_init()
    },
    Options => {
      println!("No options");
      load_init()
    },
    Exit => println!("You exit"),
  }
}
