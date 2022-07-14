#[cfg(test)]
mod tests {
    use crate::game::{
        game::{ErrorType, Game, GameResult},
        methods::custom_new,
        player::{Player, PlayerType},
        tests::util::{_run_core_test, _run_test_with_custom_game_session},
    };

    #[test]
    /// Test complete by going forward
    ///
    /// In both players will go forward and if they
    /// encounter another player they will jump over it
    fn forward_walk() {
        let script = format!(
            "
				function onTurn()
					return \"0\"
				end

				function onJump()
					return \"0\"
				end
			"
        );
        _run_core_test(script.clone(), script, |result| {
            println!("Result: {:?}", result);
            result == GameResult::PlayerTwoWon
        });
    }

    #[test]
    /// Walk forward but jump sideways
    fn forward_sideways_jump() {
        let script = format!(
            "
				function onTurn()
					return \"0\"
				end

				function onJump()
					return \"1\"
				end
			"
        );
        _run_core_test(script.clone(), script, |result| {
            println!("Result: {:?}", result);
            result == GameResult::PlayerOneWon
        });
    }

    #[test]
    /// Attempt jumpback
    ///
    /// This script will attempt to
    /// through the opponent jump
    /// back to where he was
    /// originally standing.
    fn jump_back() {
        let script = format!(
            "
				function onTurn()
					return \"0\"
				end

				function onJump()
					return \"2\"
				end
			"
        );
        _run_core_test(script.clone(), script, |result| match result {
            GameResult::Error(ErrorType::GameError {
                reason: _,
                fault: __,
            }) => true,
            _ => false,
        });
    }

    fn horizontal_spawn() -> Game {
        return custom_new(
            Player {
                x: 0,
                y: 4,
                wall_count: 0,
                player_type: PlayerType::Flipped,
            },
            Player {
                x: 8,
                y: 4,
                wall_count: 0,
                player_type: PlayerType::Regular,
            },
            Vec::new(),
            String::new(),
        );
    }

    #[test]
    /// Directional jump tests
    fn directional_jump() {
        let gen_script = |on_turn_return: i32, on_jump_return: i32| {
            format!(
                "
                function onTurn()
                    return \"{}\"
                end

                function onJump()
                    return \"{}\"
                end
            ",
                on_turn_return, on_jump_return
            )
        };

        _run_test_with_custom_game_session(
            gen_script(1, 0),
            gen_script(1, 0),
            &mut horizontal_spawn(),
            |result| match result {
                GameResult::Error(ErrorType::GameError { reason, fault }) => {
                    reason.contains("bounds") && fault.unwrap() == PlayerType::Flipped
                }
                _ => false,
            },
        );
        _run_test_with_custom_game_session(
            gen_script(1, 1),
            gen_script(1, 1),
            &mut horizontal_spawn(),
            |result| match result {
                GameResult::Error(ErrorType::GameError { reason, fault }) => {
                    reason.contains("bounds") && fault.unwrap() == PlayerType::Regular
                }
                _ => false,
            },
        );
    }

    #[test]
    /// No jump function
    fn nu_jump_function() {
        let script = format!(
            "
                function onTurn()
                    return \"0\"
                end
            "
        );

        _run_core_test(script.clone(), script, |result| match result {
            GameResult::Error(ErrorType::RuntimeError { reason, fault: __ }) => {
                reason.contains("onJump")
            }
            _ => false,
        });
    }
}
