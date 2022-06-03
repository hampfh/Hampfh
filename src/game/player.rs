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

	pub fn set_new_coordinates(&mut self, x: i32, y: i32) {
		self.x = x;
		self.y = y;
	}

	pub fn move_player(&self, player_move: &Move) -> (i32, i32) {
		return match player_move {
			Move::Up => (self.x, self.y - 1),
			Move::Down => (self.x, self.y + 1),
			Move::Left => (self.x - 1, self.y),
			Move::Right => (self.x + 1, self.y),
			_ => (self.x, self.y)
		};
	}

	pub fn decrement_wall_count(&mut self) {
		self.wall_count -= 1;
	}
}