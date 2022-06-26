use super::game::Wall;
use super::game::MAP_SIZE;
use crate::game::player::Player;

#[derive(std::fmt::Debug, Copy, Clone)]
pub enum Tile {
    Empty = 0,
    P1 = 1,
    P2 = 2,
    Wall = 3,
}

pub fn populate_board(player_one: &Player, player_two: &Player, walls: &Vec<Wall>) -> Vec<Tile> {
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
