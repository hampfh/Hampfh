use diesel::SqliteConnection;

use crate::{
    backend::models::submission_model::Submission,
    external_related::{
        github::create_issue_comment::create_issue_comment, repo_updater::update_repo,
    },
};

use super::{
    match_executor::{execute_match_queue, MatchReport},
    match_make::create_match_making_queue,
    regenerate_markdown_files::regen_markdown_files,
};

pub(crate) fn run_scheduled_matchmaking(conn: &SqliteConnection) {
    let submissions = Submission::list(conn);
    let match_queue = create_match_making_queue(submissions);
    let match_reports = execute_match_queue(conn, match_queue);
    publish_match_reports(match_reports);
    match regen_markdown_files(conn) {
        Ok(_) => (),
        Err(error) => println!("Could not update README.md: {}", error),
    }
    update_repo(format!(
        "Match Making: {}",
        chrono::Local::now().format("%Y-%m-%d %H:%M:%S")
    ));
}

fn publish_match_reports(match_reports: Vec<(MatchReport, MatchReport)>) {
    for (report1, report2) in match_reports {
        create_issue_comment(report1.issue_number, &report1.report);
        create_issue_comment(report2.issue_number, &report2.report);
    }
}
