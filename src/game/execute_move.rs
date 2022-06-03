use super::game::{Game, Move};
use super::player::Player;

pub fn execute_move(game: &mut Game, player_move: &Move) {

	let active_player = get_active_player(game);

	match &*player_move {
		Move::Wall(wall) => {
			game.walls.push(wall.clone());
			game.player_one.decrement_wall_count()
		},
		other => active_player.move_player(other)
	}
	println!("{:?} {:?}", game.player_one, game.player_two);
}

fn get_active_player(game: &mut Game) -> &mut Player {
	if game.player_one_turn {
		return &mut game.player_one;
	}
	return &mut game.player_two;
}