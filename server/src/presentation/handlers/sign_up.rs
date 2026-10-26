use actix_web::{HttpResponse, post, web};

use crate::{
    application::auth::AuthService,
    application::user::UserService,
    domain::entities::errors::Error,
    infrastructure::security::jwt::build_refresh_cookie,
    presentation::dto::{auth::AccessTokenResponse, user::SignUpRequest},
};

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
