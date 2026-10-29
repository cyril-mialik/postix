use async_trait::async_trait;
use sqlx::PgPool;

use crate::domain::{
    entities::{
        errors::{Error, SystemError},
        settings::{
            CreateSettingsDto, DeleteSettingsDto, FindSettingsByUserId, FindSettingsByUsernameDto,
            Settings, SettingsError, UpdateSettingsDto, UpdateSettingsFirstNameDto,
            UpdateSettingsLastNameDto, UpdateSettingsUsernameDto,
        },
    },
    repositories::settings::SettingsRepository,
};

pub struct PostgresSettingsRepository {
    pool: PgPool,
}

impl PostgresSettingsRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl SettingsRepository for PostgresSettingsRepository {
    async fn create_settings(&self, dto: CreateSettingsDto) -> Result<Settings, Error> {
        let settings = Settings::new(dto)?;
        let created_settings = sqlx::query_as!(
            Settings,
            r#"
            INSERT INTO settings (user_id, username, first_name, last_name)
            VALUES ($1, $2, $3, $4)
            RETURNING user_id, username, first_name, last_name
            "#,
            settings.user_id,
            settings.username,
            settings.first_name,
            settings.last_name,
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| Error::System(SystemError::Database(e.to_string())))?
        .ok_or(Error::Settings(SettingsError::AlreadyExists))?;

        Ok(created_settings)
    }

    async fn find_by_user_id(&self, dto: FindSettingsByUserId) -> Result<Option<Settings>, Error> {
        let settings = sqlx::query_as!(
            Settings,
            r#"
            SELECT user_id, username, first_name, last_name
            FROM settings
            WHERE user_id = $1
            "#,
            dto.user_id
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| Error::System(SystemError::Database(e.to_string())))?;

        Ok(settings)
    }

    async fn find_by_username(
        &self,
        dto: FindSettingsByUsernameDto,
    ) -> Result<Option<Settings>, Error> {
        let settings = sqlx::query_as!(
            Settings,
            r#"
            SELECT user_id, username, first_name, last_name
            FROM settings
            WHERE username = $1
            "#,
            dto.username
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| Error::System(SystemError::Database(e.to_string())))?;

        Ok(settings)
    }

    async fn update_settings(&self, dto: UpdateSettingsDto) -> Result<Settings, Error> {
        let exists = sqlx::query_scalar!(
            r#"SELECT EXISTS(SELECT 1 FROM settings WHERE user_id = $1) as "exists!""#,
            dto.user_id
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|e| Error::System(SystemError::Database(e.to_string())))?;

        if !exists {
            return Err(Error::Settings(SettingsError::NotFound));
        }

        let updated_settings = sqlx::query_as!(
            Settings,
            r#"
            UPDATE settings
            SET username = $2, first_name = $3, last_name = $4
            WHERE user_id = $1
            RETURNING user_id, username, first_name, last_name
            "#,
            dto.user_id,
            dto.new_username,
            dto.new_first_name,
            dto.new_last_name,
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| Error::System(SystemError::Database(e.to_string())))?
        .ok_or(Error::Settings(SettingsError::AlreadyExists))?;

        Ok(updated_settings)
    }

    async fn update_username(&self, dto: UpdateSettingsUsernameDto) -> Result<Settings, Error> {
        let exists = sqlx::query_scalar!(
            r#"SELECT EXISTS(SELECT 1 FROM settings WHERE user_id = $1) as "exists!""#,
            dto.user_id
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|e| Error::System(SystemError::Database(e.to_string())))?;

        if !exists {
            return Err(Error::Settings(SettingsError::NotFound));
        }

        let updated_username = sqlx::query_as!(
            Settings,
            r#"
            UPDATE settings
            SET username = $2
            WHERE user_id = $1
            RETURNING user_id, username, first_name, last_name
            "#,
            dto.user_id,
            dto.new_username,
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| Error::System(SystemError::Database(e.to_string())))?
        .ok_or(Error::Settings(SettingsError::AlreadyExists))?;

        Ok(updated_username)
    }

    async fn update_first_name(&self, dto: UpdateSettingsFirstNameDto) -> Result<Settings, Error> {
        let exists = sqlx::query_scalar!(
            r#"SELECT EXISTS(SELECT 1 FROM settings WHERE user_id = $1) as "exists!""#,
            dto.user_id
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|e| Error::System(SystemError::Database(e.to_string())))?;

        if !exists {
            return Err(Error::Settings(SettingsError::NotFound));
        }

        let updated_first_name = sqlx::query_as!(
            Settings,
            r#"
            UPDATE settings
            SET first_name = $2
            WHERE user_id = $1
            RETURNING user_id, username, first_name, last_name
            "#,
            dto.user_id,
            dto.new_first_name,
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| Error::System(SystemError::Database(e.to_string())))?
        .ok_or(Error::Settings(SettingsError::NotFound))?;

        Ok(updated_first_name)
    }

    async fn update_last_name(&self, dto: UpdateSettingsLastNameDto) -> Result<Settings, Error> {
        let exists = sqlx::query_scalar!(
            r#"SELECT EXISTS(SELECT 1 FROM settings WHERE user_id = $1) as "exists!""#,
            dto.user_id
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|e| Error::System(SystemError::Database(e.to_string())))?;

        if !exists {
            return Err(Error::Settings(SettingsError::NotFound));
        }

        let updated_last_name = sqlx::query_as!(
            Settings,
            r#"
            UPDATE settings
            SET last_name = $2
            WHERE user_id = $1
            RETURNING user_id, username, first_name, last_name
            "#,
            dto.user_id,
            dto.new_last_name,
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| Error::System(SystemError::Database(e.to_string())))?
        .ok_or(Error::Settings(SettingsError::NotFound))?;

        Ok(updated_last_name)
    }

    async fn delete_settings(&self, dto: DeleteSettingsDto) -> Result<(), Error> {
        let result = sqlx::query!(
            r#"
            DELETE FROM posts
            WHERE id = $1
            "#,
            dto.user_id
        )
        .execute(&self.pool)
        .await
        .map_err(|e| Error::System(SystemError::Database(e.to_string())))?;

        if result.rows_affected() == 0 {
            return Err(Error::Settings(SettingsError::NotFound));
        }

        Ok(())
    }
}
