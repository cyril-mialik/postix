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

    pub async fn delete_settings(&self, dto: DeleteSettingsDto) -> Result<(), Error> {
        self.repository.delete_settings(dto).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::repositories::settings::SettingsRepository;
    use mockall::mock;
    use mockall::predicate::*;
    use uuid::Uuid;

    mock! {
        pub SettingsRepo {}

        #[async_trait::async_trait]
        impl SettingsRepository for SettingsRepo {
            async fn create_settings(&self, dto: CreateSettingsDto) -> Result<Settings, Error>;
            async fn find_by_user_id(&self, dto: FindSettingsByUserId) -> Result<Option<Settings>, Error>;
            async fn find_by_username(&self, dto: FindSettingsByUsernameDto) -> Result<Option<Settings>, Error>;
            async fn update_settings(&self, dto: UpdateSettingsDto) -> Result<Settings, Error>;
            async fn update_username(&self, dto: UpdateSettingsUsernameDto) -> Result<Settings, Error>;
            async fn update_first_name(&self, dto: UpdateSettingsFirstNameDto) -> Result<Settings, Error>;
            async fn update_last_name(&self, dto: UpdateSettingsLastNameDto) -> Result<Settings, Error>;
            async fn delete_settings(&self, dto: DeleteSettingsDto) -> Result<(), Error>;
        }
    }

    fn mock_settings(user_id: Uuid) -> Settings {
        Settings {
            user_id,
            username: "john_doe".to_string(),
            first_name: Some("John".to_string()),
            last_name: Some("Doe".to_string()),
        }
    }

    fn mock_create_dto(user_id: Uuid) -> CreateSettingsDto {
        CreateSettingsDto {
            user_id,
            username: "john_doe".to_string(),
            first_name: Some("John".to_string()),
            last_name: Some("Doe".to_string()),
        }
    }

    #[tokio::test]
    async fn test_create_settings_success() {
        let mut mock_repo = MockSettingsRepo::new();
        let user_id = Uuid::new_v4();
        let settings = mock_settings(user_id);
        let settings_clone = settings.clone();

        mock_repo
            .expect_find_by_user_id()
            .with(eq(FindSettingsByUserId { user_id }))
            .times(1)
            .returning(|_| Ok(None));

        mock_repo
            .expect_create_settings()
            .withf(move |dto| dto.user_id == user_id)
            .times(1)
            .returning(move |_| Ok(settings_clone.clone()));

        let service = SettingsService::new(Arc::new(mock_repo));
        let result = service.create_settings(mock_create_dto(user_id)).await;

        assert!(result.is_ok());
        assert_eq!(result.unwrap().username, "john_doe");
    }

    #[tokio::test]
    async fn test_create_settings_already_exists() {
        let mut mock_repo = MockSettingsRepo::new();
        let user_id = Uuid::new_v4();
        let settings = mock_settings(user_id);

        mock_repo
            .expect_find_by_user_id()
            .once()
            .returning(move |_| Ok(Some(settings.clone())));

        mock_repo.expect_create_settings().never();

        let service = SettingsService::new(Arc::new(mock_repo));
        let result = service.create_settings(mock_create_dto(user_id)).await;

        assert!(matches!(
            result,
            Err(Error::Settings(SettingsError::AlreadyExists))
        ));
    }

    #[tokio::test]
    async fn test_create_settings_invalid_username() {
        let mut mock_repo = MockSettingsRepo::new();
        let user_id = Uuid::new_v4();

        mock_repo.expect_find_by_user_id().never();
        mock_repo.expect_create_settings().never();

        let service = SettingsService::new(Arc::new(mock_repo));
        let dto = CreateSettingsDto {
            user_id,
            username: "ab".to_string(),
            first_name: Some("John".to_string()),
            last_name: Some("Doe".to_string()),
        };
        let result = service.create_settings(dto).await;

        assert!(matches!(
            result,
            Err(Error::Settings(SettingsError::InvalidUsernameLength))
        ));
    }

    #[tokio::test]
    async fn test_find_by_user_id_found() {
        let mut mock_repo = MockSettingsRepo::new();
        let user_id = Uuid::new_v4();
        let settings = mock_settings(user_id);
        let settings_clone = settings.clone();

        mock_repo
            .expect_find_by_user_id()
            .with(eq(FindSettingsByUserId { user_id }))
            .once()
            .returning(move |_| Ok(Some(settings_clone.clone())));

        let service = SettingsService::new(Arc::new(mock_repo));
        let result = service
            .find_by_user_id(FindSettingsByUserId { user_id })
            .await;

        assert!(result.is_ok());
        assert_eq!(result.unwrap().unwrap().user_id, user_id);
    }

    #[tokio::test]
    async fn test_find_by_user_id_not_found() {
        let mut mock_repo = MockSettingsRepo::new();
        let user_id = Uuid::new_v4();

        mock_repo
            .expect_find_by_user_id()
            .once()
            .returning(|_| Ok(None));

        let service = SettingsService::new(Arc::new(mock_repo));
        let result = service
            .find_by_user_id(FindSettingsByUserId { user_id })
            .await;

        assert!(result.is_ok());
        assert!(result.unwrap().is_none());
    }

    #[tokio::test]
    async fn test_find_by_username() {
        let mut mock_repo = MockSettingsRepo::new();
        let user_id = Uuid::new_v4();
        let settings = mock_settings(user_id);
        let settings_clone = settings.clone();

        mock_repo
            .expect_find_by_username()
            .with(eq(FindSettingsByUsernameDto {
                username: "john_doe".to_string(),
            }))
            .once()
            .returning(move |_| Ok(Some(settings_clone.clone())));

        let service = SettingsService::new(Arc::new(mock_repo));
        let result = service
            .find_by_username(FindSettingsByUsernameDto {
                username: "john_doe".to_string(),
            })
            .await;

        assert!(result.is_ok());
        assert!(result.unwrap().is_some());
    }

    #[tokio::test]
    async fn test_update_settings_success() {
        let mut mock_repo = MockSettingsRepo::new();
        let user_id = Uuid::new_v4();
        let settings = mock_settings(user_id);
        let settings_clone = settings.clone();

        mock_repo
            .expect_update_settings()
            .withf(move |dto| dto.user_id == user_id)
            .once()
            .returning(move |_| Ok(settings_clone.clone()));

        let service = SettingsService::new(Arc::new(mock_repo));
        let result = service
            .update_settings(UpdateSettingsDto {
                user_id,
                new_username: "new_user".to_string(),
                new_first_name: "New".to_string(),
                new_last_name: "Name".to_string(),
            })
            .await;

        assert!(result.is_ok());
        assert_eq!(result.unwrap().username, "john_doe");
    }

    #[tokio::test]
    async fn test_update_first_name_success() {
        let mut mock_repo = MockSettingsRepo::new();
        let user_id = Uuid::new_v4();
        let settings = mock_settings(user_id);
        let settings_clone = settings.clone();
        let settings_for_update = settings.clone();

        mock_repo
            .expect_find_by_user_id()
            .once()
            .returning(move |_| Ok(Some(settings_clone.clone())));

        mock_repo
            .expect_update_first_name()
            .withf(move |dto| dto.user_id == user_id && dto.new_first_name == "NewName")
            .once()
            .returning(move |_| Ok(settings_for_update.clone()));

        let service = SettingsService::new(Arc::new(mock_repo));
        let result = service
            .update_first_name(UpdateSettingsFirstNameDto {
                user_id,
                new_first_name: "  NewName  ".to_string(),
            })
            .await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_update_first_name_not_found() {
        let mut mock_repo = MockSettingsRepo::new();
        let user_id = Uuid::new_v4();

        mock_repo
            .expect_find_by_user_id()
            .once()
            .returning(|_| Ok(None));

        mock_repo.expect_update_first_name().never();

        let service = SettingsService::new(Arc::new(mock_repo));
        let result = service
            .update_first_name(UpdateSettingsFirstNameDto {
                user_id,
                new_first_name: "NewName".to_string(),
            })
            .await;

        assert!(matches!(
            result,
            Err(Error::Settings(SettingsError::NotFound))
        ));
    }

    #[tokio::test]
    async fn test_update_last_name_success() {
        let mut mock_repo = MockSettingsRepo::new();
        let user_id = Uuid::new_v4();
        let settings = mock_settings(user_id);
        let settings_clone = settings.clone();
        let settings_for_update = settings.clone();

        mock_repo
            .expect_find_by_user_id()
            .once()
            .returning(move |_| Ok(Some(settings_clone.clone())));

        mock_repo
            .expect_update_last_name()
            .withf(move |dto| dto.user_id == user_id && dto.new_last_name == "NewLastName")
            .once()
            .returning(move |_| Ok(settings_for_update.clone()));

        let service = SettingsService::new(Arc::new(mock_repo));
        let result = service
            .update_last_name(UpdateSettingsLastNameDto {
                user_id,
                new_last_name: "  NewLastName  ".to_string(),
            })
            .await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_update_username_success() {
        let mut mock_repo = MockSettingsRepo::new();
        let user_id = Uuid::new_v4();
        let settings = mock_settings(user_id);
        let settings_clone = settings.clone();
        let settings_for_update = settings.clone();

        mock_repo
            .expect_find_by_user_id()
            .once()
            .returning(move |_| Ok(Some(settings_clone.clone())));

        mock_repo
            .expect_update_username()
            .withf(move |dto| dto.user_id == user_id && dto.new_username == "new_username")
            .once()
            .returning(move |_| Ok(settings_for_update.clone()));

        let service = SettingsService::new(Arc::new(mock_repo));
        let result = service
            .update_username(UpdateSettingsUsernameDto {
                user_id,
                new_username: "  new_username  ".to_string(),
            })
            .await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_update_username_not_found() {
        let mut mock_repo = MockSettingsRepo::new();
        let user_id = Uuid::new_v4();

        mock_repo
            .expect_find_by_user_id()
            .once()
            .returning(|_| Ok(None));

        mock_repo.expect_update_username().never();

        let service = SettingsService::new(Arc::new(mock_repo));
        let result = service
            .update_username(UpdateSettingsUsernameDto {
                user_id,
                new_username: "new_username".to_string(),
            })
            .await;

        assert!(matches!(
            result,
            Err(Error::Settings(SettingsError::NotFound))
        ));
    }

    #[tokio::test]
    async fn test_delete_settings_success() {
        let mut mock_repo = MockSettingsRepo::new();
        let user_id = Uuid::new_v4();

        mock_repo
            .expect_delete_settings()
            .with(eq(DeleteSettingsDto { user_id }))
            .once()
            .returning(|_| Ok(()));

        let service = SettingsService::new(Arc::new(mock_repo));
        let result = service.delete_settings(DeleteSettingsDto { user_id }).await;

        assert!(result.is_ok());
    }
}
