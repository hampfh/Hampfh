use rlua::Lua;

use crate::game::ascii_graphics::draw_game_in_terminal;
use crate::game::game_state::{
    ErrorType, Game, Move, Wall, INITIAL_WALL_COUNT, MAP_SIZE, MAX_TURNS,
};
use crate::game::player::{Player, PlayerType};
use crate::game::turn;
use std::sync::{Arc, Mutex};
use std::time::Duration;

use super::board::Tile;
use super::game_state::{GameConfig, GameResult};
use super::sandbox::terminate_thread::terminate_thread;

pub(crate) fn new(std: String, config: GameConfig) -> Game {
    let p1 = Player::new(
        MAP_SIZE / 2,
        MAP_SIZE - 1,
        INITIAL_WALL_COUNT,
        PlayerType::Flipped,
    );
    let p2 = Player::new(MAP_SIZE / 2, 0, INITIAL_WALL_COUNT, PlayerType::Regular);
    let walls = Vec::new();
    return custom_new(p1, p2, walls, std, config);
}

pub(crate) fn custom_new(
    player_one: Player,
    player_two: Player,
    walls: Vec<Wall>,
    std: String,
    config: GameConfig,
) -> Game {
    return Game {
        config,
        logger: Vec::new(),
        running: true,
        game_result: None,
        player_one,
        player_two,
        walls,
        player_one_sandbox: Arc::new(Mutex::new(Lua::new())),
        player_two_sandbox: Arc::new(Mutex::new(Lua::new())),
        player_one_turn: true,
        last_move: None,
        std,
        turns: Vec::new(),
    };
}

pub(crate) fn start(
    game: &mut Game,
    program1: String,
    program2: String,
) -> (GameResult, Vec<Vec<Tile>>, Vec<Move>) {
    let std = game.std.clone();

    let clone_one = game.player_one_sandbox.clone();
    let clone_two = game.player_two_sandbox.clone();

    match assert_lua_core_functions(program1.clone(), PlayerType::Flipped) {
        Ok(_) => (),
        Err(error) => return (GameResult::Error(error), Vec::new(), game.logger.clone()),
    }
    match assert_lua_core_functions(program2.clone(), PlayerType::Regular) {
        Ok(_) => (),
        Err(error) => return (GameResult::Error(error), Vec::new(), game.logger.clone()),
    }

    // Run programs for the first time
    // We limit execution here to 100 milli-seconds
    let (tx, rx) = std::sync::mpsc::channel::<Result<Option<usize>, String>>();
    std::thread::spawn(move || {
        tx.send(Ok(Some(thread_id::get()))).unwrap();

        let player_one_sandbox = clone_one.lock().unwrap();
        let player_two_sandbox = clone_two.lock().unwrap();

        match player_one_sandbox.context(|ctx| ctx.load(&program1).exec()) {
            Ok(_) => (),
            Err(err) => {
                tx.send(Err(format!(
                    "Your script could not be executed, reason: {}",
                    err
                )))
                .unwrap();
            }
        }
        match player_two_sandbox.context(|ctx| ctx.load(&program2).exec()) {
            Ok(_) => (),
            Err(_) => {
                tx.send(Err("Opponent script could not be executed".to_string()))
                    .unwrap();
            }
        }

        drop(player_one_sandbox);
        drop(player_two_sandbox);
        tx.send(Ok(None)).unwrap();
    });

    let thread_id = rx.recv().unwrap().unwrap().unwrap();
    match rx.recv_timeout(Duration::from_millis(100)) {
        Ok(Ok(_)) => (),
        Ok(Err(err)) => {
            return (
                GameResult::Error(ErrorType::RuntimeError {
                    reason: err.to_string(),
                    fault: Some(get_active_player_type(game.player_one_turn)),
                }),
                game.turns.clone(),
                game.logger.clone(),
            );
        }
        Err(_) => {
            terminate_thread(thread_id);
            return (
                GameResult::Error(ErrorType::TurnTimeout {
                    fault: Some(get_active_player_type(game.player_one_turn)),
                }),
                game.turns.clone(),
                game.logger.clone(),
            );
        }
    }

    // Load standard library
    // We don't have to check errors here since
    // this is deterministic and all lua code
    // comes from us
    let first = game.player_one_sandbox.clone();
    first
        .lock()
        .unwrap()
        .context(|ctx| ctx.load(&std).exec())
        .unwrap();
    let second = game.player_two_sandbox.clone();
    second
        .lock()
        .unwrap()
        .context(|ctx| ctx.load(&std).exec())
        .unwrap();

    // Unlock mutexes
    drop(first);
    drop(second);

    game_loop(game);

    return match game.game_result.clone() {
        Some(game_result) => (game_result, game.turns.clone(), game.logger.clone()),
        None => (
            GameResult::Error(ErrorType::GameError {
                reason: format!("Unknown match end"),
                fault: None,
            }),
            game.turns.clone(),
            game.logger.clone(),
        ),
    };
}

pub(crate) fn game_loop(game: &mut Game) {
    let mut round = 1;
    while game.running {
        let result = turn::on_turn(game);
        match result {
            Ok(_) => (),
            Err(err) => {
                game.running = false;
                game.game_result = Some(GameResult::Error(err));
            }
        }

        if game.config.live_print_match {
            draw_game_in_terminal(&game);
        }

        // Check if game is over
        if game.player_one.y == 0 {
            game.running = false;
            game.game_result = Some(GameResult::PlayerOneWon);
        } else if game.player_two.y == MAP_SIZE - 1 {
            game.running = false;
            game.game_result = Some(GameResult::PlayerTwoWon);
        }

        if round >= MAX_TURNS {
            game.running = false;
            game.game_result = Some(GameResult::Error(ErrorType::GameDeadlock));
        }
        round += 1;
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

fn assert_lua_core_functions(program: String, player_type: PlayerType) -> Result<(), ErrorType> {
    if !program.contains("function onTurn(") {
        return Err(ErrorType::RuntimeError {
            reason: "onTurn() function not found, this function is mandatory".to_string(),
            fault: Some(player_type),
        });
    }
    if !program.contains("function onJump(") {
        return Err(ErrorType::RuntimeError {
            reason: "onJump() function not found, this function is mandatory".to_string(),
            fault: Some(player_type),
        });
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::game::{game::assert_lua_core_functions, player::PlayerType};

    #[test]
    fn correctly_checks_if_core_functions_exist() {
        let program = r#"
            function onTurn()
                return "1"
            end 
            function onJump()
                return "1"
            end
        "#;

        assert_eq!(
            assert_lua_core_functions(String::from(program), PlayerType::Regular),
            Ok(())
        );
        assert_eq!(
            assert_lua_core_functions(String::from(program), PlayerType::Flipped),
            Ok(())
        );

        let only_on_turn = r#"
            function onTurn()
                return "1"
            end
        "#;
        assert_eq!(
            assert_lua_core_functions(String::from(only_on_turn), PlayerType::Regular),
            Err(crate::game::game_state::ErrorType::RuntimeError {
                reason: "onJump() function not found, this function is mandatory".to_string(),
                fault: Some(PlayerType::Regular)
            })
        );

        let only_on_jump = r#"
            function onJump()
                return "1"
            end
            "#;
        assert_eq!(
            assert_lua_core_functions(String::from(only_on_jump), PlayerType::Regular),
            Err(crate::game::game_state::ErrorType::RuntimeError {
                reason: "onTurn() function not found, this function is mandatory".to_string(),
                fault: Some(PlayerType::Regular)
            })
        );
    }
}
