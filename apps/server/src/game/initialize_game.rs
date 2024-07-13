use crate::game::game_state::GameResult;

use super::board::Tile;
use super::game_state::{Game, GameConfig, Move};

pub(crate) fn initialize_game(
    script_1: &str,
    script_2: &str,
    config: GameConfig,
) -> (GameResult, Vec<Vec<Tile>>, Vec<Move>) {
    let mut game_session = Game::new(config);
    return game_session.start(script_1.to_string(), script_2.to_string());
}
