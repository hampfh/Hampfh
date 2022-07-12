pub(crate) mod board;
pub(crate) mod entry_point;
pub(crate) mod execute_move;
pub(crate) mod game;
pub(crate) mod graphics;
pub(crate) mod map_mirroring;
pub(crate) mod methods;
pub(crate) mod path_find;
pub(crate) mod player;
pub(crate) mod sandbox;
pub(crate) mod turn;
pub(crate) mod validation;

mod tests {
    mod game_tests;
    mod security_tests;
    mod util;
}
