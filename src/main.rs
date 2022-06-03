mod game;
use game::game::Game;

fn main() {
    println!("Hello, world!") ;

    let program = std::fs::read_to_string("/Users/hampfh/dev/Projects/rust/hahalang/scripts/first.lua" )
        .expect("Something went wrong reading the file");

    let mut app = Game::new();
    app.start(program.clone(), program);
}


// Load two files
// execute them to load functionality