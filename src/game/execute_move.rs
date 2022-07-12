use super::game::{ErrorType, Move, Wall};
use super::player::Player;

pub(super) fn execute_move(
    walls: &mut Vec<Wall>,
    active_player: &mut Player,
    player_move: &Move,
) -> Result<(), ErrorType> {
    match &*player_move {
        Move::Wall(wall) => {
            if active_player.wall_count <= 0 {
                return Err(ErrorType::GameError {
                    reason: format!(
                        "No more walls to place, all walls already used, active player: {:?}",
                        active_player.player_type.clone()
                    ),
                    fault: Some(active_player.player_type.clone()),
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

/// When jumping over and opponent we use
/// the opponent's coordinates and run the move
/// from there, then apply the result to the new
/// player
pub(super) fn execute_move_jump(
    active_player: &mut Player,
    other_player: &Player,
    player_move: &Move,
) -> Result<(), ErrorType> {
    let (new_x, new_y) = other_player.move_player(player_move);
    if new_x == active_player.x && new_y == active_player.y {
        return Err(ErrorType::GameError {
            reason: format!("Invalid move, cannot jump back to original position"),
            fault: Some(active_player.player_type.clone()),
        });
    }
    active_player.set_new_coordinates(new_x, new_y);
    Ok(())
}
