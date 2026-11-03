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

    async fn is_refresh_token_valid(&self, user_id: Uuid, token: &str) -> Result<bool, Error> {
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

#[cfg(test)]
mod tests {
    use super::*;
    use deadpool_redis::{Config, Runtime};
    use rstest::{fixture, rstest};
    use uuid::Uuid;

    fn create_pool() -> Pool {
        dotenvy::dotenv().ok();

        let url = std::env::var("CACHE_URL")
            .or_else(|_| std::env::var("REDIS_URL"))
            .unwrap_or_else(|_| "redis://127.0.0.1:6379/0".to_string());
        let cfg = Config::from_url(url);

        cfg.create_pool(Some(Runtime::Tokio1))
            .expect("failed to create redis pool")
    }

    #[fixture]
    fn repo() -> RedisTokenRepository {
        RedisTokenRepository::new(create_pool())
    }

    #[fixture]
    fn user_id() -> Uuid {
        Uuid::new_v4()
    }

    #[rstest]
    #[case("test_token_1", 60)]
    #[case("test_token_long", 120)]
    #[case("x", 1)]
    #[tokio::test]
    async fn test_save_and_is_valid(
        repo: RedisTokenRepository,
        user_id: Uuid,
        #[case] token: &str,
        #[case] ttl: u64,
    ) {
        repo.save_refresh_token(user_id, token, ttl)
            .await
            .expect("save");

        let valid = repo
            .is_refresh_token_valid(user_id, token)
            .await
            .expect("valid");

        assert!(valid);

        repo.revoke_refresh_token(user_id, token).await.ok();
    }

    #[rstest]
    #[case("nonexistent")]
    #[case("")]
    #[case("random_token")]
    #[tokio::test]
    async fn test_is_valid_missing(repo: RedisTokenRepository, user_id: Uuid, #[case] token: &str) {
        let valid = repo
            .is_refresh_token_valid(user_id, token)
            .await
            .expect("valid");

        assert!(!valid);
    }

    #[rstest]
    #[case("test_token_revoke", 60)]
    #[case("another_token", 120)]
    #[tokio::test]
    async fn test_revoke_refresh_token(
        repo: RedisTokenRepository,
        user_id: Uuid,
        #[case] token: &str,
        #[case] ttl: u64,
    ) {
        repo.save_refresh_token(user_id, token, ttl)
            .await
            .expect("save");
        repo.revoke_refresh_token(user_id, token)
            .await
            .expect("revoke");

        let valid = repo
            .is_refresh_token_valid(user_id, token)
            .await
            .expect("valid");

        assert!(!valid, "token should be revoked");
    }

    #[rstest]
    #[case("test_token_ttl", 1)]
    #[tokio::test]
    async fn test_ttl_expires(
        repo: RedisTokenRepository,
        user_id: Uuid,
        #[case] token: &str,
        #[case] ttl: u64,
    ) {
        repo.save_refresh_token(user_id, token, ttl)
            .await
            .expect("save");
        assert!(repo.is_refresh_token_valid(user_id, token).await.unwrap());

        tokio::time::sleep(std::time::Duration::from_secs(ttl + 1)).await;

        let valid = repo.is_refresh_token_valid(user_id, token).await.unwrap();

        assert!(!valid, "token should expire after TTL");
    }

    #[rstest]
    #[case(vec!["token_a", "token_b", "token_c"])]
    #[case(vec!["only_one"])]
    #[case(vec!["a", "b", "c", "d", "e"])]
    #[tokio::test]
    async fn test_revoke_all_refresh_tokens(
        repo: RedisTokenRepository,
        user_id: Uuid,
        #[case] tokens: Vec<&str>,
    ) {
        for token in &tokens {
            repo.save_refresh_token(user_id, token, 60).await.unwrap();
        }

        let other_user = Uuid::new_v4();
        repo.save_refresh_token(other_user, "other_token", 60)
            .await
            .unwrap();

        repo.revoke_all_refresh_tokens(user_id)
            .await
            .expect("revoke all");

        for token in &tokens {
            assert!(
                !repo.is_refresh_token_valid(user_id, token).await.unwrap(),
                "token {} should be revoked",
                token
            );
        }

        assert!(
            repo.is_refresh_token_valid(other_user, "other_token")
                .await
                .unwrap()
        );

        repo.revoke_refresh_token(other_user, "other_token")
            .await
            .ok();
    }

    #[rstest]
    #[case("test_token_overwrite", 60, 120)]
    #[case("another", 30, 60)]
    #[tokio::test]
    async fn test_save_overwrites_ttl(
        repo: RedisTokenRepository,
        user_id: Uuid,
        #[case] token: &str,
        #[case] ttl1: u64,
        #[case] ttl2: u64,
    ) {
        repo.save_refresh_token(user_id, token, ttl1).await.unwrap();
        repo.save_refresh_token(user_id, token, ttl2).await.unwrap();

        assert!(repo.is_refresh_token_valid(user_id, token).await.unwrap());

        repo.revoke_refresh_token(user_id, token).await.ok();
    }

    #[rstest]
    #[case("same_token_string")]
    #[case("shared_token")]
    #[tokio::test]
    async fn test_isolation_between_users(repo: RedisTokenRepository, #[case] token: &str) {
        let user1 = Uuid::new_v4();
        let user2 = Uuid::new_v4();

        repo.save_refresh_token(user1, token, 60).await.unwrap();

        assert!(!repo.is_refresh_token_valid(user2, token).await.unwrap());

        repo.revoke_refresh_token(user1, token).await.ok();
    }
}
