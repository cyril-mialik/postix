use async_trait::async_trait;
use deadpool_redis::Pool;
use redis::AsyncCommands;
use uuid::Uuid;

use crate::domain::{
    entities::errors::{Error, SystemError},
    repositories::token::TokenRepository,
};

pub struct RedisTokenRepository {
    pool: Pool,
}

impl RedisTokenRepository {
    pub fn new(pool: Pool) -> Self {
        Self { pool }
    }

    fn key(user_id: Uuid, token: &str) -> String {
        format!("refresh_token:{}:{}", user_id, token)
    }
}

#[async_trait]
impl TokenRepository for RedisTokenRepository {
    async fn save_refresh_token(
        &self,
        user_id: Uuid,
        token: &str,
        ttl_seconds: u64,
    ) -> Result<(), Error> {
        let mut conn = self
            .pool
            .get()
            .await
            .map_err(|e| Error::System(SystemError::Cache(e.to_string())))?;

        let key = Self::key(user_id, token);
        let _: () = conn
            .set_ex(&key, 1, ttl_seconds)
            .await
            .map_err(|e| Error::System(SystemError::Cache(e.to_string())))?;

        Ok(())
    }

    async fn is_refresh_token_valid(
        &self,
        user_id: Uuid,
        token: &str,
    ) -> Result<bool, Error> {
        let mut conn = self
            .pool
            .get()
            .await
            .map_err(|e| Error::System(SystemError::Cache(e.to_string())))?;

        let key = Self::key(user_id, token);
        let exists: bool = conn
            .exists(&key)
            .await
            .map_err(|e| Error::System(SystemError::Cache(e.to_string())))?;

        Ok(exists)
    }

    async fn revoke_refresh_token(&self, user_id: Uuid, token: &str) -> Result<(), Error> {
        let mut conn = self
            .pool
            .get()
            .await
            .map_err(|e| Error::System(SystemError::Cache(e.to_string())))?;

        let key = Self::key(user_id, token);
        let _: () = conn
            .del(&key)
            .await
            .map_err(|e| Error::System(SystemError::Cache(e.to_string())))?;

        Ok(())
    }

    async fn revoke_all_refresh_tokens(&self, user_id: Uuid) -> Result<(), Error> {
        let mut conn = self
            .pool
            .get()
            .await
            .map_err(|e| Error::System(SystemError::Cache(e.to_string())))?;

        let pattern = format!("refresh_token:{}:*", user_id);
        let keys: Vec<String> = conn
            .keys(&pattern)
            .await
            .map_err(|e| Error::System(SystemError::Cache(e.to_string())))?;

        if !keys.is_empty() {
            let _: () = conn
                .del(keys)
                .await
                .map_err(|e| Error::System(SystemError::Cache(e.to_string())))?;
        }

        Ok(())
    }
}
