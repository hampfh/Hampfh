use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct GithubPayload {
    pub action: String,
    pub issue: Issue,
    pub repository: Repository,
    pub sender: User,
}

#[derive(Serialize, Deserialize)]
pub struct Issue {
    pub url: String,
    pub repository_url: String,
    pub labels_url: String,
    pub comments_url: String,
    pub events_url: String,
    pub html_url: String,
    pub id: i32,
    pub node_id: String,
    pub number: i32,
    pub title: String,
    pub user: User,
    pub labels: Vec<Label>,
    pub state: String,
    pub locked: bool,
    //pub assignee: null,
    //pub assignees: [],
    //pub milestone: null,
    pub comments: i32,
    pub created_at: String,
    pub updated_at: String,
    pub closed_at: Option<String>,
    pub author_association: String,
    //pub active_lock_reason: null,
    pub body: String, // This is where the code will come from
    pub reactions: Reactions,
    pub timeline_url: String,
    //pub performed_via_github_app: null,
    //pub state_reason: null
}

#[derive(Serialize, Deserialize)]
pub struct Reactions {
    pub url: String,
    pub total_count: i32,
    //pub "+1": i32,
    //pub "-1": i32,
    pub laugh: i32,
    pub hooray: i32,
    pub confused: i32,
    pub heart: i32,
    pub rocket: i32,
    pub eyes: i32,
}

#[derive(Serialize, Deserialize)]
pub struct User {
    pub login: String, // Github username
    pub id: i32,
    pub node_id: String,
    pub avatar_url: String,
    pub gravatar_id: String,
    pub url: String,
    pub html_url: String,
    pub followers_url: String,
    pub following_url: String,
    pub gists_url: String,
    pub starred_url: String,
    pub subscriptions_url: String,
    pub organizations_url: String,
    pub repos_url: String,
    pub events_url: String,
    pub received_events_url: String,
    //pub "type": String,
    pub site_admin: bool,
}

#[derive(Serialize, Deserialize)]
pub struct Repository {
    pub id: i32,
    pub name: String,
    pub full_name: String,
}

#[derive(Serialize, Deserialize)]
pub struct Label {
    pub id: i32,
    pub node_id: String,
    pub url: String,
    pub name: String,
    pub color: String,
    pub default: bool,
}
