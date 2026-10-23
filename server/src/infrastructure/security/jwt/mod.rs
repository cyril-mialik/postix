use actix_web::cookie::{Cookie, SameSite};
use anyhow::{Result, anyhow};
use chrono::{Duration, Utc};
use cookie::time::Duration as CookieDuration;
use jsonwebtoken::{Algorithm, DecodingKey, EncodingKey, Header, Validation, decode, encode};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub const REFRESH_TOKEN_NAME: &str = "refresh_token";
pub const ACCESS_TOKEN: &str = "access";
pub const REFRESH_TOKEN: &str = "refresh";
pub const ACCESS_TOKEN_TTL: u64 = 15 * 60;
pub const REFRESH_TOKEN_TTL: u64 = 30 * 24 * 60 * 60;

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
    iat: i64,
    exp: i64,
    typ: String,
}

pub fn generate_access_token(secret: &str, user_id: Uuid) -> Result<String> {
    generate_token(
        secret,
        user_id,
        Duration::seconds(ACCESS_TOKEN_TTL as i64),
        ACCESS_TOKEN,
    )
}

pub fn generate_refresh_token(secret: &str, user_id: Uuid) -> Result<String> {
    generate_token(
        secret,
        user_id,
        Duration::seconds(REFRESH_TOKEN_TTL as i64),
        REFRESH_TOKEN,
    )
}

pub fn verify_access_token(secret: &str, token: &str) -> Result<Uuid> {
    verify_token(secret, token, ACCESS_TOKEN, 60)
}

pub fn verify_refresh_token(secret: &str, token: &str) -> Result<Uuid> {
    verify_token(secret, token, REFRESH_TOKEN, 0)
}

pub fn build_refresh_cookie(token: &str) -> Cookie<'static> {
    Cookie::build(REFRESH_TOKEN_NAME, token.to_string())
        .http_only(true)
        .secure(true)
        .same_site(SameSite::Strict)
        .path("/api")
        .max_age(CookieDuration::seconds(REFRESH_TOKEN_TTL as i64))
        .finish()
}

pub fn build_clear_refresh_cookie() -> Cookie<'static> {
    Cookie::build(REFRESH_TOKEN_NAME, "")
        .http_only(true)
        .secure(true)
        .same_site(SameSite::Strict)
        .path("/api")
        .max_age(CookieDuration::ZERO)
        .finish()
}

fn verify_token(secret: &str, token: &str, expected_typ: &str, leeway: u64) -> Result<Uuid> {
    let mut validation = Validation::new(Algorithm::HS256);
    validation.leeway = leeway;
    validation.validate_exp = true;
    validation.set_required_spec_claims(&["exp", "sub", "iat", "typ"]);

    let data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &validation,
    )?;
    if data.claims.typ != expected_typ {
        return Err(anyhow!("wrong token type"));
    }
    Ok(Uuid::parse_str(&data.claims.sub)?)
}

fn generate_token(secret: &str, user_id: Uuid, duration: Duration, typ: &str) -> Result<String> {
    let now = Utc::now();
    let claims = Claims {
        sub: user_id.to_string(),
        iat: now.timestamp(),
        exp: (now + duration).timestamp(),
        typ: typ.to_string(),
    };
    Ok(encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )?)
}
