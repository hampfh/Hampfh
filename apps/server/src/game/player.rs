use super::game_state::{ErrorType, Move};

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

    pub fn decrement_wall_count(&mut self) -> Result<(), ErrorType> {
        if self.wall_count <= 0 {
            return Err(ErrorType::GameError {
                reason: format!(
                    "No more walls to place, all walls already used, active player: {:?}",
                    self.player_type.clone()
                ),
                fault: Some(self.player_type.clone()),
            });
        }
        self.wall_count -= 1;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_player_move() {
        let player = Player::new(0, 0, 0, PlayerType::Regular);
        assert_eq!(player.move_player(&Move::Up), (0, -1));
        assert_eq!(player.move_player(&Move::Down), (0, 1));
        assert_eq!(player.move_player(&Move::Left), (-1, 0));
        assert_eq!(player.move_player(&Move::Right), (1, 0));
    }

    #[test]
    fn test_decrement_wall_count() {
        let mut player = Player::new(0, 0, 1, PlayerType::Regular);
        assert_eq!(player.decrement_wall_count(), Ok(()));
        assert_eq!(0, player.wall_count);

        assert!(player.decrement_wall_count().is_err());
        assert_eq!(0, player.wall_count);
    }
}
