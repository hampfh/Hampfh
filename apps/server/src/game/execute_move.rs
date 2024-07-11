use super::game_state::{ErrorType, Move, Wall};
use super::player::Player;

pub(super) fn execute_move(
    walls: &mut Vec<Wall>,
    active_player: &mut Player,
    player_move: &Move,
) -> Result<(), ErrorType> {
    match &*player_move {
        Move::Wall(wall) => {
            match active_player.decrement_wall_count() {
                Err(error) => return Err(error),
                _ => {}
            }
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

#[cfg(test)]

mod tests {
    use crate::game::{
        execute_move::execute_move,
        game_state::{Move, Wall},
        player::{Player, PlayerType},
    };

    #[test]
    fn executes_move_forwards() {
        let mut walls: Vec<Wall> = Vec::new();
        let mut player = Player::new(4, 4, 10, PlayerType::Flipped);
        let player_move = Move::Right;
        execute_move(&mut walls, &mut player, &player_move).unwrap();
        assert_eq!(player.x, 5);
        assert_eq!(player.y, 4);
    }
}
