extern crate hlua;
use hlua::Lua;

use super::turn;
use super::player::Player;
use super::graphics::draw_game;

pub const MAP_SIZE: i32 = 9;
pub const INITIAL_WALL_COUNT: i32 = 10;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Wall {
	pub x1: i32,
	pub y1: i32,
	pub x2: i32,
	pub y2: i32
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum GameState {
	Running,
	PlayerOneWon,
	PlayerTwoWon,
}

#[derive(Debug)]
pub struct Game {
	pub game_state: GameState,
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
			game_state: GameState::Running,
			player_one: Player::new(MAP_SIZE / 2, MAP_SIZE - 1, INITIAL_WALL_COUNT),
			player_two: Player::new(MAP_SIZE / 2, 0, INITIAL_WALL_COUNT),
			walls: Vec::new(),
			player_one_sandbox: Lua::new(),
			player_two_sandbox: Lua::new(),
			player_one_turn: true,
			last_move: None
		}
	}

	pub fn start(&mut self, program1: String, program2: String) -> GameState {
		// Run programs for the first time

		// TODO make sure programs to not run longer than 1 second
		self.player_one_sandbox.execute::<()>(&program1).unwrap();
		self.player_two_sandbox.execute::<()>(&program2).unwrap();

		self.game_loop();
		return self.game_state.clone()
	}

	pub fn game_loop(&mut self) {
		while self.game_state == GameState::Running {
			self.update();
			self.winner();
		}
	}

	pub fn update(&mut self) {
		let result = turn::on_turn(self);
		if result.is_err() {
			// TODO manage error
			println!("Error: {:?}", result.err().unwrap());
		}

		draw_game(self);
		std::thread::sleep(std::time::Duration::from_millis(1000));
	}

	pub fn winner(&mut self) {
		if self.player_one.y == 0 {
			self.game_state = GameState::PlayerOneWon;
		}
		else if self.player_two.y == MAP_SIZE - 1 {
			self.game_state = GameState::PlayerTwoWon;
		}
	}
}

// Converts a string like ["x1,y1,x2,y2" -> Wall]
pub fn deserialize_wall(input: &str) -> Move {

	let splits = input.split(",").map(|s| s.trim()).collect::<Vec<&str>>();
	if splits.len() != 4 as usize {
		return Move::Invalid { reason: format!("Invalid return format, expected 4 values, got: [{}]", input) };
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