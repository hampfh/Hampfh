use super::execute_move::execute_move;
use super::game::get_active_player_type;
use super::game_state::{ErrorType, Move, Wall, MAP_SIZE};
use super::path_find::Pos;
use super::player::{Player, PlayerType};
use crate::game::path_find::path_exists_for_players;

/// Return true: success, no action required
/// Return false: success, perform jump
pub(super) fn valid_move(
    player_one_turn: bool,
    active_player: &Player,
    other: &Player,
    walls: &Vec<Wall>,
    player_move: Move,
) -> Result<bool, ErrorType> {
    let mut walls = walls.clone();
    let clousure_walls = walls.clone();
    let mut temp_active_player = active_player.clone();

    let tile_is_valid = |pos: Pos, ignore_players: bool| -> Result<(), String> {
        valid_tile(
            &clousure_walls,
            // We never want to check ourselves since that position will be taken
            None,
            Some(other),
            pos.0,
            pos.1,
            ignore_players,
        )
    };

    // If move is wall, make sure it is valid
    if let Move::Wall(wall) = player_move.clone() {
        if !valid_wall_format(&wall) {
            return Err(ErrorType::GameError {
                reason: format!(
                    "Invalid wall format, a wall must consist of two adjacent coordinates: (({},{}), ({},{}))",
                    wall.x1, wall.y1, wall.x2, wall.y2
                ),
                fault: Some(get_active_player_type(player_one_turn))
            });
        }
        // Check that wall is not out of bounds
        // or tries to populate another tile
        if tile_is_valid(Pos(wall.x1, wall.y1), false).is_err()
            || tile_is_valid(Pos(wall.x2, wall.y2), false).is_err()
        {
            return Err(ErrorType::GameError {
                reason: format!(
                    "Invalid wall placement at (({},{}),({},{})), coordinates are either occupied or out of bounds",
                    wall.x1, wall.y1, wall.x2, wall.y2
                ),
                fault: Some(get_active_player_type(player_one_turn))
            });
        }
    }

    // Execute a fake move to check if the move is valid
    if let Err(error) = execute_move(&mut walls, &mut temp_active_player, &player_move) {
        return Err(error);
    }

    let result = tile_is_valid(Pos(temp_active_player.x, temp_active_player.y), true);
    if result.is_err() {
        return Err(ErrorType::GameError {
            reason: format!("Invalid move: {}", result.err().unwrap()),
            fault: Some(get_active_player_type(player_one_turn)),
        });
    }

    let on_top_of_opponent = temp_active_player.x == other.x && temp_active_player.y == other.y;
    if on_top_of_opponent {
        if tile_is_valid(Pos(other.x, other.y - 1), false).is_err()
            && tile_is_valid(Pos(other.x + 1, other.y), false).is_err()
            && tile_is_valid(Pos(other.x, other.y + 1), false).is_err()
            && tile_is_valid(Pos(other.x - 1, other.y), false).is_err()
        {
            return Err(ErrorType::GameError {
                reason: format!("Invalid move, cannot jump, no free position around opponent"),
                fault: Some(get_active_player_type(player_one_turn)),
            });
        }
    }

    let (p1, p2) = if active_player.player_type == PlayerType::Flipped {
        (active_player, other)
    } else {
        (other, active_player)
    };

    match path_exists_for_players(&walls, &p1, &p2) {
        Ok(_) => Ok(!on_top_of_opponent),
        Err(error) => {
            return Err(ErrorType::GameError {
                reason: error.to_string(),
                fault: Some(get_active_player_type(player_one_turn)),
            })
        }
    }
}

pub(super) fn valid_tile(
    walls: &Vec<Wall>,
    player_one: Option<&Player>,
    player_two: Option<&Player>,
    x: i32,
    y: i32,
    ignore_players: bool,
) -> Result<(), String> {
    if tile_occupied(walls, player_one, player_two, x, y, ignore_players) {
        return Err(format!("Tile ({},{}) is occupied", x, y));
    }
    if out_of_bounds(x, y) {
        return Err(format!("Tile is out of bounds ({}, {})", x, y));
    }
    Ok(())
}

pub(super) fn out_of_bounds(x: i32, y: i32) -> bool {
    return x < 0 || x >= MAP_SIZE || y < 0 || y >= MAP_SIZE;
}

pub(super) fn tile_occupied(
    walls: &Vec<Wall>,
    player_one: Option<&Player>,
    player_two: Option<&Player>,
    x: i32,
    y: i32,
    ignore_players: bool,
) -> bool {
    // Check if wall exists on tile
    for wall in walls {
        if wall.x1 == x && wall.y1 == y {
            return true;
        } else if wall.x2 == x && wall.y2 == y {
            return true;
        }
    }

    if ignore_players {
        return false;
    }

    // Check if a player stands on the tile
    if player_one.is_some() && player_one.unwrap().x == x && player_one.unwrap().y == y {
        return true;
    } else if player_two.is_some() && player_two.unwrap().x == x && player_two.unwrap().y == y {
        return true;
    }

    return false;
}

#[cfg(test)]
mod tests {
    use crate::game::game_state::Wall;
    use crate::game::player::{Player, PlayerType};
    use crate::game::validation::out_of_bounds;
    use crate::game::validation::tile_occupied;

    #[test]
    fn test_out_of_bounds() {
        assert_eq!(true, out_of_bounds(-1, -1));
        assert_eq!(true, out_of_bounds(-1, 5));
        assert_eq!(true, out_of_bounds(0, 9));
        assert_eq!(false, out_of_bounds(0, 8));
    }

    #[test]
    fn test_tile_occupied() {
        let temp_player = Player::new(0, 0, 0, PlayerType::Regular);
        let walls = vec![Wall {
            x1: 0,
            y1: 2,
            x2: 0,
            y2: 1,
        }];
        assert_eq!(
            true,
            tile_occupied(&walls, Some(&temp_player), Some(&temp_player), 0, 0, false)
        );
        assert_eq!(
            true,
            tile_occupied(&walls, Some(&temp_player), Some(&temp_player), 0, 1, false)
        );
        assert_eq!(
            false,
            tile_occupied(&walls, Some(&temp_player), Some(&temp_player), 1, 0, false)
        );
    }
}

fn valid_wall_format(wall: &Wall) -> bool {
    return (wall.x1 - wall.x2).abs() + (wall.y1 - wall.y2).abs() == 1;
}
