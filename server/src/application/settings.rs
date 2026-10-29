use std::sync::Arc;
use validator::Validate;

use crate::domain::{
    entities::{
        errors::Error,
        settings::{
            CreateSettingsDto, DeleteSettingsDto, FindSettingsByUserId, FindSettingsByUsernameDto,
            Settings, SettingsError, UpdateSettingsDto, UpdateSettingsFirstNameDto,
            UpdateSettingsLastNameDto, UpdateSettingsUsernameDto,
        },
    },
    repositories::settings::SettingsRepository,
};

pub struct SettingsService {
    repository: Arc<dyn SettingsRepository>,
}

impl SettingsService {
    pub fn new(repository: Arc<dyn SettingsRepository>) -> Self {
        Self { repository }
    }

    pub async fn create_settings(&self, dto: CreateSettingsDto) -> Result<Settings, Error> {
        let settings = Settings::new(dto.clone())?;

        if self
            .repository
            .find_by_user_id(FindSettingsByUserId {
                user_id: settings.user_id,
            })
            .await?
            .is_some()
        {
            return Err(SettingsError::AlreadyExists.into());
        }

        let created_settings = self.repository.create_settings(dto).await?;

        Ok(created_settings)
    }

    pub async fn find_by_username(
        &self,
        dto: FindSettingsByUsernameDto,
    ) -> Result<Option<Settings>, Error> {
        self.repository.find_by_username(dto).await
    }

    pub async fn find_by_user_id(
        &self,
        dto: FindSettingsByUserId,
    ) -> Result<Option<Settings>, Error> {
        self.repository.find_by_user_id(dto).await
    }

    pub async fn update_settings(&self, dto: UpdateSettingsDto) -> Result<Settings, Error> {
        self.repository.update_settings(dto).await
    }

    pub async fn update_first_name(
        &self,
        dto: UpdateSettingsFirstNameDto,
    ) -> Result<Settings, Error> {
        let normalized = dto.new_first_name.trim().to_owned();
        let settings = self
            .find_by_user_id(FindSettingsByUserId {
                user_id: dto.user_id,
            })
            .await?
            .ok_or(SettingsError::NotFound)?;

        settings
            .validate()
            .map_err(|_| SettingsError::InvalidFirstNameLength)?;

        self.repository
            .update_first_name(UpdateSettingsFirstNameDto {
                user_id: dto.user_id,
                new_first_name: normalized,
            })
            .await
    }

    pub async fn update_last_name(
        &self,
        dto: UpdateSettingsLastNameDto,
    ) -> Result<Settings, Error> {
        let normalized = dto.new_last_name.trim().to_owned();
        let settings = self
            .find_by_user_id(FindSettingsByUserId {
                user_id: dto.user_id,
            })
            .await?
            .ok_or(SettingsError::NotFound)?;

        settings
            .validate()
            .map_err(|_| SettingsError::InvalidLastNameLength)?;

        self.repository
            .update_last_name(UpdateSettingsLastNameDto {
                user_id: dto.user_id,
                new_last_name: normalized,
            })
            .await
    }

    pub async fn update_username(&self, dto: UpdateSettingsUsernameDto) -> Result<Settings, Error> {
        let normalized = dto.new_username.trim().to_owned();
        let settings = self
            .find_by_user_id(FindSettingsByUserId {
                user_id: dto.user_id,
            })
            .await?
            .ok_or(SettingsError::NotFound)?;

        settings
            .validate()
            .map_err(|_| SettingsError::InvalidUsernameLength)?;

        self.repository
            .update_username(UpdateSettingsUsernameDto {
                user_id: dto.user_id,
                new_username: normalized,
            })
            .await
    }

    pub async fn delete_post(&self, dto: DeleteSettingsDto) -> Result<(), Error> {
        self.repository.delete_settings(dto).await
    }
}
