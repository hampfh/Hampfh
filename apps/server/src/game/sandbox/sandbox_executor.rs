use std::{
    sync::{Arc, Mutex},
    time::Duration,
};

use crate::game::{game::get_active_player_type, game_state::ErrorType};
use terminate_thread::Thread;

use crate::game::{
    board::{populate_board, serialize_board},
    game_state::Wall,
    map_mirroring::{conditionally_reverse_player, conditionally_reverse_walls},
    player::Player,
};

pub(crate) fn execute_lua_in_sandbox(
    player_one_sandbox_mutex: Arc<Mutex<rlua::Lua>>,
    player_two_sandbox_mutex: Arc<Mutex<rlua::Lua>>,
    walls: Vec<Wall>,
    player_one: Player,
    player_two: Player,
    player_one_turn: bool,
    lua_function: String,
) -> Result<String, ErrorType> {
    let (tx, rx) = std::sync::mpsc::channel::<Result<String, rlua::Error>>();
    // Sandbox execution of script
    // Limit execution time to 1 second
    let terminatable_thread = Thread::spawn(move || {
        let starting_script = get_lua_script(
            lua_function,
            create_lua_game_object(walls, player_one_turn, player_one, player_two),
        );

        let mut active_sandbox = player_one_sandbox_mutex.lock().unwrap();
        if !player_one_turn {
            drop(active_sandbox);
            active_sandbox = player_two_sandbox_mutex.lock().unwrap();
        }

        if let Err(err) = active_sandbox.context(|ctx| ctx.load(&starting_script).exec()) {
            tx.send(Err(err)).unwrap();
        }

        let raw_player_move =
            active_sandbox.context(|ctx| ctx.globals().get::<_, String>("ExternalGlobalVarResult"));
        drop(active_sandbox);

        tx.send(raw_player_move).unwrap();
    });

    // Second time we either get the result or a timeout error
    let player_move = match rx.recv_timeout(Duration::from_millis(500)) {
        Ok(returned) => match returned {
            Ok(move_string) => move_string,
            Err(error) => {
                return Err(ErrorType::RuntimeError {
                    reason: error.to_string(),
                    fault: Some(get_active_player_type(player_one_turn)),
                })
            }
        },
        Err(_) => {
            terminatable_thread.terminate();
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
    let conditionally_reversed_player_one = conditionally_reverse_player(&player_one, reverse);
    let conditionally_reversed_player_two = conditionally_reverse_player(&player_two, reverse);

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

#[cfg(test)]
mod tests {
    use crate::game::{
        game_state::{ErrorType, Game, GameConfig},
        load_script_with_validation::load_script_with_validation,
        player::PlayerType,
    };

    use super::execute_lua_in_sandbox;

    fn mock_game() -> Game {
        let config = GameConfig::new();
        let game = Game::new(config);
        return game;
    }

    #[test]
    fn terminates_thread_on_long_execution() {
        let game = mock_game();

        load_script_with_validation(
            &game.player_one_sandbox,
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
        )
        .unwrap();

        assert!(execute_lua_in_sandbox(
            game.player_one_sandbox,
            game.player_two_sandbox,
            game.walls,
            game.player_one,
            game.player_two,
            true,
            "onTurn".to_string(),
        )
        .is_ok());

        let game = mock_game();
        load_script_with_validation(
            &game.player_one_sandbox,
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
        )
        .unwrap();

        match execute_lua_in_sandbox(
            game.player_one_sandbox,
            game.player_two_sandbox,
            game.walls,
            game.player_one,
            game.player_two,
            true,
            "onTurn".to_string(),
        ) {
            Err(ErrorType::TurnTimeout {
                fault: Some(PlayerType::Flipped),
            }) => assert!(true),
            data => panic!("Expected timeout, got {:?}", data),
        }
    }
}
