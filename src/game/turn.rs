use crate::game::methods::{self, get_active_player_type};

use super::board::populate_board;
use super::game::{Game, Move, ErrorType};
use super::execute_move::execute_move;
use super::map_mirroring::reverse_move;
use super::sandbox::sandbox_executor::execute_lua_in_sandbox;
use super::validation::valid_move;

pub fn on_turn(game: &mut Game) -> Result<(), ErrorType> {

	let player_one_sandbox_mutex = game.player_one_sandbox.clone();
	let player_two_sandbox_mutex = game.player_two_sandbox.clone();

	let player_one = game.player_one.clone();
	let player_two = game.player_two.clone();
	let walls = game.walls.clone();
	let player_one_turn = game.player_one_turn;
	
	let player_move = match execute_lua_in_sandbox(player_one_sandbox_mutex, player_two_sandbox_mutex, walls, player_one, player_two, player_one_turn) {
		Ok(player_move) => player_move,
		Err(error) => return Err(error)
	};

	if player_move.len() != 1 && player_move.len() != 7 {
		return Err(ErrorType::RuntimeError { 
			reason: format!("Invalid input: {}", player_move),
			fault: Some(get_active_player_type(game.player_one_turn))
		});
	}
	
	let mut player_move = convert_player_move_from_string_to_object(Some(player_move));
	match player_move {
		Some(Move::Invalid { reason }) => {
			return Err(ErrorType::GameError { 
				reason,
				fault: Some(get_active_player_type(game.player_one_turn))
			});
		},
		_ => ()
	};

	if should_reverse_player_move(game, &player_move) {
		player_move = Some(reverse_move(player_move.unwrap()));
	}
	
	if player_move.is_none() {
		return Err(ErrorType::GameError { 
			reason: "Player did not return a move".to_string(),
			fault: Some(get_active_player_type(game.player_one_turn))
		});
	}

	match valid_move(game, player_move.clone().unwrap()) {
		Ok(_) => (),
		error => return error
	}
	
	let mut mutable_walls = game.walls.clone();
	// We don't have to check this since we just that the move was valid
	let (first, _) = methods::get_active_player(game);
	execute_move(&mut mutable_walls, first, &player_move.unwrap()).unwrap();
	// Reassign walls
	game.walls = mutable_walls;
	game.player_one_turn = !game.player_one_turn;

	if cfg!(debug_assertions) {
		if game.player_one_turn {
			println!("Player 1 turn");
		}
		else {
			println!("Player 2 turn");
		}
	}

	game.turns.push(populate_board(&game.player_one.clone(), &game.player_two.clone(), &game.walls));

	Ok(())
}

fn should_reverse_player_move(game: &Game, player_move: &Option<Move>) -> bool {
	return 
		!game.player_one_turn && player_move.is_some() && 
		// We compare the enums ONLY, we do not care what reason the fail has
		std::mem::discriminant(&player_move.clone().unwrap()) != std::mem::discriminant(&Move::Invalid { reason: String::new() });
}

fn convert_player_move_from_string_to_object(raw_player_move: Option<String>) -> Option<Move> {
	return match raw_player_move {
		Some(value) => {
			match value.as_str() {
				"0" => Some(Move::Up),
				"1" => Some(Move::Right),
				"2" => Some(Move::Down),
				"3" => Some(Move::Left),
				wall => {
					match methods::deserialize_wall(&wall) {
						Move::Wall(wall) => Some(Move::Wall(wall)),
						Move::Invalid { reason } => Some(Move::Invalid { reason }),
						_ => panic!("Invalid wall")
					}
				}
			}
			
		},
		None => None
	};
}