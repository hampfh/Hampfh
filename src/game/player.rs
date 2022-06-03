use super::game::Move;

#[derive(Debug, Clone)]
pub struct Player {
	pub x: i32,
	pub y: i32,
	pub wall_count: i32
}

impl Player {
	pub fn new(x: i32, y: i32, wall_count: i32) -> Player {
		Player {
			x: x,
			y: y,
			wall_count: wall_count
		}
	}

	pub fn move_player(&mut self, player_move: &Move) {
		match player_move {
			Move::Up => self.y -= 1,
			Move::Down => self.y += 1,
			Move::Left => self.x -= 1,
			Move::Right => self.x += 1,
			_ => ()
		}
	}

	pub fn decrement_wall_count(&mut self) {
		self.wall_count -= 1;
	}
}