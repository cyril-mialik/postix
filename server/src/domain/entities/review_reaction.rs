use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReviewReaction {
    Agree,
    Disagree,
}

impl ReviewReaction {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Agree => "agree",
            Self::Disagree => "disagree",
        }
    }

    pub fn from_db(value: &str) -> Option<Self> {
        match value {
            "agree" => Some(Self::Agree),
            "disagree" => Some(Self::Disagree),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReviewReactionEntity {
    pub id: i32,
    pub review_id: i32,
    pub profile_id: Uuid,
    pub reaction: ReviewReaction,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl ReviewReactionEntity {
    pub fn new(review_id: i32, profile_id: Uuid, reaction: ReviewReaction) -> Self {
        Self {
            id: 0,
            review_id,
            profile_id,
            reaction,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReviewReactionSummary {
    pub agree_count: i32,
    pub disagree_count: i32,
    pub my_reaction: Option<ReviewReaction>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reactions_map_to_the_persisted_values() {
        assert_eq!(ReviewReaction::Agree.as_str(), "agree");
        assert_eq!(ReviewReaction::Disagree.as_str(), "disagree");
        assert_eq!(
            ReviewReaction::from_db("agree"),
            Some(ReviewReaction::Agree)
        );
        assert_eq!(
            ReviewReaction::from_db("disagree"),
            Some(ReviewReaction::Disagree)
        );
        assert_eq!(ReviewReaction::from_db("other"), None);
    }

    #[test]
    fn new_reaction_starts_without_a_persisted_id() {
        let reaction = ReviewReactionEntity::new(7, Uuid::nil(), ReviewReaction::Agree);

        assert_eq!(reaction.id, 0);
        assert_eq!(reaction.review_id, 7);
        assert_eq!(reaction.reaction, ReviewReaction::Agree);
    }
}
