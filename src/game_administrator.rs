extern crate hlua;
use hlua::Lua;

#[derive(Debug, Clone)]
struct Player {
	x: i32,
	y: i32,
	wall_count: i32
}

#[derive(Debug, Clone)]
struct Wall {
	x1: i32,
	y1: i32,
	x2: i32,
	y2: i32
}

#[derive(Debug)]
pub struct Game {
	player_one: Player,
	player_two: Player,

	walls: Vec<Wall>,

	player_one_sandbox: hlua::Lua<'static>,
	player_two_sandbox: hlua::Lua<'static>,
	player_one_turn: bool,
	last_move: Option<Move>
}

#[derive(Debug, Clone)]
enum Move {
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
				x: 0,
				y: 0,
				wall_count: 0
			},
			player_two: Player {
				x: 0,
				y: 0,
				wall_count: 0
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
		let (x, y) = self.get_enemy_coords();

		let last_move: String = match &self.last_move.clone() {
			Some(Move::Up) => "0".to_string(),
			Some(Move::Left) => "1".to_string(),
			Some(Move::Down) => "2".to_string(),
			Some(Move::Right) => "3".to_string(),
			Some(Move::Wall(wall)) => serialize_wall(wall),
			Some(Move::Invalid { reason: _ }) => "nil".to_string(),
			None => "nil".to_string() 
		};

		let script_runner = format!("ExternalglobalVarResult = onTurn({}, {}, {}, {})", last_move, x, y, self.serialize_walls());

		let active_sandbox = if self.player_one_turn {
			&mut self.player_one_sandbox
		} else {
			&mut self.player_two_sandbox
		};

		active_sandbox.execute::<()>(&script_runner).unwrap();

		// TODO here we should add a timeout for the script to run
		let raw_player_move: Option<String> = active_sandbox.get("ExternalglobalVarResult");

		let player_move = match raw_player_move {
			Some(value) => {
				match value.as_str() {
					"0" => Some(Move::Up),
					"1" => Some(Move::Left),
					"2" => Some(Move::Down),
					"3" => Some(Move::Right),
					wall => {
						match deserialize_wall(&wall) {
							Move::Wall(wall) => Some(Move::Wall(wall)),
							Move::Invalid { reason } => Some(Move::Invalid { reason }),
							_ => panic!("Invalid wall")
						}
					}
				}
				
			},
			None => None
		};

		self.player_one_turn = !self.player_one_turn;
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

fn serialize_wall(wall: &Wall) -> String {
	return format!("{{x1={}, y1={}, x2={}, y2={}}},", wall.x1, wall.y1, wall.x2, wall.y2);
}

// Converts a string like ["x1,y1,x2,y2" -> Wall]
fn deserialize_wall(wall: &str) -> Move {

	let splits = wall.split(",").map(|s| s.trim()).collect::<Vec<&str>>();
	if splits.len() != 4 as usize {
		return Move::Invalid { reason: "Wall must contain exactly 4 values".to_string() };
	}
	let result = splits.iter().map(|x| x.trim()).map(|x| x.parse::<i32>().unwrap_or_else(|e| -1)).collect::<Vec<i32>>();

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