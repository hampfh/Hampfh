use std::{
    sync::{Arc, Mutex},
    time::Duration,
};

use crate::game::{game_state::ErrorType, player::PlayerType};
use terminate_thread::Thread;

use crate::game::{
    board::{populate_board, serialize_board},
    game_state::Wall,
    map_mirroring::{conditionally_reverse_player, conditionally_reverse_walls},
    player::Player,
};

pub(crate) fn execute_lua_in_sandbox(
    sandbox: Arc<Mutex<mlua::Lua>>,
    lua_script_to_run: String,
    active_player_type: PlayerType,
    expect_return: bool,
    timeout: Duration,
) -> Result<String, ErrorType> {
    let (tx, rx) = std::sync::mpsc::channel::<Result<String, mlua::Error>>();
    // Sandbox execution of script
    // Limit execution time to 1 second
    let terminatable_thread = Thread::spawn(move || {
        let active_sandbox = sandbox.lock().unwrap();
        if let Err(err) = active_sandbox.load(&lua_script_to_run).exec() {
            tx.send(Err(err)).unwrap();
        }

        if !expect_return {
            tx.send(Ok("".to_string())).unwrap();
            return;
        }

        let raw_player_move = active_sandbox
            .globals()
            .get::<_, String>("ExternalGlobalVarResult");
        drop(active_sandbox);

        tx.send(raw_player_move).unwrap();
    });

    // Second time we either get the result or a timeout error
    let player_move = match rx.recv_timeout(timeout) {
        Ok(returned) => match returned {
            Ok(move_string) => move_string,
            Err(error) => {
                return Err(ErrorType::RuntimeError {
                    reason: error.to_string(),
                    fault: Some(active_player_type),
                })
            }
        },
        Err(_) => {
            terminatable_thread.terminate();
            return Err(ErrorType::TurnTimeout {
                fault: Some(active_player_type),
            });
        }
    };

    Ok(player_move)
}

pub(crate) fn get_lua_start_inject(function_name: String, game_object: String) -> String {
    return format!(
        "ExternalGlobalVarResult = {}({})",
        function_name, game_object
    );
}

pub(crate) fn create_lua_game_object(
    walls: &Vec<Wall>,
    player_one_turn: bool,
    player_one: &Player,
    player_two: &Player,
) -> String {
    let reverse = !player_one_turn;

    let walls = conditionally_reverse_walls(walls, reverse);
    let conditionally_reversed_player_one = conditionally_reverse_player(player_one, reverse);
    let conditionally_reversed_player_two = conditionally_reverse_player(player_two, reverse);

    let serialized_board = serialize_board(populate_board(
        &conditionally_reversed_player_one,
        &conditionally_reversed_player_two,
        &walls,
    ));

    let (serialized_player, serialized_opponent) = match player_one_turn {
        true => (
            serialize_player(&conditionally_reversed_player_one),
            serialize_player(&conditionally_reversed_player_two),
        ),
        false => (
            serialize_player(&conditionally_reversed_player_two),
            serialize_player(&conditionally_reversed_player_one),
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

pub(crate) fn assert_lua_core_functions(
    program: String,
    player_type: PlayerType,
) -> Result<(), ErrorType> {
    if !program.contains("function onTurn(") {
        return Err(ErrorType::RuntimeError {
            reason: "onTurn() function not found, this function is mandatory".to_string(),
            fault: Some(player_type),
        });
    }
    if !program.contains("function onJump(") {
        return Err(ErrorType::RuntimeError {
            reason: "onJump() function not found, this function is mandatory".to_string(),
            fault: Some(player_type),
        });
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use crate::game::{
        game_state::{ErrorType, Game, GameConfig},
        player::PlayerType,
        sandbox::sandbox_executor::{create_lua_game_object, get_lua_start_inject},
    };

    use super::execute_lua_in_sandbox;

    fn mock_game() -> Game {
        let config = GameConfig::new();
        let game = Game::new(config);
        return game;
    }

    fn mock_lua_context(game: &Game) -> String {
        return create_lua_game_object(
            &game.walls,
            game.player_one_turn,
            &game.player_one,
            &game.player_two,
        );
    }

    #[test]
    fn terminates_thread_on_long_execution() {
        let game = mock_game();
        let lua_context = mock_lua_context(&game);

        execute_lua_in_sandbox(
            game.get_active_sandbox(),
            r#"
            function onTurn(context) 
                return "0"
            end
            function onJump(context) 
                return "0"
            end
        "#
            .to_string(),
            PlayerType::Flipped,
            false,
            Duration::from_millis(500),
        )
        .unwrap();

        let inject = get_lua_start_inject("onTurn".to_string(), lua_context);

        assert!(execute_lua_in_sandbox(
            game.get_active_sandbox(),
            inject.clone(),
            game.get_active_player_type(),
            true,
            Duration::from_millis(500)
        )
        .is_ok());

        let game = mock_game();
        execute_lua_in_sandbox(
            game.get_active_sandbox(),
            r#"
            function onTurn(context) 
                while true do
                end 
                return "0"
            end
            function onJump(context) 
                return "0"
            end
        "#
            .to_string(),
            PlayerType::Flipped,
            false,
            Duration::from_millis(500),
        )
        .unwrap();

        match execute_lua_in_sandbox(
            game.get_active_sandbox(),
            inject,
            game.get_active_player_type(),
            true,
            Duration::from_millis(500),
        ) {
            Err(ErrorType::TurnTimeout {
                fault: Some(PlayerType::Flipped),
            }) => assert!(true),
            data => panic!("Expected timeout, got {:?}", data),
        }
    }
}
