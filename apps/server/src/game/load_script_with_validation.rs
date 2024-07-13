use std::{
    sync::{Arc, Mutex},
    time::Duration,
};

use rlua::Lua;
use terminate_thread::Thread;

use super::{
    game_state::{ErrorType, GameResult},
    player::PlayerType,
};

pub(crate) fn load_script_with_validation(
    sandbox: &Arc<Mutex<Lua>>,
    script: String,
    player_type: PlayerType,
) -> Result<(), GameResult> {
    match assert_lua_core_functions(script.clone(), player_type.clone()) {
        Ok(_) => (),
        Err(error) => return Err(GameResult::Error(error)),
    }

    // Run programs for the first time
    // We limit execution here to 100 milli-seconds
    let (tx, rx) = std::sync::mpsc::channel::<Result<Option<usize>, String>>();
    let inner_sandbox = sandbox.clone();
    let terminatable_thread = Thread::spawn(move || {
        let player_inner_sandbox = inner_sandbox.lock().unwrap();

        match player_inner_sandbox.context(|ctx| ctx.load(&script).exec()) {
            Ok(_) => (),
            Err(err) => {
                tx.send(Err(format!(
                    "Your script could not be executed, reason: {}",
                    err
                )))
                .unwrap();
            }
        }

        drop(player_inner_sandbox);
        tx.send(Ok(None)).unwrap();
    });

    match rx.recv_timeout(Duration::from_millis(100)) {
        Ok(Ok(_)) => (),
        Ok(Err(err)) => {
            return Err(GameResult::Error(ErrorType::RuntimeError {
                reason: err.to_string(),
                fault: Some(player_type),
            }));
        }
        Err(_) => {
            terminatable_thread.terminate();
            return Err(GameResult::Error(ErrorType::TurnTimeout {
                fault: Some(player_type),
            }));
        }
    }

    Ok(())
}

pub(super) fn assert_lua_core_functions(
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
    use crate::game::{
        game_state::{Game, GameConfig},
        load_script_with_validation::assert_lua_core_functions,
        player::PlayerType,
    };

    use super::load_script_with_validation;

    #[test]
    fn loads_script_into_game() {
        let config = GameConfig::new();
        let game = Game::new(config);
        match load_script_with_validation(
            &game.player_one_sandbox,
            r#"
                local x = 1
                x = x + 1

                function onTurn()
                    return tostring(x)
                end 

                function onJump()
                    return tostring(x)
                end
            "#
            .to_string(),
            PlayerType::Flipped,
        ) {
            Ok(_) => (),
            Err(err) => panic!("Error: {:?}", err),
        }
    }

    #[test]
    fn correctly_checks_if_core_functions_exist() {
        let program = r#"
            function onTurn()
                return "1"
            end 
            function onJump()
                return "1"
            end
        "#;

        assert_eq!(
            assert_lua_core_functions(String::from(program), PlayerType::Regular),
            Ok(())
        );
        assert_eq!(
            assert_lua_core_functions(String::from(program), PlayerType::Flipped),
            Ok(())
        );

        let only_on_turn = r#"
            function onTurn()
                return "1"
            end
        "#;
        assert_eq!(
            assert_lua_core_functions(String::from(only_on_turn), PlayerType::Regular),
            Err(crate::game::game_state::ErrorType::RuntimeError {
                reason: "onJump() function not found, this function is mandatory".to_string(),
                fault: Some(PlayerType::Regular)
            })
        );

        let only_on_jump = r#"
            function onJump()
                return "1"
            end
            "#;
        assert_eq!(
            assert_lua_core_functions(String::from(only_on_jump), PlayerType::Regular),
            Err(crate::game::game_state::ErrorType::RuntimeError {
                reason: "onTurn() function not found, this function is mandatory".to_string(),
                fault: Some(PlayerType::Regular)
            })
        );
    }
}
