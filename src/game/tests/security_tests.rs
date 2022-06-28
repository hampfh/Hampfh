#[cfg(test)]
mod tests {
    use crate::game::{
        game::{ErrorType, GameResult},
        player::PlayerType,
        tests::util::_run_core_test,
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

        _run_core_test(script.clone(), script, |state| {
            state
                == GameResult::Error(ErrorType::TurnTimeout {
                    fault: Some(PlayerType::Flipped),
                })
        });
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

        _run_core_test(script.clone(), script, |state| {
            state
                == GameResult::Error(ErrorType::TurnTimeout {
                    fault: Some(PlayerType::Flipped),
                })
        });
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

        _run_core_test(script.clone(), script, |state| {
            state == GameResult::Error(ErrorType::GameDeadlock)
        });
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

        _run_core_test(script.clone(), script, |state| {
            std::mem::discriminant(&state)
                == std::mem::discriminant(&GameResult::Error(ErrorType::RuntimeError {
                    reason: String::new(),
                    fault: Some(PlayerType::Flipped),
                }))
        });
    }
}
