extern crate pathfinding;
use pathfinding::prelude::astar;

use super::game_state::{Wall, MAP_SIZE};
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
        Some(_) => true,
        None => false,
    }
}

#[cfg(test)]

mod tests {
    use crate::game::{
        game_state::Wall,
        player::{Player, PlayerType},
    };

    use super::{path_exists, path_exists_for_players, Pos};

    fn create_wall(y: i32) -> Vec<Wall> {
        return vec![
            Wall {
                x1: 0,
                y1: y,
                x2: 1,
                y2: y,
            },
            Wall {
                x1: 2,
                y1: y,
                x2: 3,
                y2: y,
            },
            Wall {
                x1: 4,
                y1: y,
                x2: 5,
                y2: y,
            },
            Wall {
                x1: 6,
                y1: y,
                x2: 7,
                y2: y,
            },
            Wall {
                x1: 8,
                y1: y,
                x2: 8,
                y2: 6,
            },
        ];
    }

    #[test]
    fn tests_clear_path() {
        let p1 = Player {
            x: 4,
            y: 8,
            player_type: PlayerType::Regular,
            wall_count: 10,
        };
        let p2 = Player {
            x: 0,
            y: 4,
            player_type: PlayerType::Flipped,
            wall_count: 10,
        };
        let mut horizontal_wall = create_wall(5);
        // Clear path
        assert!(path_exists(&Vec::new(), &p1, &p2, &p1, Pos(0, 0)));
        // Horizontal wall in middle
        assert_eq!(
            path_exists(&horizontal_wall, &p1, &p2, &p1, Pos(0, 0)),
            false
        );

        // Horizontal wall with 1-block gap
        horizontal_wall.pop();

        // Player doesn't affect path
        assert!(path_exists(
            &horizontal_wall,
            &p1,
            &Player {
                x: 8,
                y: 5,
                player_type: PlayerType::Flipped,
                wall_count: 10
            },
            &p1,
            Pos(0, 0)
        ),);
    }

    #[test]
    fn test_all_permutations_for_players() {
        let p1 = Player {
            x: 4,
            y: 7,
            player_type: PlayerType::Regular,
            wall_count: 10,
        };
        let p2 = Player {
            x: 4,
            y: 1,
            player_type: PlayerType::Flipped,
            wall_count: 10,
        };

        for i in 0..5 {
            for j in 0..5 {
                let mut walls = create_wall(0);
                walls.append(&mut create_wall(8));
                walls.remove(i);
                assert!(path_exists_for_players(&walls, &p1, &p2).is_err());
                walls.remove(4 + j);
                assert!(path_exists_for_players(&walls, &p1, &p2).is_ok());
            }
        }
    }
}
