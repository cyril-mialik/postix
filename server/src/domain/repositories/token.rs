use crate::domain::entities::errors::Error;
use async_trait::async_trait;
use uuid::Uuid;

#[async_trait]
pub trait TokenRepository: Send + Sync {
    async fn save_refresh_token(
        &self,
        user_id: Uuid,
        token: &str,
        ttl_seconds: u64,
    ) -> Result<(), Error>;

    async fn is_refresh_token_valid(&self, user_id: Uuid, token: &str)
    -> Result<bool, Error>;

    async fn revoke_refresh_token(&self, user_id: Uuid, token: &str) -> Result<(), Error>;

    async fn revoke_all_refresh_tokens(&self, user_id: Uuid) -> Result<(), Error>;
}
