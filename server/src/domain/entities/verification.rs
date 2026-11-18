use chrono::{DateTime, Utc};
use uuid::Uuid;

use super::verification_type::VerificationType;

#[derive(Debug, Clone)]
pub struct ProfileVerification {
    pub profile_id: Uuid,
    pub verification_type: VerificationType,
    pub created_at: DateTime<Utc>,
}

impl ProfileVerification {
    pub fn new(profile_id: Uuid, verification_type: VerificationType) -> Self {
        Self {
            profile_id,
            verification_type,
            created_at: Utc::now(),
        }
    }
}
