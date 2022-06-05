use super::player::Player;
use super::game::{MAP_SIZE, Game, Move, Wall};

pub fn valid_move(game: &mut Game, player_move: Move) -> Result<(), String> {

	// Reverse board for player two
	let reverse_board = !game.player_one_turn;

	match player_move {
		Move::Up => {
			if game.player_one.y > 0 {
				
			}
		},
		Move::Down => {
			if game.player_one.y < game.player_two.y {
				
			}
		},
		Move::Left => {
			if game.player_one.x > 0 {
				
			}
		},
		Move::Right => {
			if game.player_one.x < game.player_two.x {
				
			}
		},
		Move::Wall(wall) => {
			//game.walls.push(wall.clone());
			
		},
		Move::Invalid { reason } => {
			return Err(reason.to_string());
		}
	}

	Ok(())
}

pub fn valid_tile(walls: &Vec<Wall>, player_one: &Player, player_two: &Player, x: i32, y: i32) -> (bool, Option<String>) {
	if tile_occupied(walls, player_one, player_two, x, y) {
		return (false, Some("Tile is occupied".to_string()));
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