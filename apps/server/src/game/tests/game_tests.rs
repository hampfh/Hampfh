#[cfg(test)]
mod tests {
    use crate::game::{
        game::{ErrorType, GameResult, MAP_SIZE},
        methods,
        player::{Player, PlayerType},
        tests::util::{
            _run_core_test, _run_test_with_custom_game_session, aj, load_script, load_std,
        },
    };

    #[test]
    /// Invalid program structure
    ///
    /// Make sure the game properly
    /// fails if the code is invalid,
    /// aka, no onTurn method exists.
    fn fail_on_no_turn_function() {
        let script = format!(
            "
			x = 20
		"
        );

        _run_core_test(script.clone(), script, |state| match state {
            GameResult::Error(ErrorType::RuntimeError { reason, fault }) => {
                reason.contains("onTurn")
                    && fault.is_some()
                    && fault.unwrap() == PlayerType::Flipped
            }
            _ => false,
        })
    }

    #[test]
    /// Forward with clear path
    ///
    /// This script moves both player to the left
    /// and the moves the forward, clearning the
    /// path for both players.
    fn success_when_left_and_forward() {
        let script = aj(format!(
            "
			count = 0
			function onTurn()
				count = count + 1
				if count == 1 then
					return \"1\"
				else
					return \"0\"
				end 
			end
		"
        ));

        _run_core_test(script.clone(), script, |state| {
            state == GameResult::PlayerOneWon
        });
    }

    #[test]
    /// Place invalid wall
    ///
    /// Place a wall that is not contiguous
    fn place_invalid_wall() {
        let script = aj(format!(
            "
            function onTurn()
                -- This is clearly an invalid wall since
                -- the two coordinates are not connected
                return \"0,4,8,8\"
            end
            "
        ));

        _run_core_test(script.clone(), script, |state| {
            state == GameResult::Error(ErrorType::GameError {
                reason: "Invalid wall format, a wall must consist of two adjacent coordinates: ((0,4), (8,8))".to_string(),
                fault: Some(PlayerType::Flipped),
            })
        });
    }

    #[test]
    /// Complete block
    ///
    /// Test fails when player makes it impossible
    /// for opponent to win
    fn fail_on_complete_block() {
        // Should fail on 4th round, since this will block
        // both players from finishing.
        let p1_script = aj(format!(
            "
			round = -1
            y = 7
			function onTurn()  

                round = round + 1

                if round > 4 then
                    return \"0\"
                end

                if round == 4 then
                    return \"8,7,8,8\"
                end

				x = round * 2
				return x .. \",\" .. y .. \",\" .. (x + 1) .. \",\" .. y
			end
		"
        ));

        // This script will go back and forth
        let p2_script = aj(format!(
            "
			round = -1
			function onTurn()
				round = round + 1
				if round % 2 == 0 then
					return \"0\"
				else
					return \"2\"
				end
			end
		"
        ));

        _run_core_test(p1_script, p2_script, |state| {
            state
                == GameResult::Error(ErrorType::GameError {
                    reason: "No path for either bot available".to_string(),
                    fault: Some(PlayerType::Flipped),
                })
        });
    }

    #[test]
    /// Invalid wall
    ///
    /// This script tries to place a wall
    /// outside the boundary of the board.
    fn out_of_bound_wall() {
        let script = aj(format!(
            "
            function onTurn()
                return \"100,100,100,100\"
            end
        "
        ));
        _run_core_test(script.clone(), script, |game_state| {
            game_state
                == GameResult::Error(ErrorType::RuntimeError {
                    reason: String::from("Invalid input: 100,100,100,100"),
                    fault: Some(PlayerType::Flipped),
                })
        });
    }

    #[test]
    /// Over-use walls
    ///
    /// This function will try to use more than 10 walls
    fn overuse_walls() {
        let script = aj(format!(
            "
            round = -1
            y = -1
            function onTurn()  
                alternate = round % 2
                round = round + 1
                if round % 2 == 0 then
                    y = y + 1
                    return 0 .. \",\" .. y .. \",\" .. 1 .. \",\" .. y
                else
                    return 2 .. \",\" .. y .. \",\" .. 3 .. \",\" .. y
                end
            end
            "
        ));

        let p2_script = aj(format!(
            "
            round = -1
            function onTurn()
                round = round + 1
                if round % 2 == 0 then
                    return \"0\"
                end
                return \"2\"
            end
            "
        ));

        _run_core_test(script, p2_script, |game_state| match game_state {
            GameResult::Error(ErrorType::GameError { reason, fault }) => {
                reason.contains("walls") && fault.is_some() && fault.unwrap() == PlayerType::Flipped
            }
            _ => false,
        });
    }

    #[test]
    /// Walk out of bounds
    ///
    /// This script will try to walk out of bounds
    fn walk_out_of_bounds_bottom() {
        let script = aj(format!(
            "
            function onTurn()
                return \"2\"
            end
            "
        ));
        _run_core_test(script.clone(), script, |game_state| match game_state {
            GameResult::Error(ErrorType::GameError { reason, fault }) => {
                reason.contains("out of bounds")
                    && reason.contains(&MAP_SIZE.to_string())
                    && fault.is_some()
                    && fault.unwrap() == PlayerType::Flipped
            }
            _ => false,
        });
        let script = aj(format!(
            "
            function onTurn()
                return \"1\"
            end
            "
        ));
        _run_core_test(script.clone(), script, |game_state| match game_state {
            GameResult::Error(ErrorType::GameError { reason, fault }) => {
                reason.contains("out of bounds")
                    && reason.contains(&MAP_SIZE.to_string())
                    && fault.is_some()
                    && fault.unwrap() == PlayerType::Flipped
            }
            _ => false,
        });
        let script = aj(format!(
            "
            function onTurn()
                return \"3\"
            end
            "
        ));
        _run_core_test(script.clone(), script, |game_state| match game_state {
            GameResult::Error(ErrorType::GameError { reason, fault }) => {
                reason.contains("out of bounds")
                    && reason.contains("-1")
                    && fault.is_some()
                    && fault.unwrap() == PlayerType::Flipped
            }
            _ => false,
        });
    }

    #[test]
    /// Opposite tiles winnable
    ///
    /// This test checks that all tiles
    /// are winnable. It check that both
    /// player 1 and player 2 can win on
    /// all tiles.
    fn opposite_tiles_winnable() {
        for x in 0..MAP_SIZE {
            let get_script = |x: i32, stal: bool| {
                aj(format!(
                    "
                    stal = {}
                    round = -1
                    if stal then
                        round = -1
                    end
                    function onTurn()
                        -- We want to go to the far left
                        -- from there we can iterate 
                        -- though all x positions
                        round = round + 1

                        if round <= 3 then
                            return \"3\"
                        end

                        -- Stal section
                        if stal then
                            if round % 2 == 0 then
                                return \"0\"
                            end
                            return \"2\"
                        end
                        
                        if round > {} then
                            return \"0\"
                        end
                        return \"1\"
                    end
                ",
                    if stal { "true" } else { "false" },
                    x
                ))
            };

            // Second player must be offset by one otherwise
            // they will collide when they are both trying
            // x = 4
            let second_player_x = if x == 9 { 0 } else { x + 1 };

            _run_core_test(
                get_script(x, false),
                get_script(second_player_x, false),
                |game_state| game_state == GameResult::PlayerOneWon,
            );
            _run_core_test(
                get_script(x, true),
                get_script(second_player_x, false),
                |game_state| game_state == GameResult::PlayerTwoWon,
            );
        }
    }

    #[test]
    /// Place wall on player
    ///
    /// Create a bot that will attempt to
    /// place a wall on top of another player.
    fn place_wall_on_player() {
        let wall_script = aj(format!(
            "
            function onTurn()
                return \"4,0,4,1\"
            end
            "
        ));
        let script = aj(format!(
            "
            function onTurn()
                return \"0\"
            end
            "
        ));

        _run_core_test(wall_script, script.clone(), |game_state| match game_state {
            GameResult::Error(ErrorType::GameError {
                reason: _,
                fault: __,
            }) => true,
            _ => false,
        });

        let wall_script = aj(format!(
            "
            function onTurn()
                return \"4,8,5,8\"
            end
            "
        ));

        _run_core_test(wall_script, script, |game_state| match game_state {
            GameResult::Error(ErrorType::GameError {
                reason: _,
                fault: __,
            }) => true,
            _ => false,
        });
    }

    #[test]
    /// More
    ///
    ///
    fn dodger_bot() {
        let script = load_script("trivial_dodger");

        let forward = format!(
            "
                round = 0
                function onTurn()
                    round = round + 1
                    if round % 2 == 1 then
                        return \"0\"
                    end
                    return \"2\"
                end
                function onJump()
                    return \"0\"
                end
            "
        );

        _run_test_with_custom_game_session(
            script,
            forward,
            &mut methods::custom_new(
                Player {
                    player_type: PlayerType::Flipped,
                    x: 4,
                    y: 8,
                    wall_count: 5,
                },
                Player {
                    player_type: PlayerType::Regular,
                    x: 4,
                    y: 0,
                    wall_count: 5,
                },
                vec![],
                load_std(),
            ),
            |game_state| game_state == GameResult::PlayerOneWon,
        );
    }
}
