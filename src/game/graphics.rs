use super::game::{MAP_SIZE, Game};

#[derive(std::fmt::Debug)]
pub enum Tile {
	Empty = 0,
	P1 = 1,
	P2 = 2,
	Wall = 3
}

pub fn draw_game(game: &Game) {
	let buffer = populate_buffer(game);

	let mut count = 0;

	println!("Last move: {:?}", game.last_move);
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
		if count >= MAP_SIZE {
			println!("#");
			count = 0;
			print!("#");
		}
	}
	for _ in 0..MAP_SIZE + 1 {
		print!("#");
	}
	println!();
}

fn populate_buffer(game: &Game) -> Vec<Tile> {
	let mut draw_buffer = create_empty_buffer();
	
	for wall in &game.walls {
		place_tile(&mut draw_buffer, wall.x1, wall.x1, Tile::Wall);
		place_tile(&mut draw_buffer, wall.x2, wall.x2, Tile::Wall);
	}

	place_tile(&mut draw_buffer, game.player_one.x, game.player_one.y, Tile::P1);
	place_tile(&mut draw_buffer, game.player_two.x, game.player_two.y, Tile::P2);

	return draw_buffer;
}

fn place_tile(buffer: &mut Vec<Tile>, x: i32, y: i32, tile: Tile) {
	buffer[(y * MAP_SIZE + x) as usize] = tile;
}

fn create_empty_buffer() -> Vec<Tile> {
	let mut buffer: Vec<Tile> = Vec::new();
	for _ in 0..(MAP_SIZE * MAP_SIZE) {
		buffer.push(Tile::Empty)
	}
	return buffer;
}