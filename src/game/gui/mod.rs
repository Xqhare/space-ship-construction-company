use crate::{gui::separator, state::GameState};

pub fn main_menu() {
    separator();

}

pub fn show_ressources(game_state: &GameState) {
    separator();
    println!("--------------- {} ---------------", game_state.company_name);
    println!("Money: {}", game_state.money);
    println!("Reputation: {}", game_state.reputation);
}

pub fn show_actions(game_state: &GameState) {
    separator();
    println!("You can do the following actions:");
    println!("Type m for factory management");
    println!("Type r for research");
    println!("Type d for development and design");
    println!("Type p for production");
    println!("Type b for business");
    println!("Type a to advance time");
    println!("Type q to quit and save");
}

pub mod new_game {
    use crate::{get_usr_input, gui::{separator, vspace}, state::GameState};

    pub fn setup(game_state: &mut GameState) {
        separator();
        println!("First we need to set up the company!");
        separator();
        setup_company(game_state);
        separator();
        println!("Now let's set up the games difficulty!");
        setup_difficulty(game_state);
        separator();
        println!("Now we are ready to start the game!");
        vspace();
    }

    fn setup_company(game_state: &mut GameState) {
        let input = get_usr_input("What is the name of the company?");
        game_state.company_name = input;
    }

    fn setup_difficulty(game_state: &mut GameState) {
        println!("Type e for easy");
        println!("Type m for medium");
        println!("Type h for hard");
        let input = get_usr_input("What difficulty do you choose?");
        match input.as_str() {
            "e" => {
                println!("You have chosen easy");
                game_state.money = 1_000_000;
                game_state.reputation = 1_000;
            },
            "m" => {
                println!("You have chosen medium");
                game_state.money = 500_000;
                game_state.reputation = 500;
            },
            "h" => {
                println!("You have chosen hard");
                game_state.money = 100_000;
                game_state.reputation = 1;
            },
            _ => println!("Invalid input"),
        }
    }
}
