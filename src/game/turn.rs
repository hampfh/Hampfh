use crate::game::game::get_active_player;
use super::player::Player;
use super::board::{serialize_board, populate_board};
use super::game::{self, Game, Move};
use super::execute_move::execute_move;
use super::map_mirroring::{reverse_move, conditionally_reverse_player, conditionally_reverse_walls};
use super::validation::valid_move;

pub fn on_turn(game: &mut Game) -> Result<(), String> {
	let starting_script = get_lua_starting_script(create_lua_game_object(&game));

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
	match player_move {
		Some(Move::Invalid { reason }) => {
			return Err(String::from(reason));
		},
		_ => ()
	};
	println!("Player move {:?}", player_move.clone().unwrap());

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
	
	let mut walls = game.walls.clone();
	let (active_player, other) = get_active_player(game);
	execute_move(&mut walls, active_player, &other, &player_move.unwrap());
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

fn create_lua_game_object(game: &Game) -> String {
	let reverse = !game.player_one_turn;

	let walls = conditionally_reverse_walls(&game.walls.clone(), reverse);

	let serialized_board = serialize_board(populate_board(game, &walls));
	let (serialized_player, serialized_opponent) = match game.player_one_turn {
		true => (
			serialize_player(&conditionally_reverse_player(&game.player_one, reverse)), 
			serialize_player(&conditionally_reverse_player(&game.player_two, reverse))
		),
		false => (
			serialize_player(&conditionally_reverse_player(&game.player_two, reverse)), 
			serialize_player(&conditionally_reverse_player(&game.player_one, reverse))
		)
	};

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