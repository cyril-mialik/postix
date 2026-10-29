use serde::{Deserialize, Serialize};
use thiserror::Error;
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Clone)]
pub struct CreateSettingsDto {
    pub user_id: Uuid,
    pub username: String,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
}

#[derive(Debug, Clone)]
pub struct UpdateSettingsDto {
    pub user_id: Uuid,
    pub new_username: String,
    pub new_first_name: String,
    pub new_last_name: String,
}

#[derive(Debug, Clone)]
pub struct UpdateSettingsUsernameDto {
    pub user_id: Uuid,
    pub new_username: String,
}

#[derive(Debug, Clone)]
pub struct UpdateSettingsFirstNameDto {
    pub user_id: Uuid,
    pub new_first_name: String,
}

#[derive(Debug, Clone)]
pub struct UpdateSettingsLastNameDto {
    pub user_id: Uuid,
    pub new_last_name: String,
}

#[derive(Debug, Clone)]
pub struct FindSettingsByUserId {
    pub user_id: Uuid,
}

#[derive(Debug, Clone)]
pub struct FindSettingsByUsernameDto {
    pub username: String,
}

#[derive(Debug, Clone)]
pub struct DeleteSettingsDto {
    pub user_id: Uuid,
}

#[derive(Serialize, Deserialize, Debug, Clone, Validate)]
pub struct Settings {
    pub user_id: Uuid,

    #[validate(length(min = 3, max = 128))]
    pub username: String,

    #[validate(length(min = 3, max = 128))]
    pub first_name: Option<String>,
    #[validate(length(min = 3, max = 128))]
    pub last_name: Option<String>,
}

impl Settings {
    pub fn new(dto: CreateSettingsDto) -> Result<Self, SettingsError> {
        let settings = Self {
            user_id: dto.user_id,
            username: dto.username.trim().to_owned(),
            first_name: dto.first_name.map(|s| s.trim().to_owned()),
            last_name: dto.last_name.map(|s| s.trim().to_owned()),
        };

        settings.validate().map_err(|e| {
            let field_errors = e.field_errors();

            if field_errors.contains_key("username") {
                return SettingsError::InvalidUsernameLength;
            }

            if field_errors.contains_key("first_name") {
                return SettingsError::InvalidFirstNameLength;
            }

            if field_errors.contains_key("last_name") {
                return SettingsError::InvalidLastNameLength;
            }

            SettingsError::NotFound
        })?;

        Ok(settings)
    }
}

#[derive(Debug, Error)]
pub enum SettingsError {
    #[error("Настройки для этого пользователя уже существуют")]
    AlreadyExists,

    #[error("Пользователь не найден")]
    NotFound,

    #[error("Неверная длина username (3–128)")]
    InvalidUsernameLength,

    #[error("Неверная длина имени (3–128)")]
    InvalidFirstNameLength,

    #[error("Неверная длина фамилии (3–128)")]
    InvalidLastNameLength,
}
