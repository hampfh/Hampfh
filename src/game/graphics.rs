use super::board::{populate_board, Tile};
use super::game::{Game, MAP_SIZE};

pub(crate) fn draw_game(game: &Game) {
    let buffer = populate_board(&game.player_one, &game.player_two, &game.walls);

    let mut count = 0;

    println!("Last move: {:?}", game.last_move);

    print!(" ");
    for i in 0..MAP_SIZE {
        print!("{}", i);
    }
    println!();
    // Upper wall
    for _ in 0..MAP_SIZE + 2 {
        print!("#");
    }
    println!();
    print!("#");
    let mut line = 0;
    for tile in buffer.iter() {
        match tile {
            Tile::Empty => print!(" "),
            Tile::P1 => print!("O"),
            Tile::P2 => print!("X"),
            Tile::Wall => print!("#"),
        }

        count += 1;
        // Side wall
        if count >= MAP_SIZE {
            println!("# {}", line);
            count = 0;
            print!("#");
            line += 1;
        }
    }

    // Lower wall
    for _ in 0..MAP_SIZE + 1 {
        print!("#");
    }
    println!();
}
