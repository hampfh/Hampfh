use diesel::SqliteConnection;
use crate::db::models::submission_model::Submission;
use crate::db::schema::Submissions::dsl::Submissions as submission_dsl;

pub struct Challenger {
	pub script: String,
	pub wins: u32,
}

fn match_make(challenger: &mut Challenger, conn: &SqliteConnection) {
	// Order by score and pick the submission with the higest score
	
	// Match-maker goes here
}