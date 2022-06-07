extern crate pathfinding;
use pathfinding::prelude::astar;

use super::validation::{valid_tile};
use super::game::{Game, MAP_SIZE};
use super::player::{Player};

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Pos(i32, i32);

impl Pos {
	fn distance(&self, other: &Pos) -> u32 {
		(self.0.abs_diff(other.0) + self.1.abs_diff(other.1)) as u32
	}

	fn successors(&self, game: &mut Game) -> Vec<(Pos, u32)> {
		add_walkable_tiles(game, self)
			.into_iter().map(|p| (p, 1)).collect()
	}
}

fn add_walkable_tiles(game: &mut Game, pos: &Pos) -> Vec<Pos> {
	let walls = game.walls.clone();
	let player_one = game.player_one.clone();
	let player_two = game.player_two.clone();
	let mut directions = Vec::new();

	// UP
	if valid_tile(&walls, &player_one, &player_two, pos.0, pos.1 - 1).0 {
		directions.push(Pos(pos.0, pos.1 - 1));
	}
	// RIGHT
	if valid_tile(&walls, &player_one, &player_two, pos.0 + 1, pos.1).0 {
		directions.push(Pos(pos.0 + 1, pos.1));
	}
	// DOWN
	if valid_tile(&walls, &player_one, &player_two, pos.0, pos.1 + 1).0 {
		directions.push(Pos(pos.0, pos.1 + 1));
	}
	// LEFT
	if valid_tile(&walls, &player_one, &player_two, pos.0 - 1, pos.1).0 {
		directions.push(Pos(pos.0 - 1, pos.1));
	}

	return directions;
}

pub fn path_exists_for_players(game: &mut Game) -> bool {
	// Player one wants to get to y = MAP_SIZE - 1
	let player_one = game.player_one.clone();
	// Player two wants to get to y = 0
	let player_two = game.player_two.clone();

	for i in 0..MAP_SIZE {
		if path_exists(game, &player_one, Pos(i, 0)) {
			println!("Exists for player one");
			return true;
		}
		
		if path_exists(game, &player_two, Pos(i, MAP_SIZE - 1)) {
			println!("Exists for player two");
			return true;
		}
	}
	return false;
}

fn path_exists(game: &mut Game, player: &Player, target: Pos) -> bool {
	let result = astar(
		&Pos(player.x, player.y),
		|pos| pos.successors(game),
		|pos| pos.distance(&target),
		|pos| pos == &target	// This is the winning node regardless if you are player 1 or 2
	);

	match result {
		Some(path) => {
			println!("Path found: {:?}", path);
			return true;
		},
		None => {
			return false;
		}
	}
}