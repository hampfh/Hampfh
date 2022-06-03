use super::game::{Game, Move};
use super::player::Player;

pub fn execute_move(game: &mut Game, player_move: &Move) {

	let mut active_player = get_active_player(game);

	match &*player_move {
		Move::Wall(wall) => {
			game.walls.push(wall.clone());
			game.player_one.decrement_wall_count()
		},
		other => active_player.move_player(other)
	}
}

fn get_active_player(game: &Game) -> Player {
	if game.player_one_turn {
		return game.player_one.clone();
	}
	return game.player_two.clone();
}