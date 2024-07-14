use core::fmt;
use std::{
    sync::{Arc, Mutex},
    time::Duration,
};

use mlua::Lua;

use super::{
    board::Tile,
    player::{Player, PlayerType},
};

pub const MAP_SIZE: i32 = 9;
pub const INITIAL_WALL_COUNT: i32 = 10;
pub const MAX_TURNS: i32 = 400;

#[derive(Debug, Clone, Eq, PartialEq)]
pub(crate) struct Wall {
    pub x1: i32,
    pub y1: i32,
    pub x2: i32,
    pub y2: i32,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub(crate) enum GameResult {
    Error(ErrorType),
    PlayerOneWon,
    PlayerTwoWon,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub(crate) enum ErrorType {
    /// The script did not obey the rules of the game in some way
    GameError {
        reason: String,
        fault: Option<PlayerType>,
    },
    /// The script did not run properly
    RuntimeError {
        reason: String,
        fault: Option<PlayerType>,
    },
    /// The script takes to much time during a round
    TurnTimeout { fault: Option<PlayerType> },
    /// The game doesn't progress anymore
    GameDeadlock,
}

#[derive(Debug)]
pub(crate) struct Game {
    pub(crate) config: GameConfig,
    pub(crate) logger: Vec<Move>,

    pub(crate) running: bool,
    pub(crate) game_result: Option<GameResult>,
    pub(crate) player_one: Player,
    pub(crate) player_two: Player,

    pub(crate) walls: Vec<Wall>,

    pub(crate) player_one_sandbox: Arc<Mutex<mlua::Lua>>,
    pub(crate) player_two_sandbox: Arc<Mutex<mlua::Lua>>,
    pub(crate) player_one_turn: bool,
    pub(crate) last_move: Option<Move>,
    pub(crate) std: String, // Standard library
    pub(crate) turns: Vec<Vec<Tile>>,
}

impl Game {
    pub(crate) fn new(config: GameConfig) -> Game {
        let p1 = Player::new(
            MAP_SIZE / 2,
            MAP_SIZE - 1,
            INITIAL_WALL_COUNT,
            PlayerType::Flipped,
        );
        let p2 = Player::new(MAP_SIZE / 2, 0, INITIAL_WALL_COUNT, PlayerType::Regular);
        let walls = Vec::new();
        let std =
            std::fs::read_to_string("./scripts/std.lua").expect("Could not load standard library");
        return Game::custom_new(p1, p2, walls, std, config);
    }

    pub(crate) fn custom_new(
        player_one: Player,
        player_two: Player,
        walls: Vec<Wall>,
        std: String,
        config: GameConfig,
    ) -> Game {
        let p1_lua = Lua::new();
        let p2_lua = Lua::new();

        p1_lua.sandbox(true).unwrap();
        p2_lua.sandbox(true).unwrap();

        return Game {
            config,
            logger: Vec::new(),
            running: true,
            game_result: None,
            player_one,
            player_two,
            walls,
            player_one_sandbox: Arc::new(Mutex::new(p1_lua)),
            player_two_sandbox: Arc::new(Mutex::new(p2_lua)),
            player_one_turn: true,
            last_move: None,
            std,
            turns: Vec::new(),
        };
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub(crate) struct GameConfig {
    pub(crate) live_print_match: bool,
    pub(crate) bot_initialization_timeout: Duration,
    pub(crate) bot_turn_timeout: Duration,
}
impl GameConfig {
    pub(crate) fn new() -> GameConfig {
        return GameConfig {
            live_print_match: false,
            bot_initialization_timeout: Duration::from_millis(250),
            bot_turn_timeout: Duration::from_millis(250),
        };
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub(crate) enum Move {
    Up,
    Down,
    Left,
    Right,
    Wall(Wall),
    Invalid { reason: String },
}

impl fmt::Display for Move {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Move::Up => write!(f, "Up"),
            Move::Down => write!(f, "Down"),
            Move::Left => write!(f, "Left"),
            Move::Right => write!(f, "Right"),
            Move::Wall(wall) => write!(
                f,
                "Wall({}, {}, {}, {})",
                wall.x1, wall.y1, wall.x2, wall.y2
            ),
            Move::Invalid { reason } => write!(f, "Invalid({})", reason),
        }
    }
}

#[cfg(test)]
mod tests {

    use crate::game::game_state::Move;

    #[test]
    fn tests_move_printout() {
        assert_eq!(format!("{}", Move::Up), "Up");
        assert_eq!(format!("{}", Move::Down), "Down");
        assert_eq!(format!("{}", Move::Left), "Left");
        assert_eq!(format!("{}", Move::Right), "Right");
        assert_eq!(
            format!(
                "{}",
                Move::Wall(super::Wall {
                    x1: 1,
                    y1: 2,
                    x2: 3,
                    y2: 4
                })
            ),
            "Wall(1, 2, 3, 4)"
        );

        let random_string = "blajgkldsjglkjskgjdlsjgkljdslajgldsjalgdsjlakjglkad";
        assert_eq!(
            format!(
                "{}",
                Move::Invalid {
                    reason: random_string.to_string()
                }
            ),
            format!("Invalid({})", random_string)
        );
    }
}
