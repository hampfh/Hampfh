use mlua::Lua;

use crate::{
    external_related::readme_factory::{get_match_from_tiles, write_file},
    game::{
        game_state::{ErrorType, Game, GameConfig, GameResult},
        player::{Player, PlayerType},
    },
};

pub(crate) fn _run_core_test(
    script: String,
    script2: String,
    is_equal: fn(GameResult, bool) -> bool,
    swap_scripts: bool,
) {
    let mut game_session = Game::new(GameConfig::new());
    _run_test_with_custom_game_session(
        script.clone(),
        script2.clone(),
        &mut game_session,
        is_equal,
        false,
    );
    let mut game_session = Game::new(GameConfig::new());
    if swap_scripts {
        _run_test_with_custom_game_session(
            script2.clone(),
            script.clone(),
            &mut game_session,
            is_equal,
            true,
        );
    }
}

pub(crate) fn _run_test_with_custom_game_session(
    script: String,
    script2: String,
    session: &mut Game,
    is_equal: fn(GameResult, is_swapped: bool) -> bool,
    is_swapped: bool,
) {
    let (game_state_result, mut turns, _) = Game::start(session, script.clone(), script2.clone());

    turns.reverse();
    write_file("test_dump.temp.md", get_match_from_tiles(turns)).unwrap();

    if !is_equal(game_state_result.clone(), is_swapped) {
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

#[allow(dead_code)]
/// Append turn to code
pub(super) fn at(c: String) -> String {
    let mut output = c.clone();
    output.push_str(format!("\nfunction onTurn()\n    return \"0\"\nend").as_str());
    return output;
}
#[allow(dead_code)]
/// Append jump to code
pub(super) fn aj(c: String) -> String {
    let mut output = c.clone();
    output.push_str(format!("\nfunction onJump()\n    return \"0\"\nend").as_str());
    return output;
}

#[allow(dead_code)]
pub(super) fn mock_player(x: i32, y: i32, wall_count: i32, player_type: PlayerType) -> Player {
    return Player {
        x,
        y,
        wall_count,
        player_type,
    };
}

#[allow(dead_code)]
pub(super) fn load_std() -> String {
    return load_script("std");
}

#[allow(dead_code)]
pub(super) fn test_std(scripts: Vec<String>, asserts: fn(ctx: mlua::Lua) -> Result<(), ()>) {
    let sandbox = Lua::new();
    sandbox.load(&load_std()).exec().unwrap();
    for script in scripts {
        sandbox.load(&script).exec().unwrap();
    }
    asserts(sandbox).unwrap();
}

#[allow(dead_code)]
pub(super) fn test_std_conditional(scripts: Vec<(String, bool)>, game_context: Option<String>) {
    let sandbox = Lua::new();
    sandbox.load(&load_std()).exec().unwrap();
    for (script, expected_result) in scripts {
        let var = convert_uuid_to_variable(uuid::Uuid::new_v4().to_string());
        sandbox
            .load(&script.replace("[]", &format!("{} = ", var)).replace(
                "[c]",
                &if game_context.is_some() {
                    format!("{}", game_context.as_ref().unwrap())
                } else {
                    String::new()
                },
            ))
            .exec()
            .unwrap();
        assert_eq!(
            sandbox.globals().get::<_, bool>(var).unwrap(),
            expected_result
        );
    }
}

#[allow(dead_code)]
fn convert_uuid_to_variable(uuid: String) -> String {
    let mut uuid = uuid;
    uuid.insert(0, '_');
    return uuid.split("-").collect::<Vec<&str>>().join("_");
}

pub(super) fn load_script(filename: &str) -> String {
    std::fs::read_to_string(format!("{}{}.lua", "./scripts/", filename))
        .expect("Could not load script")
}
