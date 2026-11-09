use chrono::{NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PostView {
    pub post_id: Uuid,
    pub viewer_id: Uuid,
    pub viewed_at: NaiveDate,
}

impl PostView {
    pub fn new(post_id: Uuid, viewer_id: Uuid) -> Self {
        Self {
            post_id,
            viewer_id,
            viewed_at: Utc::now().date_naive(),
        }
    }
}
