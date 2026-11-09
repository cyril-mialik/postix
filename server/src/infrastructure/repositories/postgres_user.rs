use async_trait::async_trait;
use sqlx::PgPool;
use uuid::Uuid;

use crate::domain::{
    entities::{
        errors::{Error, SystemError},
        role::Role,
        theme::Theme,
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
    async fn find_by_id(&self, id: Uuid) -> Result<Option<User>, Error> {
        let row = sqlx::query!(
            r#"
            SELECT a.id, a.email, a.password, a.is_email_verified, a.is_banned,
                   a.ban_reason, a.banned_at, a.role_id,
                   p.username, p.avatar, p.first_name, p.last_name,
                   p.followers_count, p.following_count, p.posts_count,
                   p.theme_id, p.deleted_at,
                   a.created_at, a.updated_at
            FROM auth a
            JOIN profile p ON p.auth_id = a.id
            WHERE a.id = $1
            "#,
            id
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| Error::System(SystemError::Database(e.to_string())))?;

        Ok(row.map(|r| User {
            id: r.id,
            email: r.email,
            password_hash: r.password,
            is_email_verified: r.is_email_verified,
            is_banned: r.is_banned,
            ban_reason: r.ban_reason,
            banned_at: r.banned_at,
            role: Role::from_id(r.role_id).unwrap_or(Role::User),
            username: r.username,
            avatar: r.avatar,
            first_name: r.first_name,
            last_name: r.last_name,
            followers_count: r.followers_count,
            following_count: r.following_count,
            posts_count: r.posts_count,
            theme: Theme::from_id(r.theme_id).unwrap_or(Theme::Auto),
            deleted_at: r.deleted_at,
            created_at: r.created_at,
            updated_at: r.updated_at,
        }))
    }

    async fn find_by_email(&self, email: &str) -> Result<Option<User>, Error> {
        let row = sqlx::query!(
            r#"
            SELECT a.id, a.email, a.password, a.is_email_verified, a.is_banned,
                   a.ban_reason, a.banned_at, a.role_id,
                   p.username, p.avatar, p.first_name, p.last_name,
                   p.followers_count, p.following_count, p.posts_count,
                   p.theme_id, p.deleted_at,
                   a.created_at, a.updated_at
            FROM auth a
            JOIN profile p ON p.auth_id = a.id
            WHERE lower(a.email) = lower($1)
            "#,
            email
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| Error::System(SystemError::Database(e.to_string())))?;

        Ok(row.map(|r| User {
            id: r.id,
            email: r.email,
            password_hash: r.password,
            is_email_verified: r.is_email_verified,
            is_banned: r.is_banned,
            ban_reason: r.ban_reason,
            banned_at: r.banned_at,
            role: Role::from_id(r.role_id).unwrap_or(Role::User),
            username: r.username,
            avatar: r.avatar,
            first_name: r.first_name,
            last_name: r.last_name,
            followers_count: r.followers_count,
            following_count: r.following_count,
            posts_count: r.posts_count,
            theme: Theme::from_id(r.theme_id).unwrap_or(Theme::Auto),
            deleted_at: r.deleted_at,
            created_at: r.created_at,
            updated_at: r.updated_at,
        }))
    }

    async fn find_by_username(&self, username: &str) -> Result<Option<User>, Error> {
        let row = sqlx::query!(
            r#"
            SELECT a.id, a.email, a.password, a.is_email_verified, a.is_banned,
                   a.ban_reason, a.banned_at, a.role_id,
                   p.username, p.avatar, p.first_name, p.last_name,
                   p.followers_count, p.following_count, p.posts_count,
                   p.theme_id, p.deleted_at,
                   a.created_at, a.updated_at
            FROM auth a
            JOIN profile p ON p.auth_id = a.id
            WHERE lower(p.username) = lower($1)
            "#,
            username
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| Error::System(SystemError::Database(e.to_string())))?;

        Ok(row.map(|r| User {
            id: r.id,
            email: r.email,
            password_hash: r.password,
            is_email_verified: r.is_email_verified,
            is_banned: r.is_banned,
            ban_reason: r.ban_reason,
            banned_at: r.banned_at,
            role: Role::from_id(r.role_id).unwrap_or(Role::User),
            username: r.username,
            avatar: r.avatar,
            first_name: r.first_name,
            last_name: r.last_name,
            followers_count: r.followers_count,
            following_count: r.following_count,
            posts_count: r.posts_count,
            theme: Theme::from_id(r.theme_id).unwrap_or(Theme::Auto),
            deleted_at: r.deleted_at,
            created_at: r.created_at,
            updated_at: r.updated_at,
        }))
    }

    async fn find_credentials_by_email(&self, email: &str) -> Result<Option<(User, String)>, Error> {
        let row = sqlx::query!(
            r#"
            SELECT a.id, a.email, a.password, a.is_email_verified, a.is_banned,
                   a.ban_reason, a.banned_at, a.role_id,
                   p.username, p.avatar, p.first_name, p.last_name,
                   p.followers_count, p.following_count, p.posts_count,
                   p.theme_id, p.deleted_at,
                   a.created_at, a.updated_at
            FROM auth a
            JOIN profile p ON p.auth_id = a.id
            WHERE lower(a.email) = lower($1)
            "#,
            email
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| Error::System(SystemError::Database(e.to_string())))?;

        Ok(row.map(|r| {
            let password_hash = r.password.clone();
            let user = User {
                id: r.id,
                email: r.email,
                password_hash: r.password,
                is_email_verified: r.is_email_verified,
                is_banned: r.is_banned,
                ban_reason: r.ban_reason,
                banned_at: r.banned_at,
                role: Role::from_id(r.role_id).unwrap_or(Role::User),
                username: r.username,
                avatar: r.avatar,
                first_name: r.first_name,
                last_name: r.last_name,
                followers_count: r.followers_count,
                following_count: r.following_count,
                posts_count: r.posts_count,
                theme: Theme::from_id(r.theme_id).unwrap_or(Theme::Auto),
                deleted_at: r.deleted_at,
                created_at: r.created_at,
                updated_at: r.updated_at,
            };
            (user, password_hash)
        }))
    }

    async fn create(&self, user: &User) -> Result<User, Error> {
        let mut tx = self
            .pool
            .begin()
            .await
            .map_err(|e| Error::System(SystemError::Database(e.to_string())))?;

        let profile_id = Uuid::new_v4();

        sqlx::query!(
            r#"
            INSERT INTO auth (id, email, password, role_id, created_at, updated_at)
            VALUES ($1, lower($2), $3, $4, $5, $6)
            "#,
            user.id,
            user.email,
            user.password_hash,
            user.role.as_id(),
            user.created_at,
            user.updated_at,
        )
        .execute(&mut *tx)
        .await
        .map_err(|e| match e {
            sqlx::Error::Database(db) if db.is_unique_violation() => {
                Error::User(UserError::EmailAlreadyExists)
            }
            _ => Error::System(SystemError::Database(e.to_string())),
        })?;

        sqlx::query!(
            r#"
            INSERT INTO profile (id, auth_id, username, theme_id, created_at, updated_at)
            VALUES ($1, $2, lower($3), $4, $5, $6)
            "#,
            profile_id,
            user.id,
            user.username,
            user.theme.as_id(),
            user.created_at,
            user.updated_at,
        )
        .execute(&mut *tx)
        .await
        .map_err(|e| match e {
            sqlx::Error::Database(db) if db.is_unique_violation() => {
                Error::User(UserError::UsernameAlreadyExists)
            }
            _ => Error::System(SystemError::Database(e.to_string())),
        })?;

        tx.commit()
            .await
            .map_err(|e| Error::System(SystemError::Database(e.to_string())))?;

        Ok(user.clone())
    }

    async fn update(&self, user: &User) -> Result<User, Error> {
        let mut tx = self
            .pool
            .begin()
            .await
            .map_err(|e| Error::System(SystemError::Database(e.to_string())))?;

        let now = chrono::Utc::now();

        sqlx::query!(
            r#"
            UPDATE auth
            SET email = lower($2), password = $3, role_id = $4, is_email_verified = $5,
                is_banned = $6, ban_reason = $7, banned_at = $8, updated_at = $9
            WHERE id = $1
            "#,
            user.id,
            user.email,
            user.password_hash,
            user.role.as_id(),
            user.is_email_verified,
            user.is_banned,
            user.ban_reason,
            user.banned_at,
            now,
        )
        .execute(&mut *tx)
        .await
        .map_err(|e| match e {
            sqlx::Error::Database(db) if db.is_unique_violation() => {
                Error::User(UserError::EmailAlreadyExists)
            }
            _ => Error::System(SystemError::Database(e.to_string())),
        })?;

        sqlx::query!(
            r#"
            UPDATE profile
            SET username = lower($2), avatar = $3, first_name = $4, last_name = $5,
                theme_id = $6, updated_at = $7
            WHERE auth_id = $1
            "#,
            user.id,
            user.username,
            user.avatar,
            user.first_name,
            user.last_name,
            user.theme.as_id(),
            now,
        )
        .execute(&mut *tx)
        .await
        .map_err(|e| match e {
            sqlx::Error::Database(db) if db.is_unique_violation() => {
                Error::User(UserError::UsernameAlreadyExists)
            }
            _ => Error::System(SystemError::Database(e.to_string())),
        })?;

        tx.commit()
            .await
            .map_err(|e| Error::System(SystemError::Database(e.to_string())))?;

        Ok(user.clone())
    }

    async fn soft_delete(&self, id: Uuid) -> Result<(), Error> {
        sqlx::query!(
            "UPDATE profile SET deleted_at = NOW(), updated_at = NOW() WHERE auth_id = $1",
            id
        )
        .execute(&self.pool)
        .await
        .map_err(|e| Error::System(SystemError::Database(e.to_string())))?;
        Ok(())
    }

    async fn ban(&self, id: Uuid, reason: &str) -> Result<(), Error> {
        sqlx::query!(
            "UPDATE auth SET is_banned = true, ban_reason = $2, banned_at = NOW(), updated_at = NOW() WHERE id = $1",
            id, reason
        )
        .execute(&self.pool)
        .await
        .map_err(|e| Error::System(SystemError::Database(e.to_string())))?;
        Ok(())
    }

    async fn unban(&self, id: Uuid) -> Result<(), Error> {
        sqlx::query!(
            "UPDATE auth SET is_banned = false, ban_reason = NULL, banned_at = NULL, updated_at = NOW() WHERE id = $1",
            id
        )
        .execute(&self.pool)
        .await
        .map_err(|e| Error::System(SystemError::Database(e.to_string())))?;
        Ok(())
    }

    async fn increment_posts_count(&self, id: Uuid, delta: i32) -> Result<(), Error> {
        sqlx::query!(
            "UPDATE profile SET posts_count = posts_count + $2 WHERE auth_id = $1",
            id, delta
        )
        .execute(&self.pool)
        .await
        .map_err(|e| Error::System(SystemError::Database(e.to_string())))?;
        Ok(())
    }

    async fn increment_followers_count(&self, id: Uuid, delta: i32) -> Result<(), Error> {
        sqlx::query!(
            "UPDATE profile SET followers_count = followers_count + $2 WHERE auth_id = $1",
            id, delta
        )
        .execute(&self.pool)
        .await
        .map_err(|e| Error::System(SystemError::Database(e.to_string())))?;
        Ok(())
    }

    async fn increment_following_count(&self, id: Uuid, delta: i32) -> Result<(), Error> {
        sqlx::query!(
            "UPDATE profile SET following_count = following_count + $2 WHERE auth_id = $1",
            id, delta
        )
        .execute(&self.pool)
        .await
        .map_err(|e| Error::System(SystemError::Database(e.to_string())))?;
        Ok(())
    }
}
