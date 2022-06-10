use super::validation::tile_occupied;
use super::game::{Move, Wall};
use super::player::Player;

pub fn execute_move(walls: &mut Vec<Wall>, active_player: &mut Player, player_move: &Move) {
	match &*player_move {
		Move::Wall(wall) => {
			if active_player.wall_count <= 0 {
				return;
			}
			
			active_player.decrement_wall_count();
			walls.push(wall.clone());
		},
		other => {
			let (new_x, new_y) = active_player.move_player(other);
			active_player.set_new_coordinates(new_x, new_y);
		}
	}
}