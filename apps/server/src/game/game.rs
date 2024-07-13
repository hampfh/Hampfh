use crate::game::ascii_graphics::draw_game_in_terminal;
use crate::game::game_state::{ErrorType, Game, Move, MAP_SIZE, MAX_TURNS};
use crate::game::player::{Player, PlayerType};
use crate::game::turn;

use super::board::Tile;
use super::game_state::GameResult;
use super::load_script::load_script;
use super::load_script_with_validation::load_script_with_validation;

impl Game {
    pub(crate) fn start(
        &mut self,
        program1: String,
        program2: String,
    ) -> (GameResult, Vec<Vec<Tile>>, Vec<Move>) {
        match load_script_with_validation(&self.player_one_sandbox, program1, PlayerType::Flipped) {
            Ok(_) => (),
            Err(err) => return (err, self.turns.clone(), self.logger.clone()),
        }
        load_script(&self.player_one_sandbox, self.std.clone());

        match load_script_with_validation(&self.player_two_sandbox, program2, PlayerType::Regular) {
            Ok(_) => (),
            Err(err) => return (err, self.turns.clone(), self.logger.clone()),
        }
        load_script(&self.player_two_sandbox, self.std.clone());

        self.game_loop();

        return match self.game_result.clone() {
            Some(game_result) => (game_result, self.turns.clone(), self.logger.clone()),
            None => (
                GameResult::Error(ErrorType::GameError {
                    reason: format!("Unknown match end"),
                    fault: None,
                }),
                self.turns.clone(),
                self.logger.clone(),
            ),
        };
    }

    pub(crate) fn game_loop(&mut self) {
        let mut round = 1;
        while self.running {
            let result = turn::on_turn(self);
            match result {
                Ok(_) => (),
                Err(err) => {
                    self.running = false;
                    self.game_result = Some(GameResult::Error(err));
                }
            }

            if self.config.live_print_match {
                draw_game_in_terminal(&self);
            }

            // Check if game is over
            if self.player_one.y == 0 {
                self.running = false;
                self.game_result = Some(GameResult::PlayerOneWon);
            } else if self.player_two.y == MAP_SIZE - 1 {
                self.running = false;
                self.game_result = Some(GameResult::PlayerTwoWon);
            }

            if round >= MAX_TURNS {
                self.running = false;
                self.game_result = Some(GameResult::Error(ErrorType::GameDeadlock));
            }
            round += 1;
        }
    }
}

/**
 * Returns a tuple, the first player is always the active one
 * the second is the non-active player
 */
pub(crate) fn get_active_player(game: &mut Game) -> (&mut Player, &Player) {
    if game.player_one_turn {
        return (&mut game.player_one, &game.player_two);
    }
    return (&mut game.player_two, &game.player_one);
}
pub fn get_active_player_type(player_one_turn: bool) -> PlayerType {
    if player_one_turn {
        return PlayerType::Flipped;
    }
    return PlayerType::Regular;
}
