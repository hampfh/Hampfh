use crate::game::methods::{self, get_active_player_type};

use super::board::populate_board;
use super::execute_move::{execute_move, execute_move_jump};
use super::game::{ErrorType, Game, Move};
use super::map_mirroring::reverse_move;
use super::sandbox::sandbox_executor::execute_lua_in_sandbox;
use super::validation::valid_move;

pub(super) fn on_turn(game: &mut Game) -> Result<(), ErrorType> {
    let player_one_sandbox_mutex = game.player_one_sandbox.clone();
    let player_two_sandbox_mutex = game.player_two_sandbox.clone();

    let player_one = game.player_one.clone();
    let player_two = game.player_two.clone();
    let walls = game.walls.clone();
    let player_one_turn = game.player_one_turn;

    let player_move = match execute_lua_in_sandbox(
        player_one_sandbox_mutex.clone(),
        player_two_sandbox_mutex.clone(),
        walls.clone(),
        player_one.clone(),
        player_two.clone(),
        player_one_turn,
        "onTurn".to_string(),
    ) {
        Ok(player_move) => player_move,
        Err(error) => return Err(error),
    };

    let debugging_enabled = std::env::var("DEBUG")
        .unwrap_or(String::from("false"))
        .to_lowercase()
        == "true";

    // Check for debug flag
    let split = player_move.split(" ");
    if debugging_enabled && split.clone().count() > 0 && split.clone().next().unwrap() == "#debug" {
        println!("Incoming {}", player_move);
        return Err(ErrorType::GameError {
            reason: format!(
                "Player: {:?}\n<br/>Opponent: {:?}\n<br/>Walls: {:?}\n<br/>Bot ({}) debugging log:\n```\n{}\n```\n<br/>",
                player_one, player_two, walls, if player_one_turn {"🟩"} else {"🟥"}, split.skip(1).collect::<Vec<&str>>().join(" ")
            ),
            fault: None,
        });
    }

    // onTurn fail if: not 1 and not 7
    // onJump fail if: not 1
    if player_move.len() != 1 && player_move.len() != 7 {
        return Err(ErrorType::RuntimeError {
            reason: format!("Invalid input: {}", player_move),
            fault: Some(get_active_player_type(game.player_one_turn)),
        });
    }

    let mut player_move = convert_player_move_from_string_to_object(Some(player_move));
    if let Some(Move::Invalid { reason }) = player_move {
        return Err(ErrorType::GameError {
            reason,
            fault: Some(get_active_player_type(game.player_one_turn)),
        });
    }

    if should_reverse_player_move(player_one_turn, &player_move) {
        player_move = Some(reverse_move(player_move.unwrap()));
    }

    if player_move.is_none() {
        return Err(ErrorType::GameError {
            reason: "Player did not return a move".to_string(),
            fault: Some(get_active_player_type(game.player_one_turn)),
        });
    }

    let (active_player, opponent) = match player_one_turn {
        true => (player_one.clone(), player_two.clone()),
        false => (player_two.clone(), player_one.clone()),
    };

    let run_on_jump = match valid_move(
        player_one_turn,
        &active_player,
        &opponent,
        &walls,
        player_move.clone().unwrap(),
    ) {
        Ok(value) => !value,
        Err(error) => return Err(error),
    };

    let mut mutable_walls = game.walls.clone();
    let (first, other) = methods::get_active_player(game);

    if run_on_jump {
        // TODO refactor this, this should recursivly call on turn again, instead of this code repeat
        let on_jump_player_move = match execute_lua_in_sandbox(
            player_one_sandbox_mutex,
            player_two_sandbox_mutex,
            walls.clone(),
            player_one.clone(),
            player_two.clone(),
            player_one_turn,
            "onJump".to_string(),
        ) {
            Ok(player_move) => player_move,
            Err(error) => return Err(error),
        };

        if on_jump_player_move.len() != 1 {
            return Err(ErrorType::GameError {
                reason: format!(
                    "Invalid return format from onJump, return can only be a number between 0-3"
                ),
                fault: Some(get_active_player_type(game.player_one_turn)),
            });
        }
        let mut converted_on_jump_player_move =
            convert_player_move_from_string_to_object(Some(on_jump_player_move));
        if let Some(Move::Invalid { reason }) = converted_on_jump_player_move {
            return Err(ErrorType::GameError {
                reason,
                fault: Some(get_active_player_type(game.player_one_turn)),
            });
        }

        if should_reverse_player_move(player_one_turn, &converted_on_jump_player_move) {
            converted_on_jump_player_move =
                Some(reverse_move(converted_on_jump_player_move.unwrap()));
        }

        if converted_on_jump_player_move.is_none() {
            return Err(ErrorType::GameError {
                reason: "Player did not return a move".to_string(),
                fault: Some(get_active_player_type(game.player_one_turn)),
            });
        }

        if let Err(error) = execute_move_jump(
            first,
            other,
            &converted_on_jump_player_move.clone().unwrap(),
        ) {
            return Err(error);
        }

        // Check that move was correct
        if first.x == other.x && first.y == other.y {
            return Err(ErrorType::GameError {
                reason: format!(
                    "Player ended up on top of opponent in jump at ({}, {})",
                    first.x, first.y
                ),
                fault: Some(get_active_player_type(game.player_one_turn)),
            });
        }

        for wall in &mutable_walls {
            if wall.x1 == first.x && wall.y1 == first.y || wall.x2 == first.x && wall.y2 == first.y
            {
                return Err(ErrorType::GameError {
                    reason: format!(
                        "Player tried to jump into a wall at ({}, {})",
                        first.x, first.y
                    ),
                    fault: Some(get_active_player_type(game.player_one_turn)),
                });
            }
        }
    } else {
        execute_move(&mut mutable_walls, first, &player_move.clone().unwrap()).unwrap();
        // Reassign walls
        game.walls = mutable_walls;
    }

    game.player_one_turn = !game.player_one_turn;

    game.turns.push(populate_board(
        &game.player_one.clone(),
        &game.player_two.clone(),
        &game.walls,
    ));

    Ok(())
}

fn should_reverse_player_move(player_one_turn: bool, player_move: &Option<Move>) -> bool {
    return !player_one_turn && player_move.is_some() &&
		// We compare the enums ONLY, we do not care what reason the fail has
		std::mem::discriminant(&player_move.clone().unwrap()) != std::mem::discriminant(&Move::Invalid { reason: String::new() });
}

fn convert_player_move_from_string_to_object(raw_player_move: Option<String>) -> Option<Move> {
    return match raw_player_move {
        Some(value) => match value.as_str() {
            "0" => Some(Move::Up),
            "1" => Some(Move::Right),
            "2" => Some(Move::Down),
            "3" => Some(Move::Left),
            wall => match methods::deserialize_wall(&wall) {
                Move::Wall(wall) => Some(Move::Wall(wall)),
                Move::Invalid { reason } => Some(Move::Invalid { reason }),
                _ => panic!("Invalid wall"),
            },
        },
        None => None,
    };
}
