use crate::game::{
    game::{ErrorType, GameResult},
    methods,
};

pub fn _run_core_test(script: String, script2: String, is_equal: fn(GameResult) -> bool) {
    let mut game_session = methods::new(String::new());
    let (game_state_result, _) = methods::start(&mut game_session, script, script2);
    println!("Result from run: {:?}", game_state_result.clone());

    if !is_equal(game_state_result.clone()) {
        _capture_test_fail(game_state_result);
    }
}

fn _capture_test_fail(game_state: GameResult) {
    match game_state {
        GameResult::PlayerOneWon => panic!("Player 1 won"),
        GameResult::PlayerTwoWon => panic!("Player 2 won"),
        GameResult::Error(ErrorType::RuntimeError { reason, fault }) => {
            panic!("RuntimeError: {}, fault: [{:?}]", reason, fault)
        }
        GameResult::Error(ErrorType::GameError { reason, fault }) => {
            panic!("Game error: {}, fault: [{:?}]", reason, fault)
        }
        GameResult::Error(ErrorType::TurnTimeout { fault }) => {
            panic!("Turn timeout error, fault: [{:?}]", fault)
        }
        GameResult::Error(ErrorType::GameDeadlock) => panic!("Expected game error"),
    };
}
