use crate::{
    external_related::readme_factory::{get_match_from_tiles, write_file},
    game::{
        game::{ErrorType, Game, GameResult},
        methods,
    },
};

pub(crate) fn _run_core_test(script: String, script2: String, is_equal: fn(GameResult) -> bool) {
    let mut game_session = methods::new(String::new());
    _run_test_with_custom_game_session(script, script2, &mut game_session, is_equal);
}

pub(crate) fn _run_test_with_custom_game_session(
    script: String,
    script2: String,
    session: &mut Game,
    is_equal: fn(GameResult) -> bool,
) {
    let (game_state_result, mut turns) = methods::start(session, script, script2);

    turns.reverse();
    write_file("test_dump.temp.md", get_match_from_tiles(turns)).unwrap();

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

/// Append turn to code
pub(super) fn at(c: String) -> String {
    let mut output = c.clone();
    output.push_str(format!("\nfunction onTurn()\n    return \"0\"\nend").as_str());
    return output;
}
/// Append jump to code
pub(super) fn aj(c: String) -> String {
    let mut output = c.clone();
    output.push_str(format!("\nfunction onJump()\n    return \"0\"\nend").as_str());
    return output;
}
