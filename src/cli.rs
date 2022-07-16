use std::process;

use crate::{
    backend::{
        self,
        models::{
            match_model::Match, submission_model::Submission, turn_model::Turn, user_model::User,
        },
    },
    external_related::readme_factory::{
        build_match_files_wrapper, clear_match_dir, generate_readme, write_file,
    },
    match_maker::scheduler::run_scheduled_matchmaking,
};

pub fn cli(args: Vec<String>) {
    match args[1].as_str() {
        "generate-main" => generate_main(),
        "generate-matches" => build_match_files_wrapper(),
        "clear" => clear_match_dir(),
        "scheduled_matchmaking" => scheduled_matchmaking(),
        _ => {}
    }
}

fn generate_main() {
    let conn = backend::db::establish_connection().get().unwrap();
    write_file(
        "README.md",
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
