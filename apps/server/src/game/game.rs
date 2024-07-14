use std::sync::{Arc, Mutex};

use mlua::Lua;

use crate::game::ascii_graphics::draw_game_in_terminal;
use crate::game::game_state::{ErrorType, Game, Move, MAP_SIZE, MAX_TURNS};
use crate::game::player::{Player, PlayerType};
use crate::game::turn;

use super::board::Tile;
use super::game_state::GameResult;
use super::load_script::load_script;
use super::sandbox::sandbox_executor::{assert_lua_core_functions, execute_lua_in_sandbox};

impl Game {
    pub(crate) fn start(
        &mut self,
        program1: String,
        program2: String,
    ) -> (GameResult, Vec<Vec<Tile>>, Vec<Move>) {
        match assert_lua_core_functions(program1.clone(), PlayerType::Flipped) {
            Ok(_) => (),
            Err(error) => {
                return (
                    GameResult::Error(error),
                    self.turns.clone(),
                    self.logger.clone(),
                )
            }
        }
        match assert_lua_core_functions(program2.clone(), PlayerType::Regular) {
            Ok(_) => (),
            Err(error) => {
                return (
                    GameResult::Error(error),
                    self.turns.clone(),
                    self.logger.clone(),
                )
            }
        }

        match execute_lua_in_sandbox(
            self.player_one_sandbox.clone(),
            program1,
            PlayerType::Flipped,
            false,
            self.config.bot_initialization_timeout,
        ) {
            Ok(_) => (),
            Err(err) => {
                return (
                    GameResult::Error(err),
                    self.turns.clone(),
                    self.logger.clone(),
                )
            }
        }
        load_script(&self.player_one_sandbox, self.std.clone());

        match execute_lua_in_sandbox(
            self.player_two_sandbox.clone(),
            program2,
            PlayerType::Regular,
            false,
            self.config.bot_initialization_timeout,
        ) {
            Ok(_) => (),
            Err(err) => {
                return (
                    GameResult::Error(err),
                    self.turns.clone(),
                    self.logger.clone(),
                )
            }
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

    pub(crate) fn get_active_sandbox(&self) -> Arc<Mutex<Lua>> {
        if self.player_one_turn {
            return self.player_one_sandbox.clone();
        } else {
            return self.player_two_sandbox.clone();
        }
    }
    /**
     * Returns a tuple, the first player is always the active one
     * the second is the non-active player
     */
    pub(crate) fn get_active_player(&mut self) -> (&mut Player, &Player) {
        if self.player_one_turn {
            return (&mut self.player_one, &self.player_two);
        }
        return (&mut self.player_two, &self.player_one);
    }

    pub(crate) fn get_active_player_type(&self) -> PlayerType {
        if self.player_one_turn {
            return PlayerType::Flipped;
        }
        return PlayerType::Regular;
    }
}
