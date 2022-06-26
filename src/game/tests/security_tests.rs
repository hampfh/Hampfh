#[cfg(test)]
mod tests {
    use crate::game::{
        game::{ErrorType, GameState},
        methods,
    };

    /**
     * This file contains tests that are built
     * to try to break the runtime.
     */

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

    #[test]
    /// Go back and fourth
    ///
    /// This test creates two tests where
    /// the bots make no progress, they
    /// both just go back and forth.
    fn back_and_fourth() {
        let script = format!(
            "
                round = 0
                function onTurn()
                    round = round + 1
                    if round % 2 == 1 then
                        return \"0\"
                    end
                    return \"2\"
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
            GameState::Error(ErrorType::TurnTimeout) => panic!("Turn timeout"),
            GameState::Error(ErrorType::GameDeadlock) => assert!(true),
            _ => panic!("Why is game still running?"),
        }
    }

    #[test]
    /// Invalid syntax
    ///
    /// Here is a program written in
    /// python that should be gracfully
    /// returned by the program.
    fn invalid_syntax() {
        let script = format!(
            "
                test = 0
                def onTurn():
                    return \"0\"
            "
        );

        let mut game_session = methods::new(String::new());
        match methods::start(&mut game_session, script.clone(), script) {
            GameState::PlayerOneWon => panic!("Player one won"),
            GameState::PlayerTwoWon => panic!("Player two won"),
            GameState::Error(ErrorType::RuntimeError { reason: _ }) => {
                assert!(true)
            }
            GameState::Error(ErrorType::GameError { reason }) => {
                panic!("Game error: {}", reason)
            }
            GameState::Error(ErrorType::TurnTimeout) => panic!("Turn timeout"),
            GameState::Error(ErrorType::GameDeadlock) => panic!("GameDeadlock"),
            _ => panic!("Why is game still running?"),
        }
    }
}
