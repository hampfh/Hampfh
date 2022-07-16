use std::process::Command;

pub fn is_live() -> bool {
    return std::env::var("LIVE").unwrap() == "true";
}

pub fn is_plagiarism_enabled() -> bool {
    return std::env::var("PLAGIARISM_CHECK").unwrap() == "true";
}

/// This function will push changes to the repository.
pub fn update_repo(commit_msg: String) {
    if !is_live() {
        println!("[OFFLINE] Skipping update_repo");
        return;
    }

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
        .output()
        .expect("Could not push");
    println!("Commands executed!");
}
