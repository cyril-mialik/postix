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
