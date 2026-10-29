use crate::domain::entities::{
    errors::Error,
    settings::{
        CreateSettingsDto, DeleteSettingsDto, FindSettingsByUserId, FindSettingsByUsernameDto,
        Settings, UpdateSettingsDto, UpdateSettingsFirstNameDto, UpdateSettingsLastNameDto,
        UpdateSettingsUsernameDto,
    },
};
use async_trait::async_trait;

#[async_trait]
pub trait SettingsRepository: Send + Sync {
    async fn create_settings(&self, dto: CreateSettingsDto) -> Result<Settings, Error>;

    async fn find_by_user_id(&self, dto: FindSettingsByUserId) -> Result<Option<Settings>, Error>;

    async fn find_by_username(
        &self,
        dto: FindSettingsByUsernameDto,
    ) -> Result<Option<Settings>, Error>;

    async fn update_settings(&self, dto: UpdateSettingsDto) -> Result<Settings, Error>;

    async fn update_first_name(&self, dto: UpdateSettingsFirstNameDto) -> Result<Settings, Error>;

    async fn update_last_name(&self, dto: UpdateSettingsLastNameDto) -> Result<Settings, Error>;

    async fn update_username(&self, dto: UpdateSettingsUsernameDto) -> Result<Settings, Error>;

    async fn delete_settings(&self, dto: DeleteSettingsDto) -> Result<(), Error>;
}
