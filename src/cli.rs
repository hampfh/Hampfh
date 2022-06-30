use crate::{
    db::{
        self,
        models::{
            match_model::Match, submission_model::Submission, turn_model::Turn, user_model::User,
        },
    },
    readme_factory::{build_match_files_wrapper, clear_match_dir, generate_readme, write_file},
};

pub fn cli(args: Vec<String>) {
    match args[1].as_str() {
        "generate-main" => generate_main(),
        "generate-matches" => build_match_files_wrapper(),
        "clear" => clear_match_dir(),
        _ => {}
    }
}

fn generate_main() {
    let conn = db::db::establish_connection().get().unwrap();
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
