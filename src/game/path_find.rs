extern crate pathfinding;
use pathfinding::prelude::astar;

use super::game::{Wall, MAP_SIZE};
use super::player::Player;
use super::validation::valid_tile;

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Pos(pub i32, pub i32);

impl Pos {
    fn distance(&self, other: &Pos) -> u32 {
        (self.0.abs_diff(other.0) + self.1.abs_diff(other.1)) as u32
    }

    fn successors(&self, walls: &Vec<Wall>, p1: &Player, p2: &Player) -> Vec<(Pos, u32)> {
        add_walkable_tiles(walls, p1, p2, self)
            .into_iter()
            .map(|p| (p, 1))
            .collect()
    }
}

fn add_walkable_tiles(walls: &Vec<Wall>, p1: &Player, p2: &Player, pos: &Pos) -> Vec<Pos> {
    let player_one = p1;
    let player_two = p2;
    let mut directions = Vec::new();

    // TODO refactor this
    // UP
    if valid_tile(
        &walls,
        Some(&player_one),
        Some(&player_two),
        pos.0,
        pos.1 - 1,
        true,
    )
    .is_ok()
    {
        directions.push(Pos(pos.0, pos.1 - 1));
    }
    // RIGHT
    if valid_tile(
        &walls,
        Some(&player_one),
        Some(&player_two),
        pos.0 + 1,
        pos.1,
        true,
    )
    .is_ok()
    {
        directions.push(Pos(pos.0 + 1, pos.1));
    }
    // DOWN
    if valid_tile(
        &walls,
        Some(&player_one),
        Some(&player_two),
        pos.0,
        pos.1 + 1,
        true,
    )
    .is_ok()
    {
        directions.push(Pos(pos.0, pos.1 + 1));
    }
    // LEFT
    if valid_tile(
        &walls,
        Some(&player_one),
        Some(&player_two),
        pos.0 - 1,
        pos.1,
        true,
    )
    .is_ok()
    {
        directions.push(Pos(pos.0 - 1, pos.1));
    }

    return directions;
}

pub(crate) fn path_exists_for_players(
    walls: &Vec<Wall>,
    p1: &Player,
    p2: &Player,
) -> Result<(), String> {
    // Player one wants to get to y = MAP_SIZE - 1
    let player_one = p1;
    // Player two wants to get to y = 0
    let player_two = p2;

    let mut player_one_valid = false;
    let mut player_two_valid = false;

    for i in 0..MAP_SIZE {
        if !player_one_valid && path_exists(walls, p1, p2, player_one, Pos(i, 0)) {
            player_one_valid = true;
        }

        if !player_two_valid && path_exists(walls, p1, p2, player_two, Pos(i, MAP_SIZE - 1)) {
            player_two_valid = true;
        }

        if player_one_valid && player_two_valid {
            break;
        }
    }

    if player_one_valid && player_two_valid {
        return Ok(());
    } else if player_one_valid {
        return Err("No path for player 2 available".to_string());
    } else if player_two_valid {
        return Err("No path for player 1 available".to_string());
    }
    return Err("No path for either bot available".to_string());
}

fn path_exists(walls: &Vec<Wall>, p1: &Player, p2: &Player, player: &Player, target: Pos) -> bool {
    let result = astar(
        &Pos(player.x, player.y),
        |pos| pos.successors(walls, p1, p2),
        |pos| pos.distance(&target),
        |pos| pos == &target, // This is the winning node regardless if you are player 1 or 2
    );

    match result {
        Some(path) => {
            println!("Path exists {:?}", path);
            return true;
        }
        None => {
            return false;
        }
    }
}
