use crate::terminate_thread::terminate_thread;

use std::time::Duration;
use thread_id;

use crate::game::game::MAP_SIZE;
use crate::game::methods;
use super::player::Player;
use super::board::{serialize_board, populate_board};
use super::game::{Wall, Game, Move, ErrorType};
use super::execute_move::execute_move;
use super::map_mirroring::{reverse_move, conditionally_reverse_player, conditionally_reverse_walls};
use super::validation::valid_move;

struct ThreadReturn {
	thread_id: Option<usize>,
	player_move: Option<String>,
}

pub fn on_turn(game: &mut Game) -> Result<(), ErrorType> {

	let player_one_sandbox_mutex = game.player_one_sandbox.clone();
	let player_two_sandbox_mutex = game.player_two_sandbox.clone();

	let player_one = game.player_one.clone();
	let player_two = game.player_two.clone();
	let walls = game.walls.clone();
	let player_one_turn = game.player_one_turn;

	// Sandbox execution of script
	// Limit execution time to 1 second
	let (tx, rx) = std::sync::mpsc::channel::<ThreadReturn>();
	std::thread::spawn(move || {
		tx.send(ThreadReturn {
			thread_id: Some(thread_id::get()),
			player_move: None
		}).unwrap();

		let starting_script = get_lua_starting_script(create_lua_game_object(walls, player_one_turn, player_one, player_two));
		
		let mut active_sandbox = player_one_sandbox_mutex.lock().unwrap();
		if !player_one_turn {
			drop(active_sandbox);
			active_sandbox = player_two_sandbox_mutex.lock().unwrap();
		}
		
		println!("Starting script");
		match active_sandbox.execute::<()>(&starting_script) {
			Ok(_) => (),
			Err(err) => {
				tx.send(ThreadReturn {
					thread_id: None,
					player_move: Some(err.to_string())
				}).unwrap();
			}
		}
		println!("ENDING script");

		let raw_player_move: Option<String> = active_sandbox.get("ExternalGlobalVarResult");
		drop(active_sandbox);
		
		tx.send(ThreadReturn {
			thread_id: None,
			player_move: raw_player_move
		}).unwrap();
	});

	// First time we send the thread id through
	// This does not have to be timed checked since this is before we
	// execute the script.
	let sandbox_thread_id = match rx.recv().unwrap().thread_id {
		Some(id) => id,
		_ => panic!("Could not get thread id"),
	};

	// Second time we either get the result or a timeout error
	let player_move = match rx.recv_timeout(Duration::from_millis(500)) {
		Ok(returned) => returned.player_move,
		Err(_) => {
			println!("Timed out");
			terminate_thread(sandbox_thread_id);
			return Err(ErrorType::TurnTimeout);
		}
	};

	if player_move.is_some() && player_move.clone().unwrap().len() != 1 && player_move.clone().unwrap().len() != 7 {
		return Err(ErrorType::RuntimeError { reason: player_move.unwrap() });
	}
	
	let mut player_move = convert_player_move_from_string_to_object(player_move);
	println!("Player move {:?}", player_move);
	match player_move {
		Some(Move::Invalid { reason }) => {
			return Err(ErrorType::GameError { reason: reason });
		},
		_ => ()
	};

	if should_reverse_player_move(game, &player_move) {
		player_move = Some(reverse_move(player_move.unwrap()));
	}
	
	if player_move.is_none() {
		return Err(ErrorType::GameError { reason: "Player did not return a valid move".to_string() });
	}
	match valid_move(game, player_move.clone().unwrap()) {
		Ok(_) => (),
		Err(reason) => {
			return Err(ErrorType::GameError { reason: reason });
		}
	}
	
	let mut mutable_walls = game.walls.clone();
	execute_move(&mut mutable_walls, methods::get_active_player(game).0, &player_move.unwrap());
	// Reassign walls
	game.walls = mutable_walls;
	game.player_one_turn = !game.player_one_turn;

	if game.player_one_turn {
		println!("Player 1 turn");
	}
	else {
		println!("Player 2 turn");
	}

	Ok(())
}

fn should_reverse_player_move(game: &Game, player_move: &Option<Move>) -> bool {
	return 
		!game.player_one_turn && player_move.is_some() && 
		// We compare the enums ONLY, we do not care what reason the fail has
		std::mem::discriminant(&player_move.clone().unwrap()) != std::mem::discriminant(&Move::Invalid { reason: String::new() });
}

fn get_lua_starting_script(game_object: String) -> String {
	return format!(
		"ExternalGlobalVarResult = onTurn({})", 
		game_object
	);
}

fn create_lua_game_object(walls: Vec<Wall>, player_one_turn: bool, player_one: Player, player_two: Player) -> String {
	let reverse = !player_one_turn;

	let walls = conditionally_reverse_walls(&walls, reverse);

	let serialized_board = serialize_board(populate_board(&player_one, &player_two, &walls));
	let (serialized_player, serialized_opponent) = match player_one_turn {
		true => (
			serialize_player(&conditionally_reverse_player(&player_one, false)), 
			serialize_player(&conditionally_reverse_player(&player_two, false))
		),
		false => (
			serialize_player(&conditionally_reverse_player(&player_two, true)), 
			serialize_player(&conditionally_reverse_player(&player_one, true))
		)
	};

	println!("Player pos ({})", serialized_player);
	println!("Walls {:?}", walls);
	println!("Serialized board ({:?})", populate_board(&player_one, &player_two, &walls)[(2 + MAP_SIZE * player_one.y - 1) as usize]);

	return format!("{{player={}, opponent={}, board={}}}", serialized_player, serialized_opponent, serialized_board);
}

fn serialize_player(player: &Player) -> String {
	return format!("{{x={}, y={}, wall_count={}}}", player.x, player.y, player.wall_count)	
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