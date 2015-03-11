#![feature(old_io, core)]
extern crate rand;
extern crate core;

use std::old_io::stdin;
use std::old_io::IoResult;
use std::ascii::AsciiExt;
use std::option;
use std::num::SignedInt;
use core::str::FromStr;

//use rand::Rng;
use rand::distributions::{IndependentSample, Range};

use GuessResult::{Continue, Finish};

#[derive(Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Debug, Hash)]
pub enum GuessResult<T> {
  Continue(T),
  Finish(T)
}

fn main() {
  print_title();
  let user_input = stdin().read_line();
  match user_input {
    Ok(line) => start_game(line),
    _ => ()
  }

  println!("Goodbye")
}

fn start_game(input: String) {
  if input.as_slice() == "start\n" {
    launch_game()
  } else {
    let trimmed_input = input.trim();
    println!("You quit with \"{}\".", trimmed_input)
  }
}

fn print_title() {
  println!("-------------------");
  println!("| Welcome to game! |");
  println!("-------------------");
}

fn launch_game() {
  game_manager();
}

fn game_manager() {
  let mut user_wants_to_play = true;
  while user_wants_to_play {
    let quit_result = dice_rolling_game();
    user_wants_to_play = !quit_result && ask_to_continue();
  }
}

fn dice_rolling_game() -> bool {
  let range = Range::new(0, 20);
  let mut rng = rand::thread_rng();

  let number = range.ind_sample(&mut rng);

  println!("Guess the number between 1 and 20");

  dice_rolling_stage(number, None, 5);

  false
}

fn dice_rolling_stage(number: i32, last_guess: Option<i32>, tries_left: u32) {
  let guess = stdin().read_line();
  let guess_value = i32::from_str(guess.unwrap().as_slice().trim()).unwrap();
  let guess_result = get_guess_result(number, guess_value, last_guess, tries_left);
  match guess_result {
    Continue(continue_str) => {
      println!("{}", continue_str);
      dice_rolling_stage(number, Some(guess_value), tries_left - 1)
    },
    Finish(end_str) => {
      println!("{}", end_str);
      println!("it was {}", number)
    }
  }
}


fn get_guess_result(number: i32,  result: i32, last_guess: Option<i32>, tries_left: u32) -> GuessResult<String> {
  if result == number {
    Finish(String::from_str("You did it!"))
  } else if tries_left == 0 {
    Finish(String::from_str("Out of tries, you suck."))
  } else {
    match last_guess {
      Some(guess) => {
        let old_guess_dist = (guess - number).abs();
        let new_guess_dist = (result - number).abs();

        if old_guess_dist > new_guess_dist {
          Continue(String::from_str("You're getting warmer."))
        } else if old_guess_dist < new_guess_dist {
          Continue(String::from_str("You're getting colder."))
        } else {
          Continue(String::from_str("You are equal distance."))
        }
      },
      None => Continue(String::from_str("Try again!"))
    }
  }
}


fn ask_to_continue() -> bool {
  println!("Do you want to keep playing?");
  let user_input = stdin().read_line();
  match user_input {
    Ok(line) => is_yes(line),
    _ => false
  }
}

fn is_yes(input: String) -> bool {
  let posib = [ "y", "yes", "yeah", "mmhm", "yap", "ya"];
  let formatted_string = input.trim().as_slice().to_ascii_lowercase();
  posib.contains(&formatted_string.as_slice())
}
