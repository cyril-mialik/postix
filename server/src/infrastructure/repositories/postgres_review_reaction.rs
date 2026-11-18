use async_trait::async_trait;
use sqlx::PgPool;

use crate::domain::{
    entities::{
        errors::{Error, SystemError},
        review_reaction::{ReviewReaction, ReviewReactionEntity, ReviewReactionSummary},
    },
    repositories::review_reaction::ReviewReactionRepository,
};

pub struct PostgresReviewReactionRepository {
    pool: PgPool,
}

impl PostgresReviewReactionRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl ReviewReactionRepository for PostgresReviewReactionRepository {
    async fn upsert(&self, reaction: &ReviewReactionEntity) -> Result<ReviewReactionEntity, Error> {
        let row = sqlx::query_as::<_, ReviewReactionRow>(
            r#"
            INSERT INTO post_review_reaction (review_id, profile_id, reaction)
            VALUES ($1, $2, $3)
            ON CONFLICT (review_id, profile_id)
            DO UPDATE SET reaction = EXCLUDED.reaction, updated_at = NOW()
            RETURNING id, reaction, created_at, updated_at
            "#,
        )
        .bind(reaction.review_id)
        .bind(reaction.profile_id)
        .bind(reaction.reaction.as_str())
        .fetch_one(&self.pool)
        .await
        .map_err(|e| Error::System(SystemError::Database(e.to_string())))?;

        let reaction_value = ReviewReaction::from_db(&row.reaction).ok_or_else(|| {
            Error::System(SystemError::Internal(format!(
                "invalid stored review reaction: {}",
                row.reaction
            )))
        })?;

        Ok(ReviewReactionEntity {
            id: row.id,
            reaction: reaction_value,
            created_at: row.created_at,
            updated_at: row.updated_at,
            ..reaction.clone()
        })
    }

    async fn summary(
        &self,
        review_id: i32,
        profile_id: uuid::Uuid,
    ) -> Result<ReviewReactionSummary, Error> {
        let row = sqlx::query_as::<_, ReviewReactionSummaryRow>(
            r#"
            SELECT COUNT(*) FILTER (WHERE reaction = 'agree')::INT AS agree_count,
                   COUNT(*) FILTER (WHERE reaction = 'disagree')::INT AS disagree_count,
                   MAX(reaction) FILTER (WHERE profile_id = $2) AS my_reaction
            FROM post_review_reaction
            WHERE review_id = $1
            "#,
        )
        .bind(review_id)
        .bind(profile_id)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| Error::System(SystemError::Database(e.to_string())))?;

        let my_reaction = row
            .my_reaction
            .as_deref()
            .map(|value| {
                ReviewReaction::from_db(value).ok_or_else(|| {
                    Error::System(SystemError::Internal(format!(
                        "invalid stored review reaction: {value}"
                    )))
                })
            })
            .transpose()?;

        Ok(ReviewReactionSummary {
            agree_count: row.agree_count,
            disagree_count: row.disagree_count,
            my_reaction,
        })
    }
}

#[derive(sqlx::FromRow)]
struct ReviewReactionRow {
    id: i32,
    reaction: String,
    created_at: chrono::DateTime<chrono::Utc>,
    updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(sqlx::FromRow)]
struct ReviewReactionSummaryRow {
    agree_count: i32,
    disagree_count: i32,
    my_reaction: Option<String>,
}
