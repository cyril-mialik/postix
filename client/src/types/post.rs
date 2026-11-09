use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct PostResponse {
    pub id: String,
    pub title: String,
    pub content: serde_json::Value,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct PostRequest {
    pub title: String,
    pub content: serde_json::Value,
    pub tag_names: Vec<String>,
}
