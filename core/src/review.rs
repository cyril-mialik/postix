use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct ReviewRequest {
    pub review: String,
}

#[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ReviewReaction {
    Agree,
    Disagree,
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct ReviewReactionRequest {
    pub reaction: ReviewReaction,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub struct ReviewReactionSummary {
    pub agree_count: i32,
    pub disagree_count: i32,
    pub my_reaction: Option<ReviewReaction>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ReviewResponse {
    pub id: i32,
    pub post_id: Uuid,
    pub author_id: Uuid,
    pub review: String,
    pub is_mine: bool,
    pub agree_count: i32,
    pub disagree_count: i32,
    pub my_reaction: Option<ReviewReaction>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ReviewsResponse {
    pub reviews: Vec<ReviewResponse>,
}
