use actix_web::{HttpResponse, delete, get, patch, post, put, web};

use crate::{
    application::settings::SettingsService,
    domain::entities::{errors::Error, settings::SettingsError},
    presentation::dto::settings::{
        DeleteSettingsRequest, QuerySettingsId, SettingsRequest, SettingsResponse,
        UpdateSettingsFirstNameRequest, UpdateSettingsLastNameRequest, UpdateSettingsRequest,
        UpdateSettingsUsernameRequest,
    },
};

#[get("/settings")]
pub async fn get_settings_by_user_id(
    service: web::Data<SettingsService>,
    query: web::Query<QuerySettingsId>,
) -> Result<HttpResponse, Error> {
    let post = service
        .find_by_user_id(query.into_inner().into())
        .await?
        .ok_or(Error::Settings(SettingsError::NotFound))?;

    Ok(HttpResponse::Ok().json(SettingsResponse::from(post)))
}

#[post("/settings")]
pub async fn create_settings(
    service: web::Data<SettingsService>,
    payload: web::Json<SettingsRequest>,
) -> Result<HttpResponse, Error> {
    let post = service.create_settings(payload.into_inner().into()).await?;

    Ok(HttpResponse::Created().json(SettingsResponse::from(post)))
}

#[put("/settings")]
pub async fn update_settings(
    service: web::Data<SettingsService>,
    payload: web::Json<UpdateSettingsRequest>,
) -> Result<HttpResponse, Error> {
    let post = service.update_settings(payload.into_inner().into()).await?;

    Ok(HttpResponse::Ok().json(SettingsResponse::from(post)))
}

#[patch("/settings/username")]
pub async fn update_username(
    service: web::Data<SettingsService>,
    payload: web::Json<UpdateSettingsUsernameRequest>,
) -> Result<HttpResponse, Error> {
    let post = service.update_username(payload.into_inner().into()).await?;

    Ok(HttpResponse::Ok().json(SettingsResponse::from(post)))
}

#[patch("/settings/first-name")]
pub async fn update_first_name(
    service: web::Data<SettingsService>,
    payload: web::Json<UpdateSettingsFirstNameRequest>,
) -> Result<HttpResponse, Error> {
    let settings = service
        .update_first_name(payload.into_inner().into())
        .await?;

    Ok(HttpResponse::Ok().json(SettingsResponse::from(settings)))
}

#[patch("/settings/last-name")]
pub async fn update_last_name(
    service: web::Data<SettingsService>,
    payload: web::Json<UpdateSettingsLastNameRequest>,
) -> Result<HttpResponse, Error> {
    let settings = service
        .update_last_name(payload.into_inner().into())
        .await?;

    Ok(HttpResponse::Ok().json(SettingsResponse::from(settings)))
}

#[delete("/settings")]
pub async fn delete_settings(
    service: web::Data<SettingsService>,
    payload: web::Json<DeleteSettingsRequest>,
) -> Result<HttpResponse, Error> {
    service.delete_settings(payload.into_inner().into()).await?;

    Ok(HttpResponse::NoContent().finish())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::application::settings::SettingsService;
    use crate::domain::entities::settings::Settings;
    use crate::domain::repositories::settings::SettingsRepository;
    use actix_web::{App, test, web};
    use mockall::mock;
    use mockall::predicate::*;
    use std::sync::Arc;
    use uuid::Uuid;

    mock! {
        pub SettingsRepo {}
        #[async_trait::async_trait]
        impl SettingsRepository for SettingsRepo {
            async fn create_settings(&self, dto: crate::domain::entities::settings::CreateSettingsDto) -> Result<Settings, Error>;
            async fn find_by_user_id(&self, dto: crate::domain::entities::settings::FindSettingsByUserId) -> Result<Option<Settings>, Error>;
            async fn find_by_username(&self, dto: crate::domain::entities::settings::FindSettingsByUsernameDto) -> Result<Option<Settings>, Error>;
            async fn update_settings(&self, dto: crate::domain::entities::settings::UpdateSettingsDto) -> Result<Settings, Error>;
            async fn update_username(&self, dto: crate::domain::entities::settings::UpdateSettingsUsernameDto) -> Result<Settings, Error>;
            async fn update_first_name(&self, dto: crate::domain::entities::settings::UpdateSettingsFirstNameDto) -> Result<Settings, Error>;
            async fn update_last_name(&self, dto: crate::domain::entities::settings::UpdateSettingsLastNameDto) -> Result<Settings, Error>;
            async fn delete_settings(&self, dto: crate::domain::entities::settings::DeleteSettingsDto) -> Result<(), Error>;
        }
    }

    fn mock_settings() -> Settings {
        Settings {
            user_id: Uuid::new_v4(),
            username: "john_doe".to_string(),
            first_name: Some("John".to_string()),
            last_name: Some("Doe".to_string()),
        }
    }

    #[actix_web::test]
    async fn test_get_settings_by_user_id_success() {
        let mut mock_repo = MockSettingsRepo::new();
        let settings = mock_settings();
        let user_id = settings.user_id;
        let settings_clone = settings.clone();

        mock_repo
            .expect_find_by_user_id()
            .withf(move |dto| dto.user_id == user_id)
            .once()
            .returning(move |_| Ok(Some(settings_clone.clone())));

        let service = SettingsService::new(Arc::new(mock_repo));
        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(service))
                .service(get_settings_by_user_id),
        )
        .await;

        let req = test::TestRequest::get()
            .uri(&format!("/settings?user_id={}", user_id))
            .to_request();
        let resp = test::call_service(&app, req).await;

        assert_eq!(resp.status(), 200);
        let body: SettingsResponse = test::read_body_json(resp).await;
        assert_eq!(body.username, "john_doe");
    }

    #[actix_web::test]
    async fn test_get_settings_by_user_id_not_found() {
        let mut mock_repo = MockSettingsRepo::new();
        let user_id = Uuid::new_v4();

        mock_repo
            .expect_find_by_user_id()
            .once()
            .returning(|_| Ok(None));

        let service = SettingsService::new(Arc::new(mock_repo));
        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(service))
                .service(get_settings_by_user_id),
        )
        .await;

        let req = test::TestRequest::get()
            .uri(&format!("/settings?user_id={}", user_id))
            .to_request();
        let resp = test::call_service(&app, req).await;

        assert!(resp.status().is_client_error() || resp.status().is_server_error());
    }

    #[actix_web::test]
    async fn test_create_settings_success() {
        let mut mock_repo = MockSettingsRepo::new();
        let settings = mock_settings();
        let user_id = settings.user_id;
        let settings_for_create = settings.clone();

        mock_repo
            .expect_find_by_user_id()
            .once()
            .returning(|_| Ok(None));

        mock_repo
            .expect_create_settings()
            .once()
            .returning(move |_| Ok(settings_for_create.clone()));

        let service = SettingsService::new(Arc::new(mock_repo));
        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(service))
                .service(create_settings),
        )
        .await;

        let req = test::TestRequest::post()
            .uri("/settings")
            .set_json(SettingsRequest {
                user_id,
                username: "john_doe".to_string(),
                first_name: Some("John".to_string()),
                last_name: Some("Doe".to_string()),
            })
            .to_request();
        let resp = test::call_service(&app, req).await;

        assert_eq!(resp.status(), 201);
        let body: SettingsResponse = test::read_body_json(resp).await;
        assert_eq!(body.user_id, user_id);
    }

    #[actix_web::test]
    async fn test_create_settings_already_exists() {
        let mut mock_repo = MockSettingsRepo::new();
        let settings = mock_settings();
        let clone_settings_find_user_id = settings.clone();

        mock_repo
            .expect_find_by_user_id()
            .once()
            .returning(move |_| Ok(Some(clone_settings_find_user_id.clone())));

        mock_repo.expect_create_settings().times(0);

        let service = SettingsService::new(Arc::new(mock_repo));
        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(service))
                .service(create_settings),
        )
        .await;

        let user_id = settings.user_id;
        let req = test::TestRequest::post()
            .uri("/settings")
            .set_json(SettingsRequest {
                user_id,
                username: "john_doe".to_string(),
                first_name: Some("John".to_string()),
                last_name: Some("Doe".to_string()),
            })
            .to_request();
        let resp = test::call_service(&app, req).await;

        assert!(resp.status().is_client_error() || resp.status().is_server_error());
    }

    #[actix_web::test]
    async fn test_update_settings_success() {
        let mut mock_repo = MockSettingsRepo::new();
        let settings = mock_settings();
        let user_id = settings.user_id;
        let settings_clone = settings.clone();

        mock_repo
            .expect_update_settings()
            .withf(move |dto| dto.user_id == user_id)
            .once()
            .returning(move |_| Ok(settings_clone.clone()));

        let service = SettingsService::new(Arc::new(mock_repo));
        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(service))
                .service(update_settings),
        )
        .await;

        let req = test::TestRequest::put()
            .uri("/settings")
            .set_json(UpdateSettingsRequest {
                user_id,
                new_username: "new_user".to_string(),
                new_first_name: "New".to_string(),
                new_last_name: "Name".to_string(),
            })
            .to_request();
        let resp = test::call_service(&app, req).await;

        assert_eq!(resp.status(), 200);
    }

    #[actix_web::test]
    async fn test_update_username_success() {
        let mut mock_repo = MockSettingsRepo::new();
        let settings = mock_settings();
        let user_id = settings.user_id;
        let settings_for_find = settings.clone();
        let settings_for_update = settings.clone();

        mock_repo
            .expect_find_by_user_id()
            .once()
            .returning(move |_| Ok(Some(settings_for_find.clone())));

        mock_repo
            .expect_update_username()
            .withf(move |dto| dto.user_id == user_id && dto.new_username == "new_user")
            .once()
            .returning(move |_| Ok(settings_for_update.clone()));

        let service = SettingsService::new(Arc::new(mock_repo));
        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(service))
                .service(update_username),
        )
        .await;

        let req = test::TestRequest::patch()
            .uri("/settings/username")
            .set_json(UpdateSettingsUsernameRequest {
                user_id,
                new_username: "  new_user  ".to_string(),
            })
            .to_request();
        let resp = test::call_service(&app, req).await;

        assert_eq!(resp.status(), 200);
    }

    #[actix_web::test]
    async fn test_update_first_name_success() {
        let mut mock_repo = MockSettingsRepo::new();
        let settings = mock_settings();
        let user_id = settings.user_id;
        let settings_for_find = settings.clone();
        let settings_for_update = settings.clone();

        mock_repo
            .expect_find_by_user_id()
            .once()
            .returning(move |_| Ok(Some(settings_for_find.clone())));

        mock_repo
            .expect_update_first_name()
            .withf(move |dto| dto.user_id == user_id && dto.new_first_name == "NewName")
            .once()
            .returning(move |_| Ok(settings_for_update.clone()));

        let service = SettingsService::new(Arc::new(mock_repo));
        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(service))
                .service(update_first_name),
        )
        .await;

        let req = test::TestRequest::patch()
            .uri("/settings/first-name")
            .set_json(UpdateSettingsFirstNameRequest {
                user_id,
                new_first_name: "  NewName  ".to_string(),
            })
            .to_request();
        let resp = test::call_service(&app, req).await;

        assert_eq!(resp.status(), 200);
    }

    #[actix_web::test]
    async fn test_update_last_name_success() {
        let mut mock_repo = MockSettingsRepo::new();
        let settings = mock_settings();
        let user_id = settings.user_id;
        let settings_for_find = settings.clone();
        let settings_for_update = settings.clone();

        mock_repo
            .expect_find_by_user_id()
            .once()
            .returning(move |_| Ok(Some(settings_for_find.clone())));

        mock_repo
            .expect_update_last_name()
            .withf(move |dto| dto.user_id == user_id && dto.new_last_name == "NewLastName")
            .once()
            .returning(move |_| Ok(settings_for_update.clone()));

        let service = SettingsService::new(Arc::new(mock_repo));
        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(service))
                .service(update_last_name),
        )
        .await;

        let req = test::TestRequest::patch()
            .uri("/settings/last-name")
            .set_json(UpdateSettingsLastNameRequest {
                user_id,
                new_last_name: "  NewLastName  ".to_string(),
            })
            .to_request();
        let resp = test::call_service(&app, req).await;

        assert_eq!(resp.status(), 200);
    }

    #[actix_web::test]
    async fn test_delete_settings_success() {
        let mut mock_repo = MockSettingsRepo::new();
        let user_id = Uuid::new_v4();

        mock_repo
            .expect_delete_settings()
            .withf(move |dto| dto.user_id == user_id)
            .once()
            .returning(|_| Ok(()));

        let service = SettingsService::new(Arc::new(mock_repo));
        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(service))
                .service(delete_settings),
        )
        .await;

        let req = test::TestRequest::delete()
            .uri("/settings")
            .set_json(DeleteSettingsRequest { user_id })
            .to_request();
        let resp = test::call_service(&app, req).await;

        assert_eq!(resp.status(), 204);
    }
}
