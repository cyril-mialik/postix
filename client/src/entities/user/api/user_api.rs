use gloo_net::http::{Request, Response};
use postix_core::user::{UpdateProfileRequest, UserResponse};
use serde::Deserialize;

use crate::shared::ApiError;

const PROFILE_URL: &str = "/api/me";

pub fn build_update_profile_request(
    username: &str,
    first_name: &str,
    last_name: &str,
    avatar: &str,
) -> Result<UpdateProfileRequest, String> {
    let username = username.trim();
    if !(3..=128).contains(&username.chars().count()) {
        return Err("Username must be between 3 and 128 characters.".to_owned());
    }

    Ok(UpdateProfileRequest {
        username: Some(username.to_lowercase()),
        first_name: Some(first_name.trim().to_owned()),
        last_name: Some(last_name.trim().to_owned()),
        avatar: Some(avatar.trim().to_owned()),
    })
}

pub async fn get_me(token: &str) -> Result<UserResponse, ApiError> {
    let response = Request::get(PROFILE_URL)
        .header("Authorization", &format!("Bearer {token}"))
        .send()
        .await
        .map_err(|error| ApiError::Message(error.to_string()))?;
    ensure_success(response, "Could not load your profile.")
        .await?
        .json()
        .await
        .map_err(|error| ApiError::Message(error.to_string()))
}

pub async fn update_profile(
    token: &str,
    request: &UpdateProfileRequest,
) -> Result<UserResponse, ApiError> {
    let response = Request::patch(PROFILE_URL)
        .header("Authorization", &format!("Bearer {token}"))
        .json(request)
        .map_err(|error| ApiError::Message(error.to_string()))?
        .send()
        .await
        .map_err(|error| ApiError::Message(error.to_string()))?;
    ensure_success(response, "Could not update your profile.")
        .await?
        .json()
        .await
        .map_err(|error| ApiError::Message(error.to_string()))
}

#[derive(Deserialize)]
struct ErrorResponse {
    error: String,
}

async fn ensure_success(response: Response, fallback: &str) -> Result<Response, ApiError> {
    if response.ok() {
        return Ok(response);
    }

    let status = response.status();
    let server_error = response
        .json::<ErrorResponse>()
        .await
        .ok()
        .map(|body| body.error);
    match status {
        401 => Err(ApiError::Unauthorized),
        _ => Err(ApiError::Message(
            server_error
                .filter(|message| !message.trim().is_empty())
                .unwrap_or(fallback.to_owned()),
        )),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn profile_endpoint_uses_the_protected_me_route() {
        assert_eq!(PROFILE_URL, "/api/me");
    }

    #[test]
    fn profile_updates_trim_fields_and_validate_username() {
        assert_eq!(
            build_update_profile_request(
                "  Ada  ",
                " Ada ",
                " Lovelace ",
                " https://a.test/p.png "
            )
            .unwrap(),
            UpdateProfileRequest {
                username: Some("ada".to_owned()),
                first_name: Some("Ada".to_owned()),
                last_name: Some("Lovelace".to_owned()),
                avatar: Some("https://a.test/p.png".to_owned()),
            }
        );
        assert!(build_update_profile_request("ab", "", "", "").is_err());
    }
}
