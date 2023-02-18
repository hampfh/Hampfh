use super::game::Move;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PlayerType {
    Regular, // Player 2
    Flipped, // Player 1
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Player {
    pub player_type: PlayerType,
    pub x: i32,
    pub y: i32,
    pub wall_count: i32,
}

impl Player {
    pub fn new(x: i32, y: i32, wall_count: i32, player_type: PlayerType) -> Player {
        Player {
            player_type: player_type,
            x: x,
            y: y,
            wall_count: wall_count,
        }
    }

    pub fn set_new_coordinates(&mut self, x: i32, y: i32) {
        self.x = x;
        self.y = y;
    }

    pub(crate) fn move_player(&self, player_move: &Move) -> (i32, i32) {
        return match player_move {
            Move::Up => (self.x, self.y - 1),
            Move::Down => (self.x, self.y + 1),
            Move::Left => (self.x - 1, self.y),
            Move::Right => (self.x + 1, self.y),
            _ => (self.x, self.y),
        };
    }

    pub fn decrement_wall_count(&mut self) {
        self.wall_count -= 1;
    }
}
