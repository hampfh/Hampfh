use crate::game::game;
use crate::game::game_state::GameResult;

use super::board::Tile;
use super::game_state::{GameConfig, Move};

pub(crate) fn initialize_game(
    script_1: &str,
    script_2: &str,
    config: GameConfig,
) -> (GameResult, Vec<Vec<Tile>>, Vec<Move>) {
    let std =
        std::fs::read_to_string("../scripts/std.lua").expect("Could not load standard library");

    let mut game_session = game::new(std, config);
    return game::start(
        &mut game_session,
        script_1.to_string(),
        script_2.to_string(),
    );
}
