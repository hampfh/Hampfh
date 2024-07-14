use super::board::populate_board;
use super::execute_move::{execute_move, execute_move_jump};
use super::game_state::{ErrorType, Game, Move};
use super::map_mirroring::reverse_move;
use super::parsing::deserialize_wall::deserialize_wall;
use super::sandbox::sandbox_executor::{
    create_lua_game_object, execute_lua_in_sandbox, get_lua_start_inject,
};
use super::validation::valid_move;

pub(super) fn on_turn(game: &mut Game) -> Result<(), ErrorType> {
    let player_one = game.player_one.clone();
    let player_two = game.player_two.clone();
    let walls = game.walls.clone();
    let player_one_turn = game.player_one_turn;
    let active_sandbox = game.get_active_sandbox();

    let lua_script_to_run = get_lua_start_inject(
        "onTurn".to_string(),
        create_lua_game_object(&walls, player_one_turn, &player_one, &player_two),
    );
    let player_move = match execute_lua_in_sandbox(
        active_sandbox.clone(),
        lua_script_to_run,
        game.get_active_player_type(),
        true,
        game.config.bot_turn_timeout,
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
            fault: Some(game.get_active_player_type()),
        });
    }

    let mut player_move = convert_player_move_from_string_to_object(Some(player_move));
    if let Some(Move::Invalid { reason }) = player_move {
        return Err(ErrorType::GameError {
            reason,
            fault: Some(game.get_active_player_type()),
        });
    }

    if should_reverse_player_move(player_one_turn, &player_move) {
        player_move = Some(reverse_move(player_move.unwrap()));
    }

    if player_move.is_none() {
        return Err(ErrorType::GameError {
            reason: "Player did not return a move".to_string(),
            fault: Some(game.get_active_player_type()),
        });
    }

    // Add move to logger
    game.logger.push(player_move.clone().unwrap());

    let (active_player, opponent) = match player_one_turn {
        true => (player_one.clone(), player_two.clone()),
        false => (player_two.clone(), player_one.clone()),
    };

    let run_on_jump = match valid_move(
        &active_player,
        &opponent,
        &walls,
        player_move.clone().unwrap(),
        game.get_active_player_type(),
    ) {
        Ok(value) => !value,
        Err(error) => return Err(error),
    };

    let mut mutable_walls = game.walls.clone();

    if run_on_jump {
        let lua_script_to_run = get_lua_start_inject(
            "onJump".to_string(),
            create_lua_game_object(&walls, player_one_turn, &player_one, &player_two),
        );
        // TODO refactor this, this should recursivly call on turn again, instead of this code repeat
        let on_jump_player_move = match execute_lua_in_sandbox(
            active_sandbox,
            lua_script_to_run,
            game.get_active_player_type(),
            true,
            game.config.bot_turn_timeout,
        ) {
            Ok(player_move) => player_move,
            Err(error) => return Err(error),
        };

        if on_jump_player_move.len() != 1 {
            return Err(ErrorType::GameError {
                reason: format!(
                    "Invalid return format from onJump, return can only be a number between 0-3"
                ),
                fault: Some(game.get_active_player_type()),
            });
        }
        let mut converted_on_jump_player_move =
            convert_player_move_from_string_to_object(Some(on_jump_player_move));
        if let Some(Move::Invalid { reason }) = converted_on_jump_player_move {
            return Err(ErrorType::GameError {
                reason,
                fault: Some(game.get_active_player_type()),
            });
        }

        if should_reverse_player_move(player_one_turn, &converted_on_jump_player_move) {
            converted_on_jump_player_move =
                Some(reverse_move(converted_on_jump_player_move.unwrap()));
        }

        if converted_on_jump_player_move.is_none() {
            return Err(ErrorType::GameError {
                reason: "Player did not return a move".to_string(),
                fault: Some(game.get_active_player_type()),
            });
        }

        let (active_player, other_player) = game.get_active_player();

        if let Err(error) = execute_move_jump(
            active_player,
            other_player,
            &converted_on_jump_player_move.clone().unwrap(),
        ) {
            return Err(error);
        }

        // Check that move was correct
        if active_player.x == other_player.x && active_player.y == other_player.y {
            return Err(ErrorType::GameError {
                reason: format!(
                    "Player ended up on top of opponent in jump at ({}, {})",
                    active_player.x, active_player.y
                ),
                fault: Some(game.get_active_player_type()),
            });
        }

        for wall in &mutable_walls {
            if wall.x1 == active_player.x && wall.y1 == active_player.y
                || wall.x2 == active_player.x && wall.y2 == active_player.y
            {
                return Err(ErrorType::GameError {
                    reason: format!(
                        "Player tried to jump into a wall at ({}, {})",
                        active_player.x, active_player.y
                    ),
                    fault: Some(game.get_active_player_type()),
                });
            }
        }
    } else {
        execute_move(
            &mut mutable_walls,
            game.get_active_player().0,
            &player_move.clone().unwrap(),
        )
        .unwrap();
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

/// Check if we should reverse the player move
///
/// We do not reverse if it is player one's turn,
/// or if there player move is an error.
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
            wall => match deserialize_wall(&wall) {
                Move::Wall(wall) => Some(Move::Wall(wall)),
                Move::Invalid { reason } => Some(Move::Invalid { reason }),
                _ => panic!("Invalid wall"),
            },
        },
        None => None,
    };
}

#[cfg(test)]
mod tests {
    use crate::game::game_state::{Move, Wall};

    use super::convert_player_move_from_string_to_object;

    #[test]
    fn correctly_determines_when_to_reverse_player_move() {
        assert_eq!(
            super::should_reverse_player_move(true, &Some(Move::Up)),
            false
        );
        assert_eq!(
            super::should_reverse_player_move(
                true,
                &Some(Move::Invalid {
                    reason: String::new()
                })
            ),
            false
        );
        assert_eq!(
            super::should_reverse_player_move(false, &Some(Move::Up)),
            true
        );
        assert_eq!(
            super::should_reverse_player_move(
                false,
                &Some(Move::Invalid {
                    reason: String::new()
                })
            ),
            false
        );
    }

    #[test]
    fn correctly_convert_player_move() {
        assert_eq!(
            convert_player_move_from_string_to_object(Some(format!("0"))),
            Some(Move::Up)
        );
        assert_eq!(
            convert_player_move_from_string_to_object(Some(format!("1"))),
            Some(Move::Right)
        );
        assert_eq!(
            convert_player_move_from_string_to_object(Some(format!("2"))),
            Some(Move::Down)
        );
        assert_eq!(
            convert_player_move_from_string_to_object(Some(format!("3"))),
            Some(Move::Left)
        );
        assert_eq!(
            convert_player_move_from_string_to_object(Some(format!("0,0,0,1"))),
            Some(Move::Wall(Wall {
                x1: 0,
                y1: 0,
                x2: 0,
                y2: 1
            }))
        );
    }
}
