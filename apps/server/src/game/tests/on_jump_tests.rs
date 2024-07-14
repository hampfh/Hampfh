#[cfg(test)]
mod tests {
    use crate::game::{
        game_state::{ErrorType, Game, GameConfig, GameResult, Wall},
        player::{Player, PlayerType},
        tests::util::{_run_core_test, _run_test_with_custom_game_session, aj, at, mock_player},
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
        _run_core_test(
            script.clone(),
            script,
            |result, _| {
                println!("Result: {:?}", result);
                result == GameResult::PlayerTwoWon
            },
            false,
        );
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
        _run_core_test(
            script.clone(),
            script,
            |result, _| {
                println!("Result: {:?}", result);
                result == GameResult::PlayerOneWon
            },
            false,
        );
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
        _run_core_test(
            script.clone(),
            script,
            |result, _| match result {
                GameResult::Error(ErrorType::GameError {
                    reason: _,
                    fault: __,
                }) => true,
                _ => false,
            },
            false,
        );
    }

    fn horizontal_spawn() -> Game {
        return Game::custom_new(
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
            GameConfig::new(),
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
            |result, _| match result {
                GameResult::Error(ErrorType::GameError { reason, fault }) => {
                    reason.contains("bounds") && fault.unwrap() == PlayerType::Flipped
                }
                _ => false,
            },
            false,
        );
        _run_test_with_custom_game_session(
            gen_script(1, 1),
            gen_script(1, 1),
            &mut horizontal_spawn(),
            |result, _| match result {
                GameResult::Error(ErrorType::GameError { reason, fault }) => {
                    reason.contains("bounds") && fault.unwrap() == PlayerType::Regular
                }
                _ => false,
            },
            false,
        );
    }

    #[test]
    /// No jump function
    fn no_jump_function() {
        let script = format!(
            "
                function onTurn()
                    return \"0\"
                end
            "
        );

        _run_core_test(
            script.clone(),
            script,
            |result, _| match result {
                GameResult::Error(ErrorType::RuntimeError { reason, fault: __ }) => {
                    reason.contains("onJump")
                }
                _ => false,
            },
            false,
        );
    }

    #[test]
    /// Attempt to jump into wall
    ///
    /// In this test the players will spawn
    /// next to each other and there will be
    /// a wall behind the second player.
    /// The program should fail when the first
    /// player attempts to jump over the second player...
    fn jump_into_wall() {
        let script = aj(at(String::new()));

        _run_test_with_custom_game_session(
            script.clone(),
            script,
            &mut Game::custom_new(
                mock_player(0, 4, 0, PlayerType::Regular),
                mock_player(0, 5, 0, PlayerType::Flipped),
                vec![Wall {
                    x1: 0,
                    y1: 3,
                    x2: 0,
                    y2: 2,
                }],
                String::new(),
                GameConfig::new(),
            ),
            |result, _| match result {
                GameResult::Error(ErrorType::GameError { reason, fault }) => {
                    reason.contains("occupied") && fault.unwrap() == PlayerType::Flipped
                }
                _ => false,
            },
            false,
        );
    }

    #[test]
    /// Out of bounds jump
    ///
    ///
    fn jump_out_of_bounds() {
        // It is allowed to jump out of bounds
        // if it is the winning move
        let script = aj(at(String::new()));

        _run_test_with_custom_game_session(
            script.clone(),
            script,
            &mut Game::custom_new(
                mock_player(0, 0, 0, PlayerType::Regular),
                mock_player(0, 1, 0, PlayerType::Flipped),
                Vec::new(),
                String::new(),
                GameConfig::new(),
            ),
            |result, _| result == GameResult::PlayerOneWon,
            false,
        );

        // Attempt to jump out of bounds horizontally
        let sideways = format!(
            "
                function onTurn()
                    return \"1\"
                end
                function onJump()
                    return \"1\"
                end
            "
        );
        _run_test_with_custom_game_session(
            sideways.clone(),
            sideways,
            &mut Game::custom_new(
                mock_player(8, 4, 0, PlayerType::Regular),
                mock_player(7, 4, 0, PlayerType::Flipped),
                Vec::new(),
                String::new(),
                GameConfig::new(),
            ),
            |result, _| match result {
                GameResult::Error(ErrorType::GameError { reason, fault }) => {
                    reason.contains("bounds") && fault.unwrap() == PlayerType::Flipped
                }
                _ => false,
            },
            true,
        );
    }
}
