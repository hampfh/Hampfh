use super::game::{ErrorType, Move, Wall};
use super::player::Player;

pub fn execute_move(
    walls: &mut Vec<Wall>,
    active_player: &mut Player,
    player_move: &Move,
) -> Result<(), ErrorType> {
    match &*player_move {
        Move::Wall(wall) => {
            if active_player.wall_count <= 0 {
                return Err(ErrorType::GameError {
                    reason: format!("No more walls to place, all walls already used"),
                });
            }

            active_player.decrement_wall_count();
            walls.push(wall.clone());
        }
        other => {
            let (new_x, new_y) = active_player.move_player(other);
            active_player.set_new_coordinates(new_x, new_y);
        }
    }
    Ok(())
}
