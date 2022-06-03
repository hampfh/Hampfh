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

pub fn conditionally_reverse_move(player_move: Option<Move>, condition: bool) -> Option<Move> {
	if !condition || player_move.is_none() { return player_move; } 
	else { return Some(reverse_move(player_move.unwrap())); }
}

pub fn conditionally_reverse_coordinates(coordinates: (i32, i32), condition: bool) -> (i32, i32) {
	if !condition { return (coordinates.0, coordinates.1); } 
	else { return (reverse_coordinate(coordinates.0), reverse_coordinate(coordinates.1)); }
}