use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::domain::entities::settings::{
    CreateSettingsDto, DeleteSettingsDto, FindSettingsByUserId, FindSettingsByUsernameDto,
    Settings, UpdateSettingsDto, UpdateSettingsFirstNameDto, UpdateSettingsLastNameDto,
    UpdateSettingsUsernameDto,
};

#[derive(Debug, Deserialize)]
pub struct SettingsRequest {
    pub user_id: Uuid,
    pub username: String,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
}

impl Into<CreateSettingsDto> for SettingsRequest {
    fn into(self) -> CreateSettingsDto {
        CreateSettingsDto {
            user_id: self.user_id,
            username: self.username,
            first_name: self.first_name,
            last_name: self.last_name,
        }
    }
}

#[derive(Debug, Serialize, Clone)]
pub struct SettingsResponse {
    pub user_id: Uuid,
    pub username: String,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
}

impl From<Settings> for SettingsResponse {
    fn from(post: Settings) -> Self {
        Self {
            user_id: post.user_id,
            username: post.username,
            first_name: post.first_name,
            last_name: post.last_name,
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct QuerySettingsId {
    pub user_id: Uuid,
}

impl Into<FindSettingsByUserId> for QuerySettingsId {
    fn into(self) -> FindSettingsByUserId {
        FindSettingsByUserId {
            user_id: self.user_id,
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct QuerySettingsUsername {
    pub username: String,
}

impl Into<FindSettingsByUsernameDto> for QuerySettingsUsername {
    fn into(self) -> FindSettingsByUsernameDto {
        FindSettingsByUsernameDto {
            username: self.username,
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct UpdateSettingsRequest {
    pub user_id: Uuid,
    pub new_username: String,
    pub new_first_name: String,
    pub new_last_name: String,
}

impl Into<UpdateSettingsDto> for UpdateSettingsRequest {
    fn into(self) -> UpdateSettingsDto {
        UpdateSettingsDto {
            user_id: self.user_id,
            new_username: self.new_username,
            new_first_name: self.new_first_name,
            new_last_name: self.new_last_name,
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct UpdateSettingsUsernameRequest {
    pub user_id: Uuid,
    pub new_username: String,
}

impl Into<UpdateSettingsUsernameDto> for UpdateSettingsUsernameRequest {
    fn into(self) -> UpdateSettingsUsernameDto {
        UpdateSettingsUsernameDto {
            user_id: self.user_id,
            new_username: self.new_username,
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct UpdateSettingsFirstNameRequest {
    pub user_id: Uuid,
    pub new_first_name: String,
}

impl Into<UpdateSettingsFirstNameDto> for UpdateSettingsFirstNameRequest {
    fn into(self) -> UpdateSettingsFirstNameDto {
        UpdateSettingsFirstNameDto {
            user_id: self.user_id,
            new_first_name: self.new_first_name,
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct UpdateSettingsLastNameRequest {
    pub user_id: Uuid,
    pub new_last_name: String,
}

impl Into<UpdateSettingsLastNameDto> for UpdateSettingsLastNameRequest {
    fn into(self) -> UpdateSettingsLastNameDto {
        UpdateSettingsLastNameDto {
            user_id: self.user_id,
            new_last_name: self.new_last_name,
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct DeleteSettingsRequest {
    pub user_id: Uuid,
}

impl Into<DeleteSettingsDto> for DeleteSettingsRequest {
    fn into(self) -> DeleteSettingsDto {
        DeleteSettingsDto {
            user_id: self.user_id,
        }
    }
}
