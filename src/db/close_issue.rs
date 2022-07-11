use crate::external_related::repo_updater::is_live;

#[derive(Debug)]
pub enum CloseType {
    NotPlanned,
    Completed,
}
pub fn close_issue(state: CloseType, issue_number: i32) {
    let secret = std::env::var("GITHUB_POST_SECRET").unwrap();
    let user = std::env::var("GITHUB_USER").unwrap_or("hampfh".to_string());
    let repo = std::env::var("GITHUB_REPO").unwrap_or("temp".to_string());
    if !is_live() {
        println!("[OFFLINE] Close issue, reason: {:?}", state);
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
            "{{\"state\": \"closed\", \"state_reason\": \"{}\"}}",
            match state {
                CloseType::NotPlanned => "not_planned",
                CloseType::Completed => "completed",
            }
        ));

    // Send request
    req.send().unwrap();
}
