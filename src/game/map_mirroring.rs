use crate::game::game::{MAP_SIZE, Move, Wall};

/**
 * This file includes all logic for the map mirroring process.
 * Aka: All of the scripts will always think they are playing from the
 * same side thus we need to perform some logic to reverse the map for 
 * the second player.
 */

pub fn reverse_move(player_move: Move) -> Move {
	return match player_move {
		Move::Up => Move::Down,
		Move::Right => Move::Left,
		Move::Down => Move::Up,
		Move::Left => Move::Right,
		Move::Wall(wall) => Move::Wall(Wall {
			x1: reverse_coordinate(wall.x2),
			y1: reverse_coordinate(wall.y2),
			x2: reverse_coordinate(wall.x1),
			y2: reverse_coordinate(wall.y1)
		}),
		Move::Invalid { reason } => Move::Invalid { reason }
	};
}

pub fn reverse_coordinate(coordinate: i32) -> i32 {
	return (MAP_SIZE - 1) - coordinate;
}