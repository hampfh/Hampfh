use crate::db::create_issue_comment::create_issue_comment;
use crate::db::models::submission_model::Submission;
use crate::db::models::user_model::User;
use crate::db::services::webhook_schema::GithubPayload;
use crate::db::db::DbPool;
use actix_web::{HttpResponse, web};
use actix_web::{post, Error};

#[post("/api/challenge")]
pub async fn submit_challenge(
	pool: web::Data<DbPool>,
	webhook_post: web::Json<GithubPayload>,
) -> Result<HttpResponse, Error> {
	let conn = pool.get().unwrap();

	if webhook_post.action != "opened" {
		return Ok(HttpResponse::Ok().body("Invalid format, challenges are only issued on issue submissions"));
	}

	let mut user = User::by_id(&webhook_post.sender.login, &conn);
	if user.is_none() {
		// Create user
		user = User::create(&webhook_post.sender.login, &conn);
	}

	if user.is_none() {
		return Ok(HttpResponse::Ok().body("Could not find or create user..."));
	}

	let result = Submission::list(&conn);

	// Create submission
	Submission::create(
		&user.unwrap().id, 
		&webhook_post.issue.body, 
		Some(&webhook_post.issue.title), 
		0, 
		&webhook_post.issue.html_url, 
		webhook_post.issue.number,
		&conn
	);

	create_issue_comment(webhook_post.issue.number, "Thanks for submitting!<br>Your code is being processed...");

	Ok(HttpResponse::Ok().json("Challenge has started for bot"))
}