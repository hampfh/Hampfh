use crate::game::game::{Move, Wall, MAP_SIZE};
use crate::game::player::Player;

/**
 * This file includes all logic for the map mirroring process.
 * Aka: All of the scripts will always think they are playing from the
 * same side thus we need to perform some logic to reverse the map for
 * the second player.
 */

pub(crate) fn reverse_move(player_move: Move) -> Move {
    return match player_move {
        Move::Up => Move::Down,
        Move::Right => Move::Left,
        Move::Down => Move::Up,
        Move::Left => Move::Right,
        Move::Wall(wall) => Move::Wall(reverse_wall(&wall)),
        Move::Invalid { reason } => Move::Invalid { reason },
    };
}

pub(crate) fn reverse_wall(wall: &Wall) -> Wall {
    return Wall {
        x1: reverse_coordinate(wall.x1),
        y1: reverse_coordinate(wall.y1),
        x2: reverse_coordinate(wall.x2),
        y2: reverse_coordinate(wall.y2),
    };
}

pub(crate) fn conditionally_reverse_walls(walls: &Vec<Wall>, condition: bool) -> Vec<Wall> {
    if !condition {
        return walls.to_vec();
    }
    return walls.into_iter().map(|wall| reverse_wall(wall)).collect();
}

pub fn reverse_coordinate(coordinate: i32) -> i32 {
    return (MAP_SIZE - 1) - coordinate;
}

#[allow(dead_code)]
pub(crate) fn conditionally_reverse_move(player_move: Move, condition: bool) -> Move {
    if !condition {
        return player_move;
    } else {
        return reverse_move(player_move);
    }
}

#[allow(dead_code)]
pub fn conditionally_reverse_coordinates(coordinates: (i32, i32), condition: bool) -> (i32, i32) {
    if !condition {
        return (coordinates.0, coordinates.1);
    } else {
        return (
            reverse_coordinate(coordinates.0),
            reverse_coordinate(coordinates.1),
        );
    }
}

pub fn conditionally_reverse_player(player: &Player, condition: bool) -> Player {
    if !condition {
        return player.clone();
    }
    let mut new_player = player.clone();
    new_player.x = reverse_coordinate(player.x);
    new_player.y = reverse_coordinate(player.y);
    return new_player;
}

#[cfg(test)]
mod tests {
    use crate::game::{
        game::{Move, Wall, MAP_SIZE},
        player::{Player, PlayerType},
    };

    use super::{conditionally_reverse_player, reverse_coordinate, reverse_move, reverse_wall};

    #[test]
    fn test_reverse_move() {
        assert_eq!(Move::Up, reverse_move(Move::Down));
        assert_eq!(Move::Right, reverse_move(Move::Left));
        assert_eq!(Move::Down, reverse_move(Move::Up));
        assert_eq!(Move::Left, reverse_move(Move::Right));
    }

    fn reverse_wall_compare_utility(input: Vec<i32>, expected: Vec<i32>) {
        let wall = Wall {
            x1: input[0],
            y1: input[1],
            x2: input[2],
            y2: input[3],
        };
        let reversed_wall = Wall {
            x1: expected[0],
            y1: expected[1],
            x2: expected[2],
            y2: expected[3],
        };
        assert_eq!(reversed_wall, reverse_wall(&wall));
    }

    #[test]
    fn test_reverse_wall() {
        reverse_wall_compare_utility(vec![1, 1, 2, 1], vec![7, 7, 6, 7]);
        reverse_wall_compare_utility(vec![4, 8, 5, 8], vec![4, 0, 3, 0]);
        reverse_wall_compare_utility(vec![7, 8, 8, 8], vec![1, 0, 0, 0]);
        reverse_wall_compare_utility(vec![0, 0, 0, 1], vec![8, 8, 8, 7]);
    }

    fn assert_reverse_player_utility(x: i32, y: i32, expected_x: i32, expected_y: i32) {
        let player = Player::new(x, y, 10, PlayerType::Regular);
        let expected_reverse_player = Player::new(expected_x, expected_y, 10, PlayerType::Regular);
        let reversed_player = conditionally_reverse_player(&player, true);
        assert_eq!(reversed_player, expected_reverse_player);
    }

    #[test]
    fn test_conditionally_reverse_player() {
        assert_reverse_player_utility(0, 0, MAP_SIZE - 1, MAP_SIZE - 1);
        assert_reverse_player_utility(1, 1, MAP_SIZE - 2, MAP_SIZE - 2);
        assert_reverse_player_utility(0, MAP_SIZE - 1, MAP_SIZE - 1, 0);
    }

    #[test]
    fn correct_mirroring_of_corners() {
        assert_eq!(reverse_coordinate(0), MAP_SIZE - 1);
        assert_eq!(reverse_coordinate(MAP_SIZE - 1), 0);
    }
}
