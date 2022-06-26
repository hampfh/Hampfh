pub mod board;
pub mod entry_point;
pub mod execute_move;
pub mod game;
pub mod graphics;
pub mod map_mirroring;
pub mod methods;
pub mod path_find;
pub mod player;
pub mod turn;
pub mod validation;

mod tests {
    pub mod game_tests;
    pub mod security_tests;
}
