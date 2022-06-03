mod game;
use game::game::Game;

fn main() {
    println!("Hello, world!") ;

    let program = std::fs::read_to_string("/Users/hampfh/dev/Projects/rust/hahalang/scripts/first.lua" )
        .expect("Something went wrong reading the file");

    let mut lua = hlua::Lua::new();
    lua.execute::<()>(&program).unwrap();

    let result: i32 = lua.get("result").unwrap();
    println!("result: {}", result);

    let mut app = Game::new();
    app.start(program.clone(), program);
}


// Load two files
// execute them to load functionality