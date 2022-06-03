use super::game::{self, Game, Move};
use super::execute_move::execute_move;
use super::map_mirroring::{reverse_move, conditionally_reverse_coordinates, conditionally_reverse_move};
use super::validation::valid_move;

pub fn on_turn(game: &mut Game) -> Result<(), String> {
	let (x, y) = conditionally_reverse_coordinates(game.get_enemy_coords(), !game.player_one_turn);

	let last_move: String = playermove_to_string(&conditionally_reverse_move(game.last_move.clone(), !game.player_one_turn));

	let starting_script = get_lua_starting_script(game, last_move, x, y);

	let active_sandbox = if game.player_one_turn {
		&mut game.player_one_sandbox
	} else {
		&mut game.player_two_sandbox
	};

	active_sandbox.execute::<()>(&starting_script).unwrap();

	// TODO (Security) here we should add a timeout for the script to run
	let raw_player_move: Option<String> = active_sandbox.get("ExternalGlobalVarResult");
	println!("Raw Player move {}", raw_player_move.clone().unwrap());

	let mut player_move = convert_player_move_from_string_to_object(raw_player_move);
	println!("Raw Player move {:?}", player_move.clone().unwrap());

	if should_reverse_player_move(game, &player_move) {
		player_move = Some(reverse_move(player_move.unwrap()));
	}
	
	if player_move.is_none() {
		return Err("Player did not return a move".to_string());
	}
	match valid_move(game, player_move.clone().unwrap()) {
		Ok(_) => (),
		Err(reason) => {
			return Err(reason);
		}
	}

	execute_move(game, &player_move.unwrap());
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

fn get_lua_starting_script(game: &Game, last_move: String, x: i32, y: i32) -> String {
	let reverse_coordinates = !game.player_one_turn;
	return format!(
		"ExternalGlobalVarResult = onTurn({}, {}, {}, {})", 
		last_move, 
		x, 
		y, 
		game.serialize_walls()
	);
}

fn playermove_to_string(last_move: &Option<Move>) -> String {
	return match last_move.clone() {
		Some(Move::Up) => "0".to_string(),
		Some(Move::Left) => "1".to_string(),
		Some(Move::Down) => "2".to_string(),
		Some(Move::Right) => "3".to_string(),
		Some(Move::Wall(wall)) => game::serialize_wall(&wall),
		Some(Move::Invalid { reason: _ }) => "nil".to_string(),
		None => "nil".to_string() 
	};
}

fn convert_player_move_from_string_to_object(raw_player_move: Option<String>) -> Option<Move> {
	return match raw_player_move {
		Some(value) => {
			match value.as_str() {
				"0" => Some(Move::Up),
				"1" => Some(Move::Left),
				"2" => Some(Move::Down),
				"3" => Some(Move::Right),
				wall => {
					match game::deserialize_wall(&wall) {
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