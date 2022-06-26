#[cfg(test)]
mod tests {
    use crate::game::{
        game::{ErrorType, GameState},
        methods,
    };

    #[test]
    /// Infinity startup script
    ///
    /// This test tries to run an infinity
    /// loop in the startup, aka outside the
    /// "onTurn" function
    fn infinity_loop() {
        let script = format!(
            "
				while true do		
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
                panic!("Game error: {}", reason)
            }
            GameState::Error(ErrorType::TurnTimeout) => assert!(true),
            GameState::Error(ErrorType::GameDeadlock) => panic!("Expected game error"),
            _ => panic!("Why is game still running?"),
        }
    }

    #[test]
    /// Infinity loop in onTurn
    ///
    /// This test tries to loop
    /// for inifinity in the onTurn function.
    fn infinity_loop_on_turn() {
        let script = format!(
            "
				function onTurn()
                    while true do
                    end
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
                panic!("Game error: {}", reason)
            }
            GameState::Error(ErrorType::TurnTimeout) => assert!(true),
            GameState::Error(ErrorType::GameDeadlock) => panic!("Expected game error"),
            _ => panic!("Why is game still running?"),
        }
    }
}
