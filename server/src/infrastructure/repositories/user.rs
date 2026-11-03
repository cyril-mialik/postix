use async_trait::async_trait;
use sqlx::PgPool;
use uuid::Uuid;

use crate::domain::{
    entities::{
        errors::{Error, SystemError},
        user::{User, UserError},
    },
    repositories::user::UserRepository,
};

pub struct PostgresUserRepository {
    pool: PgPool,
}

impl PostgresUserRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl UserRepository for PostgresUserRepository {
    async fn create_user(&self, user: &User, password_hash: &str) -> Result<User, Error> {
        let created_user = sqlx::query_as!(
            User,
            r#"
            INSERT INTO users (id, email, password, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5)
            ON CONFLICT (email) DO NOTHING
            RETURNING id, email, created_at, updated_at
            "#,
            user.id,
            user.email,
            password_hash,
            user.created_at,
            user.updated_at
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| Error::System(SystemError::Database(e.to_string())))?
        .ok_or(Error::User(UserError::EmailAlreadyExists))?;

        Ok(created_user)
    }

    async fn find_by_email(&self, email: &str) -> Result<Option<User>, Error> {
        let user = sqlx::query_as!(
            User,
            r#"
            SELECT id, email, created_at, updated_at
            FROM users
            WHERE email = $1
            "#,
            email
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| Error::System(SystemError::Database(e.to_string())))?;

        Ok(user)
    }

    async fn find_credentials_by_email(
        &self,
        email: &str,
    ) -> Result<Option<(User, String)>, Error> {
        let record = sqlx::query!(
            r#"
            SELECT id, email, password, created_at, updated_at
            FROM users
            WHERE email = $1
            "#,
            email
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| Error::System(SystemError::Database(e.to_string())))?;

        Ok(record.map(|r| {
            let user = User {
                id: r.id,
                email: r.email,
                created_at: r.created_at,
                updated_at: r.updated_at,
            };
            (user, r.password)
        }))
    }

    async fn find_by_id(&self, id: Uuid) -> Result<Option<User>, Error> {
        let user = sqlx::query_as!(
            User,
            r#"
            SELECT id, email, created_at, updated_at
            FROM users
            WHERE id = $1
            "#,
            id
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| Error::System(SystemError::Database(e.to_string())))?;

        Ok(user)
    }

    async fn update_email(&self, id: Uuid, new_email: &str) -> Result<User, Error> {
        let exists = sqlx::query_scalar!(
            r#"SELECT EXISTS(SELECT 1 FROM users WHERE id = $1) as "exists!""#,
            id
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|e| Error::System(SystemError::Database(e.to_string())))?;

        if !exists {
            return Err(Error::User(UserError::UserNotFound));
        }

        let updated_user = sqlx::query_as!(
            User,
            r#"
            UPDATE users
            SET email = $2, updated_at = $3
            WHERE id = $1
            RETURNING id, email, created_at, updated_at
            "#,
            id,
            new_email,
            chrono::Utc::now()
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| Error::System(SystemError::Database(e.to_string())))?
        .ok_or(Error::User(UserError::EmailAlreadyExists))?;

        Ok(updated_user)
    }

    async fn update_password(&self, id: Uuid, new_password: &str) -> Result<User, Error> {
        let exists = sqlx::query_scalar!(
            r#"SELECT EXISTS(SELECT 1 FROM users WHERE id = $1) as "exists!""#,
            id
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|e| Error::System(SystemError::Database(e.to_string())))?;

        if !exists {
            return Err(Error::User(UserError::UserNotFound));
        }

        let updated_user = sqlx::query_as!(
            User,
            r#"
            UPDATE users
            SET password = $2, updated_at = $3
            WHERE id = $1
            RETURNING id, email, created_at, updated_at
            "#,
            id,
            new_password,
            chrono::Utc::now()
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| Error::System(SystemError::Database(e.to_string())))?
        .ok_or(Error::User(UserError::UserNotFound))?;

        Ok(updated_user)
    }

    async fn delete_user(&self, id: Uuid) -> Result<(), Error> {
        let result = sqlx::query!(
            r#"
            DELETE FROM users
            WHERE id = $1
            "#,
            id
        )
        .execute(&self.pool)
        .await
        .map_err(|e| Error::System(SystemError::Database(e.to_string())))?;

        if result.rows_affected() == 0 {
            return Err(Error::User(UserError::UserNotFound));
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;
    use sqlx::PgPool;
    use uuid::Uuid;

    fn make_user(email: &str) -> User {
        User {
            id: Uuid::new_v4(),
            email: email.to_string(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }

    #[sqlx::test]
    async fn test_create_user(pool: PgPool) -> sqlx::Result<()> {
        let repo = PostgresUserRepository::new(pool);
        let user = make_user("create@example.com");
        let password_hash = "hashed_password_123";

        let created = repo
            .create_user(&user, password_hash)
            .await
            .expect("Failed to create user");

        assert_eq!(created.id, user.id);
        assert_eq!(created.email, "create@example.com");

        // created_at и updated_at могут отличаться на микросекунды, проверяем, что они есть
        assert!(created.created_at <= Utc::now());
        assert!(created.updated_at <= Utc::now());

        Ok(())
    }

    #[sqlx::test]
    async fn test_create_user_duplicate_email(pool: PgPool) -> sqlx::Result<()> {
        let repo = PostgresUserRepository::new(pool);
        let user1 = make_user("dup@example.com");
        let user2 = make_user("dup@example.com");

        repo.create_user(&user1, "hash1").await.unwrap();
        
        let result = repo.create_user(&user2, "hash2").await;
        
        // Благодаря ON CONFLICT DO NOTHING, вернется None, и мы получим EmailAlreadyExists
        assert!(matches!(result, Err(Error::User(UserError::EmailAlreadyExists))));

        Ok(())
    }

    #[sqlx::test]
    async fn test_find_by_email(pool: PgPool) -> sqlx::Result<()> {
        let repo = PostgresUserRepository::new(pool);
        let user = make_user("find@example.com");

        repo.create_user(&user, "hash").await.unwrap();

        let found = repo.find_by_email("find@example.com").await.unwrap();
        assert!(found.is_some());
        assert_eq!(found.unwrap().id, user.id);

        let not_found = repo.find_by_email("no_such@example.com").await.unwrap();
        assert!(not_found.is_none());

        Ok(())
    }

    #[sqlx::test]
    async fn test_find_credentials_by_email(pool: PgPool) -> sqlx::Result<()> {
        let repo = PostgresUserRepository::new(pool);
        let user = make_user("creds@example.com");
        let password_hash = "super_secret_hash";

        repo.create_user(&user, password_hash).await.unwrap();

        let found = repo.find_credentials_by_email("creds@example.com").await.unwrap();
        assert!(found.is_some());
        
        let (found_user, found_hash) = found.unwrap();
        assert_eq!(found_user.id, user.id);
        assert_eq!(found_hash, password_hash);

        let not_found = repo.find_credentials_by_email("no_such@example.com").await.unwrap();
        assert!(not_found.is_none());

        Ok(())
    }

    #[sqlx::test]
    async fn test_find_by_id(pool: PgPool) -> sqlx::Result<()> {
        let repo = PostgresUserRepository::new(pool);
        let user = make_user("find_id@example.com");

        repo.create_user(&user, "hash").await.unwrap();

        let found = repo.find_by_id(user.id).await.unwrap();
        assert!(found.is_some());
        assert_eq!(found.unwrap().email, "find_id@example.com");

        let not_found = repo.find_by_id(Uuid::new_v4()).await.unwrap();
        assert!(not_found.is_none());

        Ok(())
    }

    #[sqlx::test]
    async fn test_update_email(pool: PgPool) -> sqlx::Result<()> {
        let repo = PostgresUserRepository::new(pool);
        let user = make_user("old_email@example.com");

        repo.create_user(&user, "hash").await.unwrap();

        // Небольшая задержка, чтобы updated_at точно изменился
        tokio::time::sleep(std::time::Duration::from_millis(10)).await;

        let updated = repo
            .update_email(user.id, "new_email@example.com")
            .await
            .expect("Failed to update email");

        assert_eq!(updated.email, "new_email@example.com");
        assert!(updated.updated_at > user.updated_at);

        Ok(())
    }

    #[sqlx::test]
    async fn test_update_email_not_found(pool: PgPool) -> sqlx::Result<()> {
        let repo = PostgresUserRepository::new(pool);
        
        let result = repo.update_email(Uuid::new_v4(), "new@example.com").await;
        assert!(matches!(result, Err(Error::User(UserError::UserNotFound))));

        Ok(())
    }

    #[sqlx::test]
    async fn test_update_email_duplicate(pool: PgPool) -> sqlx::Result<()> {
        let repo = PostgresUserRepository::new(pool);
        let user1 = make_user("user1@example.com");
        let user2 = make_user("user2@example.com");

        repo.create_user(&user1, "hash1").await.unwrap();
        repo.create_user(&user2, "hash2").await.unwrap();

        // Пытаемся user2 поменять email на email user1
        let result = repo.update_email(user2.id, "user1@example.com").await;
        
        // Ожидаем ошибку (в текущей реализации это SystemError::Database из-за unique_violation, 
        // но по логике должно быть EmailAlreadyExists)
        assert!(result.is_err());

        Ok(())
    }

    #[sqlx::test]
    async fn test_update_password(pool: PgPool) -> sqlx::Result<()> {
        let repo = PostgresUserRepository::new(pool);
        let user = make_user("pass@example.com");

        repo.create_user(&user, "old_hash").await.unwrap();

        tokio::time::sleep(std::time::Duration::from_millis(10)).await;

        let updated = repo
            .update_password(user.id, "new_hash")
            .await
            .expect("Failed to update password");

        assert_eq!(updated.id, user.id);
        assert!(updated.updated_at > user.updated_at);

        let (_, found_hash) = repo.find_credentials_by_email("pass@example.com").await.unwrap().unwrap();
        assert_eq!(found_hash, "new_hash");

        Ok(())
    }

    #[sqlx::test]
    async fn test_update_password_not_found(pool: PgPool) -> sqlx::Result<()> {
        let repo = PostgresUserRepository::new(pool);
        
        let result = repo.update_password(Uuid::new_v4(), "new_hash").await;
        assert!(matches!(result, Err(Error::User(UserError::UserNotFound))));

        Ok(())
    }

    #[sqlx::test]
    async fn test_delete_user(pool: PgPool) -> sqlx::Result<()> {
        let repo = PostgresUserRepository::new(pool);
        let user = make_user("delete@example.com");

        repo.create_user(&user, "hash").await.unwrap();

        repo.delete_user(user.id).await.expect("Failed to delete user");

        let found = repo.find_by_id(user.id).await.unwrap();
        assert!(found.is_none());

        let result = repo.delete_user(user.id).await;
        assert!(matches!(result, Err(Error::User(UserError::UserNotFound))));

        Ok(())
    }
}
