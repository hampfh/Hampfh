use std::sync::{Arc, Mutex};

use super::player::Player;

pub const MAP_SIZE: i32 = 9;
pub const INITIAL_WALL_COUNT: i32 = 10;
pub const MAX_TURNS: i32 = 2000;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Wall {
    pub x1: i32,
    pub y1: i32,
    pub x2: i32,
    pub y2: i32,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum GameState {
    Running,
    PlayerOneWon,
    PlayerTwoWon,
    Error(ErrorType),
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum ErrorType {
    /// The script did not obey the rules of the game in some way
    GameError {
        reason: String,
    },
    /// The script did not run properly
    RuntimeError {
        reason: String,
    },
    /// The script takes to much time during a round
    TurnTimeout,
    GameDeadlock,
}

#[derive(Debug, Clone)]
pub struct Game {
    pub game_state: GameState,
    pub player_one: Player,
    pub player_two: Player,

    pub walls: Vec<Wall>,

    pub player_one_sandbox: Arc<Mutex<hlua::Lua<'static>>>,
    pub player_two_sandbox: Arc<Mutex<hlua::Lua<'static>>>,
    pub player_one_turn: bool,
    pub last_move: Option<Move>,
    pub std: String, // Standard library
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Move {
    Up,
    Down,
    Left,
    Right,
    Wall(Wall),
    Invalid { reason: String },
}
