use std::process::Command;

pub fn is_live() -> bool {
    return std::env::var("LIVE").unwrap() == "true";
}

pub fn is_plagiarism_enabled() -> bool {
    return std::env::var("PLAGIARISM_CHECK").unwrap() == "true";
}

pub(crate) fn get_issue_url(issue_number: i32) -> String {
    format!(
        "https://github.com/{}/{}/issues/{}",
        std::env::var("GITHUB_USER").unwrap(),
        std::env::var("GITHUB_REPO").unwrap(),
        issue_number
    )
}

/// This function will push changes to the repository.
pub fn update_repo(commit_msg: String) {
    if !is_live() {
        println!("[OFFLINE] Skipping update_repo");
        return;
    }

    Command::new("git")
        .arg("fetch")
        .output()
        .expect("Could not fetch");

    /*
       ! This action is very important since what we're doing
       ! here is destructive to the git history. Hence we need
       ! to make sure that we are not on a branch with actual data.
    */
    Command::new("git")
        .arg("switch")
        .arg("live")
        .output()
        .expect("Could not switch to live branch");

    Command::new("git")
        .arg("stash")
        .output()
        .expect("Could not stash");

    Command::new("git")
        .arg("reset")
        .arg("--hard")
        .arg("origin/master")
        .output()
        .expect("Could not reset live to master branch");

    Command::new("git")
        .arg("stash")
        .arg("pop")
        .output()
        .expect("Could not pop stash");

    Command::new("git")
        .arg("add")
        .arg("-A")
        .output()
        .expect("Could not add submission");
    Command::new("git")
        .arg("commit")
        .arg("-m")
        .arg(commit_msg)
        .output()
        .expect("Could not create commit");
    Command::new("git")
        .arg("push")
        .arg("-f") // ! This is a potentially dangerous flag but we need it here to overwrite old match data.
        .output()
        .expect("Could not push");
    println!("Commands executed!");
}
