mod game;
use game::game::{Game, GameState};

fn main() {
    println!("Hello, world!") ;

    let program = std::fs::read_to_string("/Users/hampfh/dev/Projects/rust/hahalang/scripts/temp.lua" )
        .expect("Something went wrong reading the file");

    let mut app = Game::new();
    match app.start(program.clone(), program) {
        GameState::PlayerOneWon => println!("Player 1 won"),
        GameState::PlayerTwoWon => println!("Player 2 won"),
        _ => ()
    }
}


// Load two files
// execute them to load functionality