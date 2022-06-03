extern crate hlua;
use hlua::Lua;
use crate::game::turn;

pub const MAP_SIZE: i32 = 9;
pub const INITIAL_WALL_COUNT: i32 = 10;

#[derive(Debug, Clone)]
pub struct Player {
	pub x: i32,
	pub y: i32,
	pub wall_count: i32
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Wall {
	pub x1: i32,
	pub y1: i32,
	pub x2: i32,
	pub y2: i32
}

#[derive(Debug)]
pub struct Game {
	pub player_one: Player,
	pub player_two: Player,
 
	pub walls: Vec<Wall>,
 
	pub player_one_sandbox: hlua::Lua<'static>,
	pub player_two_sandbox: hlua::Lua<'static>,
	pub player_one_turn: bool,
	pub last_move: Option<Move>
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Move {
	Up,
	Down,
	Left,
	Right,
	Wall(Wall),
	Invalid { reason: String }
}

impl Game {
	pub fn new() -> Game {
		Game {
			player_one: Player {
				x: MAP_SIZE / 2 + 1,
				y: MAP_SIZE - 1,
				wall_count: INITIAL_WALL_COUNT
			},
			player_two: Player {
				x: MAP_SIZE / 2,
				y: 0,
				wall_count: INITIAL_WALL_COUNT
			},
			walls: Vec::new(),
			player_one_sandbox: Lua::new(),
			player_two_sandbox: Lua::new(),
			player_one_turn: true,
			last_move: None
		}
	}

	pub fn start(&mut self, program1: String, program2: String) {
		// Run programs for the first time

		// TODO make sure programs to not run longer than 1 second
		self.player_one_sandbox.execute::<()>(&program1).unwrap();
		self.player_two_sandbox.execute::<()>(&program2).unwrap();

		self.game_loop();
	}

	pub fn game_loop(&mut self) {
		loop {
			self.update();
			self.winner();
		}
	}

	pub fn update(&mut self) {
		let result = turn::on_turn(self);
		if result.is_err() {
			// TODO manage error
		}
	}

	pub fn get_enemy_coords(&self) -> (i32, i32) {
		if self.player_one_turn {
			return (self.player_two.x, self.player_two.y);
		}
		return (self.player_one.x, self.player_one.y);
	}

	pub fn winner(&self) {
		// TODO check if player one or two has won
	}

	pub fn serialize_walls(&self) -> String {
		return format!("{}{}{}", "{", self.walls.iter().map(|wall| serialize_wall(wall)).collect::<Vec<String>>().join("\n"), "}")
	}
}

pub fn serialize_wall(wall: &Wall) -> String {
	return format!("{{x1={}, y1={}, x2={}, y2={}}},", wall.x1, wall.y1, wall.x2, wall.y2);
}

// Converts a string like ["x1,y1,x2,y2" -> Wall]
pub fn deserialize_wall(wall: &str) -> Move {

	let splits = wall.split(",").map(|s| s.trim()).collect::<Vec<&str>>();
	if splits.len() != 4 as usize {
		return Move::Invalid { reason: "Wall must contain exactly 4 values".to_string() };
	}
	let result = splits.iter().map(|x| x.trim()).map(|x| x.parse::<i32>().unwrap_or_else(|_| -1)).collect::<Vec<i32>>();

	// If any of the values are invalid (negative), the move is invalid
	if result.iter().any(|x| *x < 0) {
		return Move::Invalid { reason: "Invalid wall param".to_string() };
	}

	return Move::Wall(Wall {
		x1: result[0],
		y1: result[1],
		x2: result[2],
		y2: result[3]
	});
}