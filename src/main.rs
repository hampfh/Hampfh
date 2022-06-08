mod game;
use game::game::{Game, GameState};

fn main() {
    let std = std::fs::read_to_string("/Users/hampfh/dev/Projects/rust/hahalang/scripts/std.lua")
        .expect("Could not load standard library");

    let program = std::fs::read_to_string("/Users/hampfh/dev/Projects/rust/hahalang/scripts/forward.temp.lua")
        .expect("Something went wrong reading the file");
    let program_2 = std::fs::read_to_string("/Users/hampfh/dev/Projects/rust/hahalang/scripts/stuck.temp.lua")
        .expect("Something went wrong reading the file");

    let mut app = Game::new(std);
    match app.start(program, program_2) {
        Ok(GameState::PlayerOneWon) => println!("Player 1 won"),
        Ok(GameState::PlayerTwoWon) => println!("Player 1 won"),
        Ok(game_state) => panic!("Unknown gamestate: [{:?}]", game_state),
        Err(reason) => println!("Error: {}", reason)
    }
}


// Load two files
// execute them to load functionality