use super::game::{Game, Move, Player};

pub fn execute_move(game: &mut Game, player_move: &Move) {

	match &*player_move {
		Move::Up => {
			if game.player_one.y > 0 {
				game.player_one.y -= 1;
			}
		},
		Move::Down => {
			if game.player_one.y < game.player_two.y {
				game.player_one.y += 1;
			}
		},
		Move::Left => {
			if game.player_one.x > 0 {
				game.player_one.x -= 1;
			}
		},
		Move::Right => {
			if game.player_one.x < game.player_two.x {
				game.player_one.x += 1;
			}
		},
		Move::Wall(wall) => {
			game.walls.push(wall.clone());
			game.player_one.wall_count -= 1;
		},
		Move::Invalid { reason } => {
			println!("{}", reason);
		}
	}
}

fn get_active_player(game: &Game) -> Player {
	if game.player_one_turn {
		return game.player_one.clone();
	}
	return game.player_two.clone();
}