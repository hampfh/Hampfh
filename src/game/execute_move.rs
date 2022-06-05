use super::validation::tile_occupied;
use super::game::{Game, Move};
use super::player::Player;

pub fn execute_move(game: &mut Game, player_move: &Move) {
	match &*player_move {
		Move::Wall(wall) => {
			let player = get_active_player(game);
			if player.0.wall_count <= 0 {
				return;
			}
			
			player.0.decrement_wall_count();
			game.walls.push(wall.clone());
		},
		other => {
			
			let (active_player, _) = get_active_player(game);
			let (new_x, new_y) = active_player.move_player(other);
			
			// If the tile is occupied we do not allow the player to move
			if tile_occupied(game, new_x, new_y) {
				return;
			}

			let (active_player, _) = get_active_player(game);
			active_player.set_new_coordinates(new_x, new_y);
		}
	}

	println!("{:?} {:?}", game.player_one, game.player_two);
}

/**
 * Returns a tuple, the first player is always the active one
 * the second is the non-active player
 */
fn get_active_player(game: &mut Game) -> (&mut Player, &Player) {
	if game.player_one_turn {
		return (&mut game.player_one, &game.player_two);
	}
	return (&mut game.player_two, &game.player_one);
}