use crate::game::game::GameResult;
use crate::game::methods;

use super::board::Tile;

pub fn initialize_game_session(script_1: &str, script_2: &str) -> (GameResult, Vec<Vec<Tile>>) {
    let std =
        std::fs::read_to_string("./scripts/std.lua").expect("Could not load standard library");

    let mut game_session = methods::new(std);
    return methods::start(
        &mut game_session,
        script_1.to_string(),
        script_2.to_string(),
    );
}
