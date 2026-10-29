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
