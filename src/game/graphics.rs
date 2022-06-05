use super::board::{populate_board, Tile};
use super::game::{MAP_SIZE, Game};

pub fn draw_game(game: &Game) {
	let buffer = populate_board(game, &game.walls);

	let mut count = 0;

	println!("Last move: {:?}", game.last_move);

	// Upper wall
	for _ in 0..MAP_SIZE + 2 {
		print!("#");
	}
	println!();
	print!("#");
	for tile in buffer.iter() {
		match tile {
			Tile::Empty => print!(" "),
			Tile::P1 => print!("O"),
			Tile::P2 => print!("X"),
			Tile::Wall => print!("#")
		}

		count += 1;
		// Side wall
		if count >= MAP_SIZE {
			println!("#");
			count = 0;
			print!("#");
		}
	}

	// Lower wall
	for _ in 0..MAP_SIZE + 1 {
		print!("#");
	}
	println!();
}