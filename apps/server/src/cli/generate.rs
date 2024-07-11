use clap::ArgMatches;

use crate::{
    backend::{
        self,
        models::{
            match_model::Match, submission_model::Submission, turn_model::Turn, user_model::User,
        },
    },
    external_related::readme_factory::{
        build_match_files_wrapper, build_match_log_wrapper, build_submission_log_wrapper,
        clear_match_dir, generate_readme, write_file,
    },
};

pub(crate) fn generate(matches: &ArgMatches) {
    let primary_flag = *matches.get_one::<bool>("primary").unwrap();
    let matches_flag = *matches.get_one::<bool>("matches").unwrap();
    let logs_flag = *matches.get_one::<bool>("logs").unwrap();
    let clear_flag = *matches.get_one::<bool>("clear").unwrap();

    let no_flags = !primary_flag && !matches_flag && !logs_flag && !clear_flag;

    if clear_flag || no_flags {
        clear_match_dir();
    }
    if primary_flag || no_flags {
        generate_main();
    }
    if matches_flag || no_flags {
        build_match_files_wrapper();
    }
    if logs_flag || no_flags {
        build_match_log_wrapper();
        build_submission_log_wrapper();
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
