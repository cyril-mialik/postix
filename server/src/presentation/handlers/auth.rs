use actix_web::{HttpRequest, HttpResponse, post, web};

use crate::{
    application::{auth::AuthService, user::UserService},
    domain::entities::{auth::AuthError, errors::Error},
    infrastructure::security::jwt::{
        REFRESH_TOKEN_NAME, build_clear_refresh_cookie, build_refresh_cookie,
    },
    presentation::dto::{
        auth::{AccessTokenResponse, SignInRequest},
        user::SignUpRequest,
    },
};

#[post("/sign-in")]
pub async fn sign_in(
    service: web::Data<AuthService>,
    payload: web::Json<SignInRequest>,
) -> Result<HttpResponse, Error> {
    let tokens = service
        .sign_in(payload.email.clone(), &payload.password)
        .await?;

    let cookie = build_refresh_cookie(&tokens.refresh_token);

    Ok(HttpResponse::Ok().cookie(cookie).json(AccessTokenResponse {
        access_token: tokens.access_token,
    }))
}

#[post("/sign-up")]
pub async fn sign_up(
    user_service: web::Data<UserService>,
    auth_service: web::Data<AuthService>,
    payload: web::Json<SignUpRequest>,
) -> Result<HttpResponse, Error> {
    let _user = user_service
        .create_user(payload.email.clone(), &payload.password)
        .await?;

    let tokens = auth_service
        .sign_in(payload.email.clone(), &payload.password)
        .await?;

    let cookie = build_refresh_cookie(&tokens.refresh_token);

    Ok(HttpResponse::Created()
        .cookie(cookie)
        .json(AccessTokenResponse {
            access_token: tokens.access_token,
        }))
}

#[post("/sign-out")]
pub async fn sign_out(
    service: web::Data<AuthService>,
    req: HttpRequest,
) -> Result<HttpResponse, Error> {
    if let Some(cookie) = req.cookie(REFRESH_TOKEN_NAME) {
        service.sign_out(cookie.value()).await?;
    }

    Ok(HttpResponse::NoContent()
        .cookie(build_clear_refresh_cookie())
        .finish())
}

#[post("/refresh")]
pub async fn refresh(
    service: web::Data<AuthService>,
    req: HttpRequest,
) -> Result<HttpResponse, Error> {
    let token = req
        .cookie(REFRESH_TOKEN_NAME)
        .map(|c| c.value().to_string())
        .ok_or(Error::Auth(AuthError::InvalidToken))?;

    let tokens = service.refresh(&token).await?;
    let cookie = build_refresh_cookie(&tokens.refresh_token);

    Ok(HttpResponse::Ok().cookie(cookie).json(AccessTokenResponse {
        access_token: tokens.access_token,
    }))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::entities::user::User;
    use crate::domain::repositories::{token::TokenRepository, user::UserRepository};
    use actix_web::{App, test, web};
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

    mock! {
        pub TokenRepo {}
        #[async_trait::async_trait]
        impl TokenRepository for TokenRepo {
            async fn save_refresh_token(&self, user_id: Uuid, token: &str, ttl_seconds: u64) -> Result<(), Error>;
            async fn is_refresh_token_valid(&self, user_id: Uuid, token: &str) -> Result<bool, Error>;
            async fn revoke_refresh_token(&self, user_id: Uuid, token: &str) -> Result<(), Error>;
            async fn revoke_all_refresh_tokens(&self, user_id: Uuid) -> Result<(), Error>;
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
    async fn test_sign_in_success() {
        let mut mock_user_repo = MockUserRepo::new();
        let mut mock_token_repo = MockTokenRepo::new();
        let user = mock_user();
        let email = user.email.clone();
        let password = "password123";
        let password_hash =
            crate::infrastructure::security::password::hash_password(password).unwrap();

        mock_user_repo
            .expect_find_credentials_by_email()
            .with(eq(email.clone()))
            .once()
            .returning(move |_| Ok(Some((user.clone(), password_hash.clone()))));

        mock_token_repo
            .expect_save_refresh_token()
            .once()
            .returning(|_, _, _| Ok(()));

        let auth_service = AuthService::new(
            Arc::new(mock_user_repo),
            Arc::new(mock_token_repo),
            "test_secret".to_string(),
        );

        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(auth_service))
                .service(sign_in),
        )
        .await;

        let req = test::TestRequest::post()
            .uri("/sign-in")
            .set_json(SignInRequest {
                email,
                password: password.to_string(),
            })
            .to_request();

        let resp = test::call_service(&app, req).await;

        assert_eq!(resp.status(), 200);
        assert!(
            resp.headers()
                .get(actix_web::http::header::SET_COOKIE)
                .is_some()
        );

        let body: AccessTokenResponse = test::read_body_json(resp).await;
        assert!(!body.access_token.is_empty());
    }

    #[actix_web::test]
    async fn test_sign_up_success() {
        let mut mock_user_repo = MockUserRepo::new();
        let mut mock_token_repo = MockTokenRepo::new();
        let user = mock_user();
        let email = user.email.clone();
        let password = "password123";
        let password_hash =
            crate::infrastructure::security::password::hash_password(password).unwrap();

        let user_for_create = user.clone();
        let user_for_sign_in = user.clone();

        mock_user_repo
            .expect_find_by_email()
            .with(eq(email.clone()))
            .once()
            .returning(|_| Ok(None));

        mock_user_repo
            .expect_create_user()
            .once()
            .returning(move |_, _| Ok(user_for_create.clone()));

        mock_user_repo
            .expect_find_credentials_by_email()
            .once()
            .returning(move |_| Ok(Some((user_for_sign_in.clone(), password_hash.clone()))));

        mock_token_repo
            .expect_save_refresh_token()
            .once()
            .returning(|_, _, _| Ok(()));

        let repo: Arc<dyn UserRepository> = Arc::new(mock_user_repo);
        let user_service = UserService::new(repo.clone());
        let auth_service =
            AuthService::new(repo, Arc::new(mock_token_repo), "test_secret".to_string());

        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(user_service))
                .app_data(web::Data::new(auth_service))
                .service(sign_up),
        )
        .await;

        let req = test::TestRequest::post()
            .uri("/sign-up")
            .set_json(SignUpRequest {
                email,
                password: password.to_string(),
            })
            .to_request();

        let resp = test::call_service(&app, req).await;

        assert_eq!(resp.status(), 201);
        assert!(
            resp.headers()
                .get(actix_web::http::header::SET_COOKIE)
                .is_some()
        );

        let body: AccessTokenResponse = test::read_body_json(resp).await;
        assert!(!body.access_token.is_empty());
    }

    #[actix_web::test]
    async fn test_sign_out_success() {
        let mock_user_repo = MockUserRepo::new();
        let mut mock_token_repo = MockTokenRepo::new();
        let user_id = Uuid::new_v4();
        let secret = "test_secret".to_string();
        let refresh_token =
            crate::infrastructure::security::jwt::generate_refresh_token(&secret, user_id).unwrap();

        mock_token_repo
            .expect_revoke_refresh_token()
            .with(eq(user_id), eq(refresh_token.clone()))
            .once()
            .returning(|_, _| Ok(()));

        let auth_service =
            AuthService::new(Arc::new(mock_user_repo), Arc::new(mock_token_repo), secret);

        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(auth_service))
                .service(sign_out),
        )
        .await;

        let req = test::TestRequest::post()
            .uri("/sign-out")
            .cookie(actix_web::cookie::Cookie::new(
                REFRESH_TOKEN_NAME,
                refresh_token,
            ))
            .to_request();

        let resp = test::call_service(&app, req).await;

        assert_eq!(resp.status(), 204);
        assert!(
            resp.headers()
                .get(actix_web::http::header::SET_COOKIE)
                .is_some()
        );
    }

    #[actix_web::test]
    async fn test_refresh_success() {
        let mut mock_user_repo = MockUserRepo::new();
        let mut mock_token_repo = MockTokenRepo::new();
        let user = mock_user();
        let user_id = user.id;
        let user_clone = user.clone();
        let secret = "test_secret".to_string();
        let refresh_token =
            crate::infrastructure::security::jwt::generate_refresh_token(&secret, user_id).unwrap();

        mock_token_repo
            .expect_is_refresh_token_valid()
            .with(eq(user_id), eq(refresh_token.clone()))
            .once()
            .returning(|_, _| Ok(true));

        mock_user_repo
            .expect_find_by_id()
            .with(eq(user_id))
            .once()
            .returning(move |_| Ok(Some(user_clone.clone())));

        mock_token_repo
            .expect_revoke_refresh_token()
            .with(eq(user_id), eq(refresh_token.clone()))
            .once()
            .returning(|_, _| Ok(()));

        mock_token_repo
            .expect_save_refresh_token()
            .once()
            .returning(|_, _, _| Ok(()));

        let auth_service =
            AuthService::new(Arc::new(mock_user_repo), Arc::new(mock_token_repo), secret);

        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(auth_service))
                .service(refresh),
        )
        .await;

        let req = test::TestRequest::post()
            .uri("/refresh")
            .cookie(actix_web::cookie::Cookie::new(
                REFRESH_TOKEN_NAME,
                refresh_token,
            ))
            .to_request();

        let resp = test::call_service(&app, req).await;

        assert_eq!(resp.status(), 200);
        assert!(
            resp.headers()
                .get(actix_web::http::header::SET_COOKIE)
                .is_some()
        );

        let body: AccessTokenResponse = test::read_body_json(resp).await;
        assert!(!body.access_token.is_empty());
    }

    #[actix_web::test]
    async fn test_refresh_missing_cookie() {
        let mock_user_repo = MockUserRepo::new();
        let mock_token_repo = MockTokenRepo::new();
        let auth_service = AuthService::new(
            Arc::new(mock_user_repo),
            Arc::new(mock_token_repo),
            "test_secret".to_string(),
        );

        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(auth_service))
                .service(refresh),
        )
        .await;

        let req = test::TestRequest::post().uri("/refresh").to_request();

        let resp = test::call_service(&app, req).await;

        assert!(resp.status().is_client_error() || resp.status().is_server_error());
    }
}
