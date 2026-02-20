
use crate::{get_usr_input, gui::loop_separator, quit, state::GameState};

pub mod tech;
mod gui;

/// The main game loop - advances the game state increment_timer by 1
pub fn game_loop(game_state: &mut GameState) {
    game_state.increment_timer += 1;
    println!("Time passed: {}", game_state.increment_timer);
}

pub fn action_loop(game_state: &mut GameState) {
    let mut end_turn = false;
    while !end_turn {
        loop_separator();
        gui::show_ressources(game_state);
        gui::show_actions(game_state);
        let input = get_usr_input("What would you like to do?");
        match input.as_str() {
            "m" => println!("Factory management"),
            "r" => println!("Research"),
            "d" => println!("Development and design"),
            "p" => println!("Production"),
            "b" => println!("Business"),
            "a" => end_turn = true,
            "q" => quit(&game_state),
            _ => println!("Unknown input: {}", input),
        }
    }

}

pub fn new_game(game_state: &mut GameState) {
    gui::new_game::setup(game_state);
}
