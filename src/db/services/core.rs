use crate::code_unwrapper::unwrap_code;
use crate::db::create_issue_comment::create_issue_comment;
use crate::db::db::DbPool;
use crate::db::models::match_model::Match;
use crate::db::models::submission_model::Submission;
use crate::db::models::turn_model::Turn;
use crate::db::models::user_model::User;
use crate::db::services::match_maker::match_make;
use crate::db::services::webhook_schema::GithubPayload;
use crate::readme_factory::generate_readme;
use crate::readme_factory::write_file;
use crate::repo_updater::update_repo;
use actix_web::{post, web};

#[post("/api/challenge")]
#[allow(unreachable_code)]
pub async fn submit_challenge(
    webhook_post: web::Json<GithubPayload>,
    pool: web::Data<DbPool>,
) -> actix_web::Result<String> {
    let conn = pool.get().unwrap();

    // Validate the the submission is a challenger submission
    if webhook_post.action != "opened" || webhook_post.issue.title != "[Challenger-submission]" {
        return Ok(format!("Only accepts \"opened\" actions"));
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
            return Ok(format!("Internal error"));
        }
    }

    // Get lua code from issue body
    let code = match unwrap_code(&webhook_post.issue.body) {
        Ok(code) => code,
        Err(e) => {
            create_issue_comment(webhook_post.issue.number, &e);
            return Ok(format!("{}", e));
        }
    };

    // Create submission
    let challenger = match Submission::create(
        &user.unwrap().id,
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
        return Ok(format!("Could not create submission...<br>Try again later"));
    }

    create_issue_comment(webhook_post.issue.number, &format!("User: {}<br>Script-id: {}<br>Thanks for submitting!<br>Your code is being processed...", webhook_post.sender.login, challenger.as_ref().unwrap().id));

    let reports = match_make(&challenger.clone().unwrap(), &conn);

    let mut output = String::new();
    for report in reports {
        output += &report;
        output += "<br>";
        create_issue_comment(webhook_post.issue.number, &output);
    }

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
            // Submit new files to repo
            update_repo(&challenger.unwrap().id, &webhook_post.sender.login);
            return Ok("README.md updated".to_string());
        }
        Err(e) => {
            create_issue_comment(
                webhook_post.issue.number,
                &format!("Internal error: Could not update README.md: {}", e),
            );
            return Ok("Could not update README.md".to_string());
        }
    }
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(submit_challenge);
}
