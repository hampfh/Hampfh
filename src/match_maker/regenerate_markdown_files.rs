use diesel::SqliteConnection;

use crate::{
    backend::models::{
        match_model::Match, submission_model::Submission, turn_model::Turn, user_model::User,
    },
    external_related::{
        readme_factory::{
            build_match_files_wrapper, build_match_log_wrapper, build_submission_log_wrapper,
            clear_match_dir, generate_readme, write_file,
        },
        repo_updater,
    },
};

pub(crate) fn regen_markdown_files(conn: &SqliteConnection) -> Result<String, String> {
    clear_match_dir();
    build_match_files_wrapper();
    build_match_log_wrapper();
    build_submission_log_wrapper();
    match write_file(
        "README.md",
        generate_readme(
            User::list(&conn),
            Submission::list(&conn),
            Match::list(&conn),
            Turn::list(&conn),
        ),
    ) {
        Ok(_) => {
            repo_updater::update_repo(format!(
                "Match Maker update {}",
                chrono::Local::now().format("%Y-%m-%d %H:%M:%S")
            ));
            Ok(format!("Successfully updated repository"))
        }
        Err(error) => Err(format!("Could not update README.md: {}", error)),
    }
}
