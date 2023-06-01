use std::{fs, process};

use crate::{
    backend::{
        self,
        models::{
            match_model::Match, submission_model::Submission, turn_model::Turn, user_model::User,
        },
    },
    external_related::readme_factory::{
        build_match_files_wrapper, build_match_log_wrapper, build_submission_log_wrapper,
        clear_match_dir, create_and_encode_file, generate_gif_from_turn, generate_readme,
        get_match_from_tiles_compact, write_file,
    },
    game::{
        entry_point::initialize_game_session,
        game::{ErrorType, GameResult},
        player::PlayerType,
    },
    match_maker::scheduler::run_scheduled_matchmaking,
};

pub fn cli(args: Vec<String>) {
    match args[1].as_str() {
        "match" => {
            if (args.len() - 1) < 3 {
                println!("match command requries 2 arguments: script1_path script2_path");
                process::exit(1);
            }
            run_local_match(args[2].as_str(), args[3].as_str())
        }
        "generate" => {
            clear_match_dir();
            generate_main();
            build_match_files_wrapper();
            build_match_log_wrapper();
            build_submission_log_wrapper();
        }
        "generate-main" => generate_main(),
        "generate-matches" => build_match_files_wrapper(),
        "generate-logs" => {
            build_match_log_wrapper();
            build_submission_log_wrapper();
        }
        "clear" => clear_match_dir(),
        "schedule_matchmaking" => scheduled_matchmaking(),
        _ => {}
    }
}

fn generate_main() {
    let conn = backend::db::establish_connection().get().unwrap();
    write_file(
        "../../README.md",
        generate_readme(
            User::list(&conn),
            Submission::list(&conn),
            Match::list(&conn),
            Turn::list(&conn),
        ),
    )
    .unwrap();
}

fn scheduled_matchmaking() {
    let db_connection = backend::db::establish_connection();
    let conn = match db_connection.get() {
        Ok(conn) => conn,
        Err(error) => {
            println!("Could not establish connection to database: {}", error);
            process::exit(1);
        }
    };

    run_scheduled_matchmaking(&conn);
    process::exit(0);
}

fn run_local_match(script1_path: &str, script2_path: &str) {
    let script1 = std::fs::read_to_string(script1_path).expect("Could not load script 1");
    let script2 = std::fs::read_to_string(script2_path).expect("Could not load script 2");

    let (results, turns) = initialize_game_session(&script1, &script2);
    let mut file: String = "".to_string();
    file.push_str(&format!(
        "<div align=\"center\"><p>{}</p></div>\n\n",
        match results {
            GameResult::PlayerOneWon => "Script 1 (🟩) won",
            GameResult::PlayerTwoWon => "Script 2 (🟥) won",
            _ => "",
        }
    ));

    if let GameResult::Error(error) = results.clone() {
        let string: String;
        file.push_str(&format!(
            "<div align=\"center\"><p>--- Match has errors ---</p>\n\n{}</div>",
            match error {
                ErrorType::GameDeadlock => "Reason for error: Game Deadlock",
                ErrorType::GameError { reason, fault } => {
                    // TODO This feels very hacky, there should be another solution for this
                    string = print_error(Some(reason), fault);
                    &string
                }
                ErrorType::RuntimeError { reason, fault } => {
                    string = print_error(Some(reason), fault);
                    &string
                }
                ErrorType::TurnTimeout { fault } => {
                    string = print_error(None, fault);
                    &string
                }
            }
        ))
    }
    file.push_str(&get_match_from_tiles_compact(turns.clone()));

    fs::write("match.temp.md", file).expect("Could not write match file");
    let (color_palette, images) = generate_gif_from_turn(turns, Some(results), 50);
    create_and_encode_file("match.temp.gif".to_string(), images, &color_palette, 50);
}

fn print_error(reason: Option<String>, fault: Option<PlayerType>) -> String {
    format!(
        "Reason for error: {}\n\nFault: {}",
        &reason.unwrap_or("Unknown".to_string()),
        match fault {
            Some(PlayerType::Flipped) => "Player 1 (🟩)",
            Some(PlayerType::Regular) => "Player 2 (🟥)",
            _ => "Unknown",
        }
    )
}
