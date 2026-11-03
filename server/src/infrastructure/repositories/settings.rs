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
            DELETE FROM settings
            WHERE user_id = $1
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

#[cfg(test)]
mod tests {
    use super::*;
    use sqlx::PgPool;
    use uuid::Uuid;

    async fn create_test_user(pool: &PgPool) -> Uuid {
        let user_id = Uuid::new_v4();
        sqlx::query(
            r#"
            INSERT INTO users (id, email, password)
            VALUES ($1, $2, $3)
            "#,
        )
        .bind(user_id)
        .bind(format!("test_{}@example.com", user_id))
        .bind("hashed_password")
        .execute(pool)
        .await
        .expect("Failed to create test user");

        user_id
    }

    #[sqlx::test]
    async fn test_create_settings(pool: PgPool) -> sqlx::Result<()> {
        let user_id = create_test_user(&pool).await;
        let repo = PostgresSettingsRepository::new(pool);

        let dto = CreateSettingsDto {
            user_id,
            username: "john_doe".into(),
            first_name: Some("John".into()),
            last_name: Some("Doe".into()),
        };

        let settings = repo
            .create_settings(dto)
            .await
            .expect("Failed to create settings");

        assert_eq!(settings.user_id, user_id);
        assert_eq!(settings.username, "john_doe");
        assert_eq!(settings.first_name, Some("John".into()));
        assert_eq!(settings.last_name, Some("Doe".into()));

        Ok(())
    }

    #[sqlx::test]
    async fn test_create_settings_invalid_length(pool: PgPool) -> sqlx::Result<()> {
        let user_id = create_test_user(&pool).await;
        let repo = PostgresSettingsRepository::new(pool);

        let dto = CreateSettingsDto {
            user_id,
            username: "ab".to_string(),
            first_name: Some("John".into()),
            last_name: Some("Doe".into()),
        };

        let result = repo.create_settings(dto).await;
        assert!(matches!(
            result,
            Err(Error::Settings(SettingsError::InvalidUsernameLength))
        ));

        Ok(())
    }

    #[sqlx::test]
    async fn test_find_by_user_id(pool: PgPool) -> sqlx::Result<()> {
        let user_id = create_test_user(&pool).await;
        let repo = PostgresSettingsRepository::new(pool);

        repo.create_settings(CreateSettingsDto {
            user_id,
            username: "find_me".into(),
            first_name: Some("Find".into()),
            last_name: Some("Doe".into()),
        })
        .await
        .unwrap();

        let found = repo
            .find_by_user_id(FindSettingsByUserId { user_id })
            .await
            .unwrap();
        assert!(found.is_some());
        assert_eq!(found.unwrap().username, "find_me");

        let not_found = repo
            .find_by_user_id(FindSettingsByUserId {
                user_id: Uuid::new_v4(),
            })
            .await
            .unwrap();
        assert!(not_found.is_none());

        Ok(())
    }

    #[sqlx::test]
    async fn test_find_by_username(pool: PgPool) -> sqlx::Result<()> {
        let user_id = create_test_user(&pool).await;
        let repo = PostgresSettingsRepository::new(pool);

        repo.create_settings(CreateSettingsDto {
            user_id,
            username: "unique_name".into(),
            first_name: Some("First".into()),
            last_name: Some("Last".into()),
        })
        .await
        .unwrap();

        let found = repo
            .find_by_username(FindSettingsByUsernameDto {
                username: "unique_name".into(),
            })
            .await
            .unwrap();
        assert!(found.is_some());
        assert_eq!(found.unwrap().user_id, user_id);

        let not_found = repo
            .find_by_username(FindSettingsByUsernameDto {
                username: "no_such".into(),
            })
            .await
            .unwrap();
        assert!(not_found.is_none());
        Ok(())
    }

    #[sqlx::test]
    async fn test_update_settings(pool: PgPool) -> sqlx::Result<()> {
        let user_id = create_test_user(&pool).await;
        let repo = PostgresSettingsRepository::new(pool);

        repo.create_settings(CreateSettingsDto {
            user_id,
            username: "old_username".into(),
            first_name: Some("Old".into()),
            last_name: Some("Name".into()),
        })
        .await
        .unwrap();

        let updated = repo
            .update_settings(UpdateSettingsDto {
                user_id,
                new_username: "new_username".into(),
                new_first_name: "New".into(),
                new_last_name: "Name".into(),
            })
            .await
            .expect("Failed to update");

        assert_eq!(updated.username, "new_username");
        assert_eq!(updated.first_name, Some("New".into()));
        assert_eq!(updated.last_name, Some("Name".into()));
        Ok(())
    }

    #[sqlx::test]
    async fn test_update_settings_not_found(pool: PgPool) -> sqlx::Result<()> {
        let user_id = create_test_user(&pool).await;
        let repo = PostgresSettingsRepository::new(pool);

        let result = repo
            .update_settings(UpdateSettingsDto {
                user_id,
                new_username: "x".into(),
                new_first_name: "y".into(),
                new_last_name: "z".into(),
            })
            .await;

        assert!(matches!(
            result,
            Err(Error::Settings(SettingsError::NotFound))
        ));
        Ok(())
    }

    #[sqlx::test]
    async fn test_update_username(pool: PgPool) -> sqlx::Result<()> {
        let user_id = create_test_user(&pool).await;
        let repo = PostgresSettingsRepository::new(pool);

        repo.create_settings(CreateSettingsDto {
            user_id,
            username: "old_user".into(),
            first_name: Some("First".into()),
            last_name: Some("Last".into()),
        })
        .await
        .unwrap();

        let updated = repo
            .update_username(UpdateSettingsUsernameDto {
                user_id,
                new_username: "new_user".into(),
            })
            .await
            .expect("Failed to update username");

        assert_eq!(updated.username, "new_user");
        assert_eq!(updated.first_name, Some("First".into()));
        Ok(())
    }

    #[sqlx::test]
    async fn test_update_first_name(pool: PgPool) -> sqlx::Result<()> {
        let user_id = create_test_user(&pool).await;
        let repo = PostgresSettingsRepository::new(pool);

        repo.create_settings(CreateSettingsDto {
            user_id,
            username: "user".into(),
            first_name: Some("Old".into()),
            last_name: Some("Last".into()),
        })
        .await
        .unwrap();

        let updated = repo
            .update_first_name(UpdateSettingsFirstNameDto {
                user_id,
                new_first_name: "New".into(),
            })
            .await
            .expect("Failed to update first name");

        assert_eq!(updated.first_name, Some("New".into()));
        assert_eq!(updated.username, "user");

        Ok(())
    }

    #[sqlx::test]
    async fn test_update_last_name(pool: PgPool) -> sqlx::Result<()> {
        let user_id = create_test_user(&pool).await;
        let repo = PostgresSettingsRepository::new(pool);

        repo.create_settings(CreateSettingsDto {
            user_id,
            username: "user".into(),
            first_name: Some("First".into()),
            last_name: Some("Old".into()),
        })
        .await
        .unwrap();

        let updated = repo
            .update_last_name(UpdateSettingsLastNameDto {
                user_id,
                new_last_name: "New".into(),
            })
            .await
            .expect("Failed to update last name");

        assert_eq!(updated.last_name, Some("New".into()));
        assert_eq!(updated.first_name, Some("First".into()));

        Ok(())
    }

    #[sqlx::test]
    async fn test_delete_settings(pool: PgPool) -> sqlx::Result<()> {
        let user_id = create_test_user(&pool).await;
        let repo = PostgresSettingsRepository::new(pool);

        repo.create_settings(CreateSettingsDto {
            user_id,
            username: "to_delete".into(),
            first_name: Some("Delete".into()),
            last_name: Some("Meow".into()),
        })
        .await
        .unwrap();

        repo.delete_settings(DeleteSettingsDto { user_id })
            .await
            .expect("Failed to delete");

        let found = repo
            .find_by_user_id(FindSettingsByUserId { user_id })
            .await
            .unwrap();
        assert!(found.is_none());

        let result = repo.delete_settings(DeleteSettingsDto { user_id }).await;
        assert!(matches!(
            result,
            Err(Error::Settings(SettingsError::NotFound))
        ));
        Ok(())
    }

    #[sqlx::test]
    async fn test_update_username_duplicate(pool: PgPool) -> sqlx::Result<()> {
        let user1 = create_test_user(&pool).await;
        let user2 = create_test_user(&pool).await;
        let repo = PostgresSettingsRepository::new(pool);

        repo.create_settings(CreateSettingsDto {
            user_id: user1,
            username: "taken".into(),
            first_name: Some("Ann".into()),
            last_name: Some("Smith".into()),
        })
        .await
        .unwrap();

        repo.create_settings(CreateSettingsDto {
            user_id: user2,
            username: "free".into(),
            first_name: Some("Bob".into()),
            last_name: Some("Jones".into()),
        })
        .await
        .unwrap();

        let result = repo
            .update_username(UpdateSettingsUsernameDto {
                user_id: user2,
                new_username: "taken".into(),
            })
            .await;

        assert!(result.is_err());

        Ok(())
    }
}
