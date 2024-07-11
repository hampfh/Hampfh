use super::game_state::Wall;
use super::game_state::MAP_SIZE;
use crate::game::player::Player;

#[derive(std::fmt::Debug, Copy, Clone, PartialEq, Eq)]
pub enum Tile {
    Empty = 0,
    P1 = 1,
    P2 = 2,
    Wall = 3,
}

pub(crate) fn populate_board(
    player_one: &Player,
    player_two: &Player,
    walls: &Vec<Wall>,
) -> Vec<Tile> {
    let mut draw_buffer = create_empty_board();

    for wall in walls {
        place_tile(&mut draw_buffer, wall.x1, wall.y1, Tile::Wall);
        place_tile(&mut draw_buffer, wall.x2, wall.y2, Tile::Wall);
    }

    // Place players
    place_tile(&mut draw_buffer, player_one.x, player_one.y, Tile::P1);
    place_tile(&mut draw_buffer, player_two.x, player_two.y, Tile::P2);

    return draw_buffer;
}

fn place_tile(buffer: &mut Vec<Tile>, x: i32, y: i32, tile: Tile) {
    buffer[(y * MAP_SIZE + x) as usize] = tile;
}

fn create_empty_board() -> Vec<Tile> {
    let mut buffer: Vec<Tile> = Vec::new();
    for _ in 0..(MAP_SIZE * MAP_SIZE) {
        buffer.push(Tile::Empty)
    }
    return buffer;
}

pub fn serialize_board(board: Vec<Tile>) -> String {
    let mut output = String::from("{");
    // Serialize board
    for tile in board.iter() {
        let value = *tile as i32;
        output.push_str(&format!("{},", value));
    }
    output.push_str("}");
    return output;
}

pub fn board_to_string(board: Vec<Tile>) -> String {
    // Serialize board
    let mut output = String::from("");
    for tile in board.iter() {
        output.push_str(&(*tile as i32).to_string());
    }
    return output;
}

pub fn board_from_string(board: String) -> Vec<Tile> {
    let mut output = Vec::new();
    for char in board.chars() {
        match char.to_digit(10).unwrap() {
            0 => output.push(Tile::Empty),
            1 => output.push(Tile::P1),
            2 => output.push(Tile::P2),
            3 => output.push(Tile::Wall),
            _ => panic!("Invalid tile"),
        };
    }
    return output;
}

#[cfg(test)]
mod tests {

    use crate::game::{
        game_state::{Wall, MAP_SIZE},
        player::{Player, PlayerType},
    };

    use super::{board_from_string, board_to_string, create_empty_board, place_tile, Tile};

    #[test]
    fn populate_board_test() {
        let p1 = Player::new(4, 4, 10, PlayerType::Flipped);
        let p2 = Player::new(8, 8, 10, PlayerType::Regular);
        // Create walls
        let mut walls: Vec<Wall> = Vec::new();
        for i in 0..MAP_SIZE {
            walls.push(Wall {
                x1: i,
                y1: 0,
                x2: i,
                y2: 1,
            });
            walls.push(Wall {
                x1: i,
                y1: MAP_SIZE - 1,
                x2: i,
                y2: MAP_SIZE - 2,
            });
            walls.push(Wall {
                x1: 0,
                y1: i,
                x2: 1,
                y2: i,
            });
            walls.push(Wall {
                x1: MAP_SIZE - 1,
                y1: i,
                x2: MAP_SIZE - 2,
                y2: i,
            });
        }

        // Create expected board
        let mut expected_board = create_empty_board();
        for wall in walls.clone() {
            place_tile(&mut expected_board, wall.x1, wall.y1, Tile::Wall);
            place_tile(&mut expected_board, wall.x2, wall.y2, Tile::Wall);
        }
        place_tile(&mut expected_board, p1.x, p1.y, Tile::P1);
        place_tile(&mut expected_board, p2.x, p2.y, Tile::P2);

        let board = super::populate_board(&p1, &p2, &walls);
        assert_eq!(board, expected_board);
    }

    #[test]
    fn create_empty_board_test() {
        let board = create_empty_board();
        for tile in board.iter() {
            assert_eq!(*tile, Tile::Empty);
        }
    }

    #[test]
    fn place_tile_test() {
        let mut board = create_empty_board();

        place_tile(&mut board, 0, 0, Tile::P1);
        assert_eq!(board[0], Tile::P1);

        place_tile(&mut board, MAP_SIZE - 1, MAP_SIZE - 1, Tile::P1);
        assert_eq!(board[(MAP_SIZE * MAP_SIZE - 1) as usize], Tile::P1);

        place_tile(&mut board, 0, 0, Tile::P2);
        assert_eq!(board[0], Tile::P2);

        place_tile(&mut board, MAP_SIZE - 1, 0, Tile::Wall);
        place_tile(&mut board, MAP_SIZE - 2, 0, Tile::Wall);
        assert_eq!(board[(MAP_SIZE - 1) as usize], Tile::Wall);
        assert_eq!(board[(MAP_SIZE - 2) as usize], Tile::Wall);
    }

    #[test]
    fn board_conversions_test() {
        let mut board_string = String::new();
        let mut board: Vec<Tile> = Vec::new();
        for i in 0..(MAP_SIZE * MAP_SIZE) {
            let (tile, tile_string) = match i % 4 {
                0 => (Tile::Empty, "0"),
                1 => (Tile::P1, "1"),
                2 => (Tile::P2, "2"),
                3 => (Tile::Wall, "3"),
                num => panic!("Invalid tile {}", num),
            };
            board_string.push_str(tile_string);
            board.push(tile);
        }

        assert_eq!(board.clone(), board_from_string(board_string.clone()));
        assert_eq!(board_string, board_to_string(board));
    }
}
