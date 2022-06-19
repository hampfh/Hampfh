use super::game::{Game, GameState};

pub fn initialize_game_session(script_1: &str, script_2: &str) -> Result<GameState, String> {
    let std = std::fs::read_to_string("/Users/hampfh/dev/Projects/rust/hahalang/scripts/std.lua")
        .expect("Could not load standard library");

    let mut game_session = Game::new(std);
    match game_session.start(script_1.to_string(), script_2.to_string()) {
        Ok(GameState::PlayerOneWon) => Ok(GameState::PlayerOneWon),
        Ok(GameState::PlayerTwoWon) => Ok(GameState::PlayerTwoWon),
        Ok(game_state) => Err(format!("Invalid game state {:?}", game_state)),
        Err(reason) => Err(format!("[Error] Game interupted with error: {}", reason))
    }
}