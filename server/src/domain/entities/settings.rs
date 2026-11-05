use serde::{Deserialize, Serialize};
use thiserror::Error;
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CreateSettingsDto {
    pub user_id: Uuid,
    pub username: String,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UpdateSettingsDto {
    pub user_id: Uuid,
    pub new_username: String,
    pub new_first_name: String,
    pub new_last_name: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UpdateSettingsUsernameDto {
    pub user_id: Uuid,
    pub new_username: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UpdateSettingsFirstNameDto {
    pub user_id: Uuid,
    pub new_first_name: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UpdateSettingsLastNameDto {
    pub user_id: Uuid,
    pub new_last_name: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FindSettingsByUserId {
    pub user_id: Uuid,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FindSettingsByUsernameDto {
    pub username: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
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

#[derive(Debug, PartialEq, Error)]
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

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("username", "username")]
    fn test_settings_new_valid_username(
        #[case] username: String,
        #[case] expected_username: String,
    ) {
        let settings = Settings::new(CreateSettingsDto {
            user_id: Uuid::new_v4(),
            username,
            first_name: Some("first_name".to_owned()),
            last_name: Some("last_name".to_owned()),
        })
        .unwrap();

        assert_eq!(settings.username, expected_username);
    }

    #[rstest]
    #[case("u", SettingsError::InvalidUsernameLength)]
    #[case("u".repeat(129), SettingsError::InvalidUsernameLength)]
    fn test_settings_new_invalid_username(
        #[case] username: String,
        #[case] expected_error: SettingsError,
    ) {
        let settings = Settings::new(CreateSettingsDto {
            user_id: Uuid::new_v4(),
            username,
            first_name: Some("first_name".to_owned()),
            last_name: Some("last_name".to_owned()),
        });

        assert_eq!(settings.unwrap_err(), expected_error);
    }

    #[rstest]
    #[case("firstname", "firstname")]
    fn test_settings_new_valid_first_name(
        #[case] first_name: String,
        #[case] expected_first_name: String,
    ) {
        let settings = Settings::new(CreateSettingsDto {
            user_id: Uuid::new_v4(),
            username: "username".to_owned(),
            first_name: Some(first_name),
            last_name: Some("last_name".to_owned()),
        })
        .unwrap();

        assert_eq!(settings.first_name, Some(expected_first_name));
    }

    #[rstest]
    #[case("f", SettingsError::InvalidFirstNameLength)]
    #[case("f".repeat(129), SettingsError::InvalidFirstNameLength)]
    fn test_settings_new_invalid_first_name(
        #[case] first_name: String,
        #[case] expected_error: SettingsError,
    ) {
        let settings = Settings::new(CreateSettingsDto {
            user_id: Uuid::new_v4(),
            username: "username".to_owned(),
            first_name: Some(first_name),
            last_name: Some("last_name".to_owned()),
        });

        assert_eq!(settings.unwrap_err(), expected_error);
    }

    #[rstest]
    #[case("lastname", "lastname")]
    fn test_settings_new_valid_last_name(
        #[case] last_name: String,
        #[case] expected_last_name: String,
    ) {
        let settings = Settings::new(CreateSettingsDto {
            user_id: Uuid::new_v4(),
            username: "username".to_owned(),
            first_name: Some("first_name".to_owned()),
            last_name: Some(last_name),
        })
        .unwrap();

        assert_eq!(settings.last_name, Some(expected_last_name));
    }

    #[rstest]
    #[case("l", SettingsError::InvalidLastNameLength)]
    #[case("l".repeat(129), SettingsError::InvalidLastNameLength)]
    fn test_settings_new_invalid_last_name(
        #[case] last_name: String,
        #[case] expected_error: SettingsError,
    ) {
        let settings = Settings::new(CreateSettingsDto {
            user_id: Uuid::new_v4(),
            username: "username".to_owned(),
            first_name: Some("first_name".to_owned()),
            last_name: Some(last_name),
        });

        assert_eq!(settings.unwrap_err(), expected_error);
    }
}
