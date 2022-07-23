use std::{
    sync::{Arc, Mutex},
    time::Duration,
};

use crate::game::{
    game::ErrorType, methods::get_active_player_type, sandbox::terminate_thread::terminate_thread,
};

use crate::game::{
    board::{populate_board, serialize_board},
    game::Wall,
    map_mirroring::{conditionally_reverse_player, conditionally_reverse_walls},
    player::Player,
};

pub struct ThreadReturn {
    thread_id: Option<usize>,
    player_move: Result<String, rlua::Error>,
}

pub(crate) fn execute_lua_in_sandbox(
    player_one_sandbox_mutex: Arc<Mutex<rlua::Lua>>,
    player_two_sandbox_mutex: Arc<Mutex<rlua::Lua>>,
    walls: Vec<Wall>,
    player_one: Player,
    player_two: Player,
    player_one_turn: bool,
    lua_function: String,
) -> Result<String, ErrorType> {
    let (tx, rx) = std::sync::mpsc::channel::<ThreadReturn>();
    // Sandbox execution of script
    // Limit execution time to 1 second
    std::thread::spawn(move || {
        tx.send(ThreadReturn {
            thread_id: Some(thread_id::get()),
            player_move: Ok(String::new()),
        })
        .unwrap();

        let starting_script = get_lua_script(
            lua_function,
            create_lua_game_object(walls, player_one_turn, player_one, player_two),
        );

        let mut active_sandbox = player_one_sandbox_mutex.lock().unwrap();
        if !player_one_turn {
            drop(active_sandbox);
            active_sandbox = player_two_sandbox_mutex.lock().unwrap();
        }

        match active_sandbox.context(|ctx| ctx.load(&starting_script).exec()) {
            Ok(_) => (),
            Err(err) => {
                tx.send(ThreadReturn {
                    thread_id: None,
                    player_move: Err(err),
                })
                .unwrap();
            }
        }

        let raw_player_move =
            active_sandbox.context(|ctx| ctx.globals().get::<_, String>("ExternalGlobalVarResult"));
        drop(active_sandbox);

        tx.send(ThreadReturn {
            thread_id: None,
            player_move: raw_player_move,
        })
        .unwrap();
    });

    // First time we send the thread id through
    // This does not have to be timed checked since this is before we
    // execute the script.
    let sandbox_thread_id = match rx.recv().unwrap().thread_id {
        Some(id) => id,
        _ => panic!("Could not get thread id"),
    };

    // Second time we either get the result or a timeout error
    let player_move = match rx.recv_timeout(Duration::from_millis(500)) {
        Ok(returned) => match returned.player_move {
            Ok(move_string) => move_string,
            Err(error) => {
                return Err(ErrorType::RuntimeError {
                    reason: error.to_string(),
                    fault: Some(get_active_player_type(player_one_turn)),
                })
            }
        },
        Err(_) => {
            println!("Timed out");
            terminate_thread(sandbox_thread_id);
            return Err(ErrorType::TurnTimeout {
                fault: Some(get_active_player_type(player_one_turn)),
            });
        }
    };

    Ok(player_move)
}

fn get_lua_script(function_name: String, game_object: String) -> String {
    return format!(
        "ExternalGlobalVarResult = {}({})",
        function_name, game_object
    );
}

pub(crate) fn create_lua_game_object(
    walls: Vec<Wall>,
    player_one_turn: bool,
    player_one: Player,
    player_two: Player,
) -> String {
    let reverse = !player_one_turn;

    let walls = conditionally_reverse_walls(&walls, reverse);

    let serialized_board = serialize_board(populate_board(&player_one, &player_two, &walls));
    let (serialized_player, serialized_opponent) = match player_one_turn {
        true => (
            serialize_player(&conditionally_reverse_player(&player_one, false)),
            serialize_player(&conditionally_reverse_player(&player_two, false)),
        ),
        false => (
            serialize_player(&conditionally_reverse_player(&player_two, true)),
            serialize_player(&conditionally_reverse_player(&player_one, true)),
        ),
    };

    return format!(
        "{{player={}, opponent={}, board={}}}",
        serialized_player, serialized_opponent, serialized_board
    );
}

fn serialize_player(player: &Player) -> String {
    return format!(
        "{{x={}, y={}, wall_count={}}}",
        player.x, player.y, player.wall_count
    );
}
