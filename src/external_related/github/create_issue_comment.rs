use crate::external_related::{repo_updater::is_live, string_escape::escape_string};

pub fn create_issue_comment(issue_number: i32, msg: &str) {
    let secret = std::env::var("GITHUB_POST_SECRET").unwrap();
    let user = std::env::var("GITHUB_USER").unwrap_or("hampfh".to_string());
    let repo = std::env::var("GITHUB_REPO").unwrap_or("temp".to_string());
    if !is_live() {
        println!("[OFFLINE] Issue comment: {}", msg);
        return;
    }

    // use reqwest to send a post request to https://api.github.com
    let client = reqwest::blocking::Client::new();
    let url = format!(
        "https://api.github.com/repos/{}/{}/issues/{}/comments",
        user, repo, issue_number
    );
    let req = client
        .post(&url)
        .header("User-Agent", user)
        .header("Accept", "application/vnd.github.v3+json")
        .header("Authorization", format!("token {}", secret))
        .body(format!(
            "{{\"body\": \"[THIS MESSAGE IS AUTOMATIC]<br> {}\"}}",
            escape_string(msg.to_string())
        ));

    // Send request
    match req.send() {
        Ok(_) => (),
        Err(e) => {
            println!(
                "[ERROR] Could not post issue comment for {}: Error: {}",
                issue_number, e
            );
        }
    }
}
