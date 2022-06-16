use std::process::Command;

/**
 * This function will push changes to the repository.
 */
pub fn update_repo(submission_id: &str, author: &str) {
	Command::new("git").arg("add").arg("-A").output().expect("Could not add submission");
	Command::new("git").arg("commit").arg("-m").arg(format!("\"Submission [{}] by @{}\"", submission_id, author)).output().expect("Could not create commit");
	Command::new("git").arg("push").output().expect("Could not push");
	println!("Commands executed!");
}