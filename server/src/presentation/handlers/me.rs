use actix_web::{HttpResponse, delete, get, patch, web};

use crate::{
    application::user::UserService,
    domain::entities::{errors::Error, user::UserError},
    presentation::{
        dto::user::{UpdateEmailRequest, UpdatePasswordRequest, UserResponse},
        extractors::jwt_auth::AuthenticatedUser,
    },
};

#[get("/me")]
pub async fn get_me(
    auth: AuthenticatedUser,
    service: web::Data<UserService>,
) -> Result<HttpResponse, Error> {
    let user = service
        .find_by_id(auth.user_id)
        .await?
        .ok_or(Error::User(UserError::UserNotFound))?;

    Ok(HttpResponse::Ok().json(UserResponse::from(user)))
}

#[patch("/me/email")]
pub async fn update_email(
    auth: AuthenticatedUser,
    service: web::Data<UserService>,
    payload: web::Json<UpdateEmailRequest>,
) -> Result<HttpResponse, Error> {
    let user = service
        .update_email(auth.user_id, &payload.new_email)
        .await?;

    Ok(HttpResponse::Ok().json(UserResponse::from(user)))
}

#[patch("/me/password")]
pub async fn update_password(
    auth: AuthenticatedUser,
    service: web::Data<UserService>,
    payload: web::Json<UpdatePasswordRequest>,
) -> Result<HttpResponse, Error> {
    service
        .update_password(auth.user_id, &payload.new_password)
        .await?;

    Ok(HttpResponse::Ok().finish())
}

#[delete("/me")]
pub async fn delete_me(
    auth: AuthenticatedUser,
    service: web::Data<UserService>,
) -> Result<HttpResponse, Error> {
    service.delete_user(auth.user_id).await?;
    Ok(HttpResponse::NoContent().finish())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::entities::user::User;
    use crate::domain::repositories::user::UserRepository;
    use actix_web::dev::Service;
    use actix_web::{App, HttpMessage, test, web};
    use chrono::Utc;
    use mockall::mock;
    use mockall::predicate::*;
    use std::sync::Arc;
    use uuid::Uuid;

    mock! {
        pub UserRepo {}
        #[async_trait::async_trait]
        impl UserRepository for UserRepo {
            async fn create_user(&self, user: &User, password_hash: &str) -> Result<User, Error>;
            async fn find_by_email(&self, email: &str) -> Result<Option<User>, Error>;
            async fn find_credentials_by_email(&self, email: &str) -> Result<Option<(User, String)>, Error>;
            async fn find_by_id(&self, id: Uuid) -> Result<Option<User>, Error>;
            async fn update_email(&self, id: Uuid, new_email: &str) -> Result<User, Error>;
            async fn update_password(&self, id: Uuid, new_password: &str) -> Result<User, Error>;
            async fn delete_user(&self, id: Uuid) -> Result<(), Error>;
        }
    }

    fn mock_user() -> User {
        User {
            id: Uuid::new_v4(),
            email: "test@example.com".to_string(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }

    #[actix_web::test]
    async fn test_get_me_success() {
        let mut mock_repo = MockUserRepo::new();
        let user = mock_user();
        let user_id = user.id;
        let user_clone = user.clone();

        mock_repo
            .expect_find_by_id()
            .with(eq(user_id))
            .once()
            .returning(move |_| Ok(Some(user_clone.clone())));

        let service = UserService::new(Arc::new(mock_repo));

        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(service))
                .wrap_fn(move |req, srv| {
                    req.extensions_mut().insert(user_id);
                    let fut = srv.call(req);
                    Box::pin(async move { fut.await })
                })
                .service(get_me),
        )
        .await;

        let req = test::TestRequest::get().uri("/me").to_request();
        let resp = test::call_service(&app, req).await;

        assert!(resp.status().is_success());
        let body: UserResponse = test::read_body_json(resp).await;
        assert_eq!(body.id, user_id);
    }

    #[actix_web::test]
    async fn test_get_me_not_found() {
        let mut mock_repo = MockUserRepo::new();
        let user_id = Uuid::new_v4();

        mock_repo
            .expect_find_by_id()
            .with(eq(user_id))
            .once()
            .returning(|_| Ok(None));

        let service = UserService::new(Arc::new(mock_repo));

        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(service))
                .wrap_fn(move |req, srv| {
                    req.extensions_mut().insert(user_id);
                    let fut = srv.call(req);
                    Box::pin(async move { fut.await })
                })
                .service(get_me),
        )
        .await;

        let req = test::TestRequest::get().uri("/me").to_request();
        let resp = test::call_service(&app, req).await;

        assert!(resp.status().is_client_error() || resp.status().is_server_error());
    }

    #[actix_web::test]
    async fn test_update_email_success() {
        let mut mock_repo = MockUserRepo::new();
        let user = mock_user();
        let user_id = user.id;
        let user_clone = user.clone();
        let user_for_update = user.clone();

        mock_repo
            .expect_find_by_email()
            .with(eq("new@example.com"))
            .once()
            .returning(move |_| Ok(Some(user_clone.clone())));

        mock_repo
            .expect_update_email()
            .with(eq(user_id), eq("new@example.com"))
            .once()
            .returning(move |_, new_email: &str| {
                let mut updated = user_for_update.clone();
                updated.email = new_email.to_string();
                Ok(updated)
            });

        let service = UserService::new(Arc::new(mock_repo));

        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(service))
                .wrap_fn(move |req, srv| {
                    req.extensions_mut().insert(user_id);
                    let fut = srv.call(req);
                    Box::pin(async move { fut.await })
                })
                .service(update_email),
        )
        .await;

        let req = test::TestRequest::patch()
            .uri("/me/email")
            .set_json(UpdateEmailRequest {
                new_email: "new@example.com".to_string(),
            })
            .to_request();
        let resp = test::call_service(&app, req).await;

        assert!(resp.status().is_success());
        let body: UserResponse = test::read_body_json(resp).await;
        assert_eq!(body.email, "new@example.com");
    }

    #[actix_web::test]
    async fn test_update_password_success() {
        let mut mock_repo = MockUserRepo::new();
        let user_id = Uuid::new_v4();

        mock_repo
            .expect_update_password()
            .withf(move |id, hash| id == &user_id && !hash.is_empty())
            .once()
            .returning(|_, _| Ok(mock_user()));

        let service = UserService::new(Arc::new(mock_repo));

        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(service))
                .wrap_fn(move |req, srv| {
                    req.extensions_mut().insert(user_id);
                    let fut = srv.call(req);
                    Box::pin(async move { fut.await })
                })
                .service(update_password),
        )
        .await;

        let req = test::TestRequest::patch()
            .uri("/me/password")
            .set_json(UpdatePasswordRequest {
                new_password: "new_password123".to_string(),
            })
            .to_request();
        let resp = test::call_service(&app, req).await;

        assert!(resp.status().is_success());
        assert_eq!(resp.status(), 200);
    }

    #[actix_web::test]
    async fn test_delete_me_success() {
        let mut mock_repo = MockUserRepo::new();
        let user_id = Uuid::new_v4();

        mock_repo
            .expect_delete_user()
            .with(eq(user_id))
            .once()
            .returning(|_| Ok(()));

        let service = UserService::new(Arc::new(mock_repo));

        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(service))
                .wrap_fn(move |req, srv| {
                    req.extensions_mut().insert(user_id);
                    let fut = srv.call(req);
                    Box::pin(async move { fut.await })
                })
                .service(delete_me),
        )
        .await;

        let req = test::TestRequest::delete().uri("/me").to_request();
        let resp = test::call_service(&app, req).await;

        assert_eq!(resp.status(), 204);
    }
}
