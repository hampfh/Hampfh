use crate::game::{
    game::{ErrorType, GameState},
    methods,
};

pub fn _run_core_test(script: String, script2: String, is_equal: fn(GameState) -> bool) {
    let mut game_session = methods::new(String::new());
    let (game_state_result, _) = methods::start(&mut game_session, script, script2);
    println!("Result from run: {:?}", game_state_result.clone());

    if !is_equal(game_state_result.clone()) {
        _capture_test_fail(game_state_result);
    }
}

fn _capture_test_fail(game_state: GameState) {
    match game_state {
        GameState::PlayerOneWon => panic!("Player 1 won"),
        GameState::PlayerTwoWon => panic!("Player 2 won"),
        GameState::Error(ErrorType::RuntimeError { reason, fault }) => {
            panic!("RuntimeError: {}, fault: [{:?}]", reason, fault)
        }
        GameState::Error(ErrorType::GameError { reason, fault }) => {
            panic!("Game error: {}, fault: [{:?}]", reason, fault)
        }
        GameState::Error(ErrorType::TurnTimeout { fault }) => {
            panic!("Turn timeout error, fault: [{:?}]", fault)
        }
        GameState::Error(ErrorType::GameDeadlock) => panic!("Expected game error"),
        _ => panic!("Why is game still running?"),
    };
}
