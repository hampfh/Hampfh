use super::game::{Game, Move};

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

pub fn tile_occupied(game: &Game, x: i32, y: i32) -> bool {
	// Check if wall exists on tile
	for wall in &game.walls {
		if wall.x1 == x && wall.y1 == y {
			return true;
		}
		else if wall.x2 == x && wall.y2 == y {
			return true;
		}
	}
	
	// Check if a player stands on the tile
	if game.player_one.x == x && game.player_one.y == y {
		return true;
	}
	else if game.player_two.x == x && game.player_two.y == y {
		return true;
	}

	return false;
}