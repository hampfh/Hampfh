use crate::game::map_mirroring::conditionally_reverse_move;
use crate::game::map_mirroring::conditionally_reverse_walls;
use crate::game::path_find::path_exists_for_players;
use super::player::Player;
use super::execute_move::execute_move;
use super::game::get_active_player;
use super::game::{MAP_SIZE, Game, Move, Wall};

pub fn valid_move(game: &mut Game, player_move: Move) -> Result<(), String> {
	
	let mut walls = game.walls.clone();
	let (active_player, other) = get_active_player(game);
	let mut temp_active_player = active_player.clone();

	// Execute a fake move to check if the move is valid
	execute_move(&mut walls, &mut temp_active_player, other, &player_move);

	let result = valid_tile(&walls, other, other, temp_active_player.x, temp_active_player.y);
	if result.is_err() {
		return Err(format!("Invalid move: {}", result.err().unwrap()));
	}

	match path_exists_for_players(game) {
		Ok(_) => Ok(()),
		Err(reason) => Err(reason)
	}
}

pub fn valid_tile(walls: &Vec<Wall>, player_one: &Player, player_two: &Player, x: i32, y: i32) -> Result<(), String> {
	if tile_occupied(walls, player_one, player_two, x, y) {
		return Err(format!("Tile ({},{}) is occupied", x, y));
	} 
	if out_of_bounds(x, y) {
		return Err(format!("Tile is out of bounds ({}, {})", x, y));
	}
	Ok(())
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
