use crate::game::path_find::path_exists_for_players;
use super::player::Player;
use super::execute_move::execute_move;
use super::game::get_active_player;
use super::game::{MAP_SIZE, Game, Move, Wall};

pub fn valid_move(game: &mut Game, player_move: Move) -> Result<(), String> {

	// Reverse board for player two
	let reverse_board = !game.player_one_turn;

	let mut temp_walls = game.walls.clone();
	let (active_player, other) = get_active_player(game);
	let mut temp_active_player = active_player.clone();
	execute_move(&mut temp_walls, &mut temp_active_player, other, &player_move);

	let (success, reason) = valid_tile(&temp_walls, other, other, temp_active_player.x, temp_active_player.y);
	if !success {
		return Err(format!("Invalid move: {}", reason.unwrap()));
	}

	if path_exists_for_players(game) {
		println!("Path exists");
	} else {
		println!("Path does not exist");
	}

	Ok(())
}

pub fn valid_tile(walls: &Vec<Wall>, player_one: &Player, player_two: &Player, x: i32, y: i32) -> (bool, Option<String>) {
	if tile_occupied(walls, player_one, player_two, x, y) {
		return (false, Some(format!("Tile ({},{}) is occupied", x, y)));
	} 
	if out_of_bounds(x, y) {
		return (false, Some("Tile is out of bounds".to_string()));
	}
	return (true, None);
}

pub fn out_of_bounds(x: i32, y: i32) -> bool {
	return x < 0 || x >= MAP_SIZE || y < 0 || y >= MAP_SIZE
}

pub fn tile_occupied(walls: &Vec<Wall>, player_one: &Player, player_two: &Player, x: i32, y: i32) -> bool {
	// Check if wall exists on tile
	for wall in walls {
		if wall.x1 == x && wall.y1 == y {
			return true;
		}
		else if wall.x2 == x && wall.y2 == y {
			return true;
		}
	}
	
	// Check if a player stands on the tile
	if player_one.x == x && player_one.y == y {
		return true;
	}
	else if player_two.x == x && player_two.y == y {
		return true;
	}

	return false;
}

#[cfg(test)]
mod tests {
	use crate::game::game::Wall;
	use crate::game::player::{Player, PlayerType};
	use crate::game::validation::tile_occupied;
	use crate::game::validation::out_of_bounds;

	#[test]
	fn test_out_of_bounds() {
		assert_eq!(true, out_of_bounds(-1, -1));
		assert_eq!(true, out_of_bounds(-1, 5));
		assert_eq!(true, out_of_bounds(0, 9));
		assert_eq!(false, out_of_bounds(0, 8));
	}

	#[test]
	fn test_tile_occupied() {
		let temp_player = Player::new(0, 0, 0, PlayerType::Regular);
		let walls = vec![Wall { x1:0, y1:2, x2:0, y2: 1 }];
		assert_eq!(true, tile_occupied(&walls, &temp_player, &temp_player, 0, 0));
		assert_eq!(true, tile_occupied(&walls, &temp_player, &temp_player, 0, 1));
		assert_eq!(false, tile_occupied(&walls, &temp_player, &temp_player, 1, 0));
	}
}
