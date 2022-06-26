#[cfg(test)]
mod tests {
    use crate::game::{
        game::{ErrorType, GameState},
        methods,
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

        let mut game_session = methods::new(String::new());
        match methods::start(&mut game_session, script.to_string(), String::new()) {
            GameState::PlayerOneWon => panic!("Expected game to fail"),
            GameState::PlayerTwoWon => panic!("Expected game to fail"),
            GameState::Error(ErrorType::RuntimeError { reason }) => {
                assert!(reason.contains("onTurn"));
            }
            GameState::Error(ErrorType::GameError { reason }) => panic!("Game error: {}", reason),
            GameState::Error(ErrorType::TurnTimeout) => panic!("Expected game to fail"),
            GameState::Error(ErrorType::GameDeadlock) => panic!("Expected game to fail"),
            _ => panic!("Why is game still running?"),
        }
    }

    #[test]
    /// Player collides
    ///
    /// This test makes sure that the game
    /// crashes if both players continue
    /// straight forward.
    fn fail_on_just_go_forward() {
        let script = format!(
            "
			function onTurn()
				return \"0\"
			end
		"
        );

        let mut game_session = methods::new(String::new());
        match methods::start(&mut game_session, script.clone(), script) {
            GameState::PlayerOneWon => panic!("Player one won"),
            GameState::PlayerTwoWon => panic!("Player two won"),
            GameState::Error(ErrorType::RuntimeError { reason }) => {
                panic!("RuntimeError: {}", reason)
            }
            GameState::Error(ErrorType::GameError { reason }) => {
                assert_eq!(reason, "Invalid move: Tile (4,4) is occupied")
            }
            GameState::Error(ErrorType::TurnTimeout) => panic!("Expected game error"),
            GameState::Error(ErrorType::GameDeadlock) => panic!("Expected game error"),
            _ => panic!("Why is game still running?"),
        }
    }

    #[test]
    /// Forward with clear path
    ///
    /// This script moves both player to the left
    /// and the moves the forward, clearning the
    /// path for both players.
    fn success_when_left_and_forward() {
        let script = format!(
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
        );

        let mut game_sesion = methods::new(String::new());
        match methods::start(&mut game_sesion, script.clone(), script) {
            GameState::PlayerOneWon => assert_eq!(true, true),
            GameState::PlayerTwoWon => panic!("Player two won"),
            GameState::Error(ErrorType::RuntimeError { reason }) => {
                panic!("RuntimeError: {}", reason)
            }
            GameState::Error(ErrorType::GameError { reason }) => panic!("Game error: {}", reason),
            GameState::Error(ErrorType::TurnTimeout) => panic!("Expected player 1 to win"),
            GameState::Error(ErrorType::GameDeadlock) => panic!("Expected player 1 to win"),
            _ => panic!("Why is game still running?"),
        };
    }

    #[test]
    fn place_invalid_wall() {
        let script = format!(
            "
            function onTurn()
                -- This is clearly an invalid wall since
                -- the two coordinates are not connected
                return \"0,4,8,8\"
            end
            "
        );

        let mut game_session = methods::new(String::new());
        match methods::start(&mut game_session, script.clone(), script) {
            GameState::PlayerOneWon => panic!("Expected game to fail"),
            GameState::PlayerTwoWon => panic!("Expected game to fail"),
            GameState::Error(ErrorType::RuntimeError { reason }) => {
                panic!("RuntimeError: {}", reason)
            }
            GameState::Error(ErrorType::GameError { reason }) => {
                assert_eq!(
                    reason,
                    "Invalid wall format, a wall must consist of two adjacent coordinates: ((0,4), (8,8))"
                )
            }
            GameState::Error(ErrorType::TurnTimeout) => panic!("Expected game to fail"),
            GameState::Error(ErrorType::GameDeadlock) => assert!(true),
            _ => panic!("Why is game still running?"),
        };
    }

    #[test]
    /// Complete block
    ///
    /// Test fails when player makes it impossible
    /// for opponent to win
    fn fail_on_complete_block() {
        // Should fail on 4th round, since this will block
        // both players from finishing.
        let p1_script = format!(
            "
			round = -1
            y = 1
			function onTurn()  

                round = round + 1

                if round > 4 then
                    return \"0\"
                end

                if round == 4 then
                    return \"8,7,8,8\"
                end

				x = round * 2
				return x .. \",4,\" .. (x + 1) .. \",4\"
			end
		"
        );

        // This script will go back and forth
        let p2_script = format!(
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
        );

        let mut game_session = methods::new(String::new());
        match methods::start(&mut game_session, p1_script, p2_script) {
            GameState::PlayerOneWon => panic!("Player one won"),
            GameState::PlayerTwoWon => panic!("Player two won"),
            GameState::Error(ErrorType::RuntimeError { reason }) => {
                panic!("RuntimeError: {}", reason)
            }
            GameState::Error(ErrorType::GameError { reason }) => {
                assert_eq!(reason, "No path for either bot available")
            }
            GameState::Error(ErrorType::TurnTimeout) => panic!("Expected game error"),
            GameState::Error(ErrorType::GameDeadlock) => panic!("Expected game error"),
            _ => panic!("Why is game still running?"),
        };
    }
}
