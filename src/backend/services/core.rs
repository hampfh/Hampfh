use crate::backend::db::DbPool;
use crate::backend::models::submission_model::Submission;
use crate::backend::models::user_model::User;
use crate::external_related::code_unwrapper::unwrap_code;
use crate::external_related::github::close_issue::{close_issue, CloseType};
use crate::external_related::github::create_issue_comment::create_issue_comment;
use crate::external_related::github::webhook_schema::{GithubPayload, Label};
use crate::match_maker::match_executor::MatchReport;
use crate::match_maker::placements::run_placements;
use crate::match_maker::regenerate_markdown_files::regen_markdown_files_and_update_repo;
use actix_web::{post, web};

#[post("/api/challenge")]
#[allow(unreachable_code)]
pub async fn submit_challenge(
    webhook_post: web::Json<GithubPayload>,
    db_pool: web::Data<DbPool>,
) -> actix_web::Result<String> {
    let conn = db_pool.get().unwrap();
    // Validate the the submission is a challenger submission
    if !valid_request(&webhook_post.action, &webhook_post.issue.labels) {
        return Ok(format!(
            "Only accepts \"opened\" actions and must be marked with the \"challenger\" label"
        ));
    }

    // If user doesn't exist we create it
    let mut user = User::by_username(&webhook_post.sender.login, &conn);
    if user.is_none() {
        // Create user
        user = User::create(&webhook_post.sender.login, &conn);

        if user.is_none() {
            println!("Error: Could not create user");
            create_issue_comment(
                webhook_post.issue.number,
                "Internal error, please try again later",
            );
            close_issue(CloseType::NotPlanned, webhook_post.issue.number);
            return Ok(format!("Internal error"));
        }
    }

    // Get lua code from issue body
    let code = match unwrap_code(&webhook_post.issue.body) {
        Ok(code) => code,
        Err(e) => {
            create_issue_comment(webhook_post.issue.number, &e);
            close_issue(CloseType::NotPlanned, webhook_post.issue.number);
            return Ok(format!("{}", e));
        }
    };

    // Create submission
    let challenger = match Submission::create(
        &user.as_ref().unwrap().id,
        &code,
        Some(&webhook_post.issue.title),
        0,
        &webhook_post.issue.html_url,
        webhook_post.issue.number,
        &conn,
    ) {
        Ok(submission) => submission,
        Err(_) => {
            create_issue_comment(
                webhook_post.issue.number,
                &format!("This submission has already been submitted before"),
            );
            close_issue(CloseType::NotPlanned, webhook_post.issue.number);
            return Ok(format!(
                "{}",
                "This submission has already been submitted before"
            ));
        }
    };

    if challenger.is_none() {
        create_issue_comment(
            webhook_post.issue.number,
            "Error: Internal error, could not create submission...<br>Try again later",
        );
        close_issue(CloseType::NotPlanned, webhook_post.issue.number);
        return Ok(format!("Could not create submission...<br>Try again later"));
    }

    create_issue_comment(webhook_post.issue.number, &format!("User: {}<br>Script-id: {}<br>Thanks for submitting!<br>Your code is being processed...", webhook_post.sender.login, challenger.as_ref().unwrap().id));

    let reports = run_placements(&challenger.clone().unwrap(), &conn);

    let mut output = String::new();
    let mut opponent_output: Vec<&MatchReport> = Vec::new();
    for (challenger_report, opponent_report) in reports.iter() {
        output += &challenger_report.report;
        output += "<br>";
        opponent_output.push(opponent_report);
    }
    if reports.len() == 0 {
        create_issue_comment(webhook_post.issue.number, &format!("Bot has been registered but could not be match-maked against another bot, wait for someone else to create a bot..."));
    } else {
        create_issue_comment(webhook_post.issue.number, &output);
    }

    // Notify opponents too
    for opponent in opponent_output.iter() {
        create_issue_comment(opponent.issue_number, &opponent.report);
    }

    close_issue(CloseType::Completed, webhook_post.issue.number);

    match regen_markdown_files_and_update_repo(&conn) {
        Ok(msg) | Err(msg) => Ok(msg),
    }
}

fn valid_request(action: &String, labels: &Vec<Label>) -> bool {
    return action == "opened" && labels.iter().any(|current| current.name == "challenger");
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(submit_challenge);
}
