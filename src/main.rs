use gui::{separator, vspace};
use state::{GameState, State};

mod game;
mod state;

const CREDITS: &str = "c";
const HELP: &str = "h";
const LOAD_GAME: &str = "l";
const NEW_GAME: &str = "n";
const QUIT: &str = "q";
const SETTINGS: &str = "s";

fn main() {
    let mut app_state = State::default();
    loop {
        gui::loop_separator();
        if app_state.startup {
            gui::main_menu();
            let input = get_usr_input("What would you like to do?");
            match input.as_str() {
                NEW_GAME => {
                    app_state.startup = false;
                    app_state.run_game = true;
                    app_state.new_game = true;
                },
                LOAD_GAME => println!("Not yet implemented"),
                HELP => println!("Not yet implemented"),
                CREDITS => println!("Not yet implemented"),
                SETTINGS => println!("Not yet implemented"),
                QUIT => quit(&app_state.game_state),
                _ => println!("Unknown input: {}", input),
            }
            vspace();
        }
        if app_state.new_game {
            game::new_game(&mut app_state.game_state);
            app_state.new_game = false;
        }
        if app_state.run_game {
            // first display menus and user input
            game::action_loop(&mut app_state.game_state);

            // run game by chosen amount
            separator();
            println!("To advance time type a number - You will then be asked if you want to pass seconds, hours, days or months.");
            println!("Don't worry, we are talking game time here, not real time - that depends on your hardware.");
            let time_in = get_usr_input("How much time do you want to advance?");
            let time = time_in.parse::<u32>();
            if time.is_err() {
                println!("Invalid input: {} - Reason: {:?},\n aborting - no time will pass", time_in, time);
            } else {
                let time = time.unwrap();
                if time == 0 {
                    println!("Invalid input: {}, aborting - no time will pass", time_in);
                } else {
                    separator();
                    println!("Type s to advance {} seconds", time);
                    println!("Type m to advance {} minutes", time);
                    println!("Type h to advance {} hours", time);
                    println!("Type d to advance {} days", time);
                    println!("Type M or w to advance {} months", time);
                    let input = get_usr_input("What do you want to do?");
                    // 0 ticks is used as special case to abort
                    let ticks = {
                        match input.as_str() {
                            "s" => time,
                            "m" => time.saturating_mul(60),
                            "h" => time.saturating_mul(60).saturating_mul(60),
                            "d" => time.saturating_mul(60).saturating_mul(60).saturating_mul(24),
                            "M" | "w" => time.saturating_mul(60).saturating_mul(60).saturating_mul(24).saturating_mul(30),
                            _ => {
                                println!("Unknown input: {}, aborting - no time will pass", input);
                                0
                            },
                        }
                    };
                    vspace();
                    if ticks != 0 {
                        for _ in 0..ticks {
                            game::game_loop(&mut app_state.game_state);
                        }
                    }
                }
            }
        }
    }
}

mod gui {
    pub fn loop_separator() {
        println!("########################################################");
    }
    pub fn separator() {
        println!("--------------------------------------------------------");
    }
    pub fn main_menu() {
        separator();
        println!("Welcome to Space Ship Construction Company!");
        separator();
        println!("Type n to start a new game");
        println!("Type l to load a game");
        println!("Type h for help");
        println!("Type c for credits");
        println!("Type s for settings");
        println!("Type q to quit");
        separator();
    }

    /// 10 empty lines
    pub fn vspace() {
        println!("");
        println!("");
        println!("");
        println!("");
        println!("");
        println!("");
        println!("");
        println!("");
        println!("");
        println!("");
    }

    pub fn custom_vspace(lines: u32) {
        for _ in 0..lines {
            vspace();
        }
    }
}

fn get_usr_input(prompt: &str) -> String {
    println!("{}", prompt);
    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    println!("############################ |RESPONSE| ############################");
    input.trim().to_string()
}

fn quit(app_state: &GameState) {
    // TODO: Save state to disk

    // end process
    std::process::exit(0);
}
