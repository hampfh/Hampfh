use rlua::Lua;

use crate::game::game::{
    ErrorType, Game, GameState, Move, Wall, INITIAL_WALL_COUNT, MAP_SIZE, MAX_TURNS,
};
use crate::game::graphics::draw_game;
use crate::game::player::{Player, PlayerType};
use crate::game::turn;
use crate::terminate_thread;
use std::sync::{Arc, Mutex};
use std::time::Duration;

pub fn new(std: String) -> Game {
    Game {
        game_state: GameState::Running,
        player_one: Player::new(
            MAP_SIZE / 2,
            MAP_SIZE - 1,
            INITIAL_WALL_COUNT,
            PlayerType::Flipped,
        ),
        player_two: Player::new(MAP_SIZE / 2, 0, INITIAL_WALL_COUNT, PlayerType::Regular),
        walls: Vec::new(),
        player_one_sandbox: Arc::new(Mutex::new(Lua::new())),
        player_two_sandbox: Arc::new(Mutex::new(Lua::new())),
        player_one_turn: true,
        last_move: None,
        std: std,
    }
}

pub fn start(game: &mut Game, program1: String, program2: String) -> GameState {
    let std = game.std.clone();

    let clone_one = game.player_one_sandbox.clone();
    let clone_two = game.player_two_sandbox.clone();

    // Run programs for the first time
    // We limit execution here to 100 milli-seconds
    let (tx, rx) = std::sync::mpsc::channel::<Result<Option<usize>, String>>();
    std::thread::spawn(move || {
        tx.send(Ok(Some(thread_id::get()))).unwrap();

        let player_one_sandbox = clone_one.lock().unwrap();
        let player_two_sandbox = clone_two.lock().unwrap();

        match player_one_sandbox.context(|ctx| ctx.load(&program1).exec()) {
            Ok(_) => (),
            Err(_) => {
                tx.send(Err("Your script could not be executed".to_string()))
                    .unwrap();
            }
        }
        match player_two_sandbox.context(|ctx| ctx.load(&program2).exec()) {
            Ok(_) => (),
            Err(_) => {
                tx.send(Err("Opponent script could not be executed".to_string()))
                    .unwrap();
            }
        }

        drop(player_one_sandbox);
        drop(player_two_sandbox);
        tx.send(Ok(None)).unwrap();
    });

    let thread_id = rx.recv().unwrap().unwrap().unwrap();
    match rx.recv_timeout(Duration::from_millis(100)) {
        Ok(Ok(_)) => (),
        Ok(Err(err)) => {
            return GameState::Error(ErrorType::RuntimeError {
                reason: err.to_string(),
            });
        }
        Err(_) => {
            terminate_thread::terminate_thread(thread_id);
            return GameState::Error(ErrorType::TurnTimeout);
        }
    }

    // Load standard library
    // We don't have to check errors here since
    // this is deterministic and all lua code
    // comes from us
    let first = game.player_one_sandbox.clone();
    first
        .lock()
        .unwrap()
        .context(|ctx| ctx.load(&std).exec())
        .unwrap();
    let second = game.player_two_sandbox.clone();
    second
        .lock()
        .unwrap()
        .context(|ctx| ctx.load(&std).exec())
        .unwrap();

    // Unlock mutexes
    drop(first);
    drop(second);

    game_loop(game);

    return game.game_state.clone();
}

pub fn game_loop(game: &mut Game) {
    let mut round = 1;
    while game.game_state == GameState::Running {
        println!("\n\n## Round {} ##", round);
        update(game);
        winner(game);
        if round >= MAX_TURNS {
            game.game_state = GameState::Error(ErrorType::GameDeadlock);
        }
        round += 1;
    }
}

pub fn update(game: &mut Game) {
    let result = turn::on_turn(game);
    game.game_state = match result {
        Ok(_) => game.game_state.clone(),
        Err(err) => GameState::Error(err),
    };

    if cfg!(debug_assertions) && !cfg!(test) {
        draw_game(&game);
        std::thread::sleep(std::time::Duration::from_millis(200));
    }
}

pub fn winner(game: &mut Game) {
    if game.player_one.y == 0 {
        game.game_state = GameState::PlayerOneWon;
    } else if game.player_two.y == MAP_SIZE - 1 {
        game.game_state = GameState::PlayerTwoWon;
    }
}

/**
 * Returns a tuple, the first player is always the active one
 * the second is the non-active player
 */
pub fn get_active_player(game: &mut Game) -> (&mut Player, &Player) {
    if game.player_one_turn {
        return (&mut game.player_one, &game.player_two);
    }
    return (&mut game.player_two, &game.player_one);
}

// Converts a string like ["x1,y1,x2,y2" -> Wall]
pub fn deserialize_wall(input: &str) -> Move {
    let splits = input.split(",").map(|s| s.trim()).collect::<Vec<&str>>();
    if splits.len() != 4 as usize {
        return Move::Invalid {
            reason: format!("Invalid return format, expected 4 values, got: [{}]", input),
        };
    }
    let result = splits
        .iter()
        .map(|x| x.trim())
        .map(|x| x.parse::<i32>().unwrap_or_else(|_| -1))
        .collect::<Vec<i32>>();

    // If any of the values are invalid (negative), the move is invalid
    if result.iter().any(|x| *x < 0) {
        return Move::Invalid {
            reason: "Invalid wall param".to_string(),
        };
    }

    return Move::Wall(Wall {
        x1: result[0],
        y1: result[1],
        x2: result[2],
        y2: result[3],
    });
}
