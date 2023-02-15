use std::sync::{Arc, Mutex};

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
    pub(crate) running: bool,
    pub(crate) game_result: Option<GameResult>,
    pub(crate) player_one: Player,
    pub(crate) player_two: Player,

    pub(crate) walls: Vec<Wall>,

    pub(crate) player_one_sandbox: Arc<Mutex<rlua::Lua>>,
    pub(crate) player_two_sandbox: Arc<Mutex<rlua::Lua>>,
    pub(crate) player_one_turn: bool,
    pub(crate) last_move: Option<Move>,
    pub(crate) std: String, // Standard library
    pub(crate) turns: Vec<Vec<Tile>>,
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
