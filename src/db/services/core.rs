use crate::db::create_issue_comment::create_issue_comment;
use crate::db::db::DbPool;
use crate::db::models::submission_model::Submission;
use crate::db::models::user_model::User;
use crate::db::services::match_maker::match_make;
use crate::db::services::webhook_schema::GithubPayload;
use actix_web::{post, Error};
use actix_web::{web};

#[post("/api/challenge")]
pub async fn submit_challenge(
	webhook_post: web::Json<GithubPayload>,
	pool: web::Data<DbPool>,
) -> actix_web::Result<String> {
	let conn = pool.get().unwrap();

	if webhook_post.action != "opened" {
		return Ok(format!("Only accepts \"opened\" actions"));
	}

	let mut user = User::by_id(&webhook_post.sender.login, &conn);
	if user.is_none() {
		// Create user
		user = User::create(&webhook_post.sender.login, &conn);
	}

	if user.is_none() {
		create_issue_comment(webhook_post.issue.number, "Internal error");
		return Ok(format!("Internal error"));
	}

	// Create submission
	let challenger = Submission::create(
		&user.unwrap().id,
		&webhook_post.issue.body,
		Some(&webhook_post.issue.title),
		0,
		&webhook_post.issue.html_url,
		webhook_post.issue.number,
		&conn,
	);

	if challenger.is_none() {
		create_issue_comment(
			webhook_post.issue.number,
			"Could not create submission...<br>Try again later",
		);
		return Ok(format!("Could not create submission...<br>Try again later"));
	}

	create_issue_comment(webhook_post.issue.number, &format!("User: {}<br>Script-id: {}<br>Thanks for submitting!<br>Your code is being processed...", webhook_post.sender.login, challenger.as_ref().unwrap().id));

	let errors = match_make(&challenger.unwrap(), &conn);

	let mut output = String::new();
	if errors.len() > 0 {
		for error in errors {
			output += &error;
			output += "<br>";
		}
		create_issue_comment(webhook_post.issue.number, &output);
	} else {
		create_issue_comment(webhook_post.issue.number, "Matches performed!");
	}

	Ok(format!("Success"))
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(submit_challenge);
}