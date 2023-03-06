use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetGistResponse {
    pub url: String,
    pub forks_url: String,
    pub commits_url: String,
    pub id: String,
    pub node_id: String,
    pub git_pull_url: String,
    pub git_push_url: String,
    pub html_url: String,
    pub files: HashMap<String, File>,
    pub public: bool,
    pub created_at: String,
    pub updated_at: String,
    pub description: String,
    pub comments: i64,
    pub comments_url: String,
    pub owner: Owner,
    pub history: Vec<History>,
    pub truncated: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct File {
    pub filename: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub language: String,
    pub raw_url: String,
    pub size: i64,
    pub truncated: bool,
    pub content: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Owner {
    pub login: String,
    pub id: i64,
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
    #[serde(rename = "type")]
    pub type_field: String,
    pub site_admin: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct History {
    pub user: Owner,
    pub version: String,
    pub committed_at: String,
    pub change_status: ChangeStatus,
    pub url: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ChangeStatus {
    pub total: i64,
    pub additions: i64,
    pub deletions: i64,
}
