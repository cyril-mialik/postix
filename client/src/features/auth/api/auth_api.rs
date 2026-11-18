use gloo_net::http::{Request, Response};
use postix_core::auth::{AccessTokenResponse, SignInRequest, SignUpRequest};
use serde::Deserialize;
use validator::Validate;

const API_URL: &str = "/api/auth";

fn auth_endpoint(action: &str) -> String {
    format!("{API_URL}/{action}")
}

#[derive(Clone, Debug, PartialEq)]
pub enum RefreshError {
    InvalidSession,
    Unavailable(String),
}

pub fn refresh_endpoint() -> String {
    auth_endpoint("refresh")
}

pub fn sign_out_endpoint() -> String {
    auth_endpoint("sign-out")
}

pub async fn refresh_access_token() -> Result<String, RefreshError> {
    let response = Request::post(&refresh_endpoint())
        .send()
        .await
        .map_err(|error| RefreshError::Unavailable(error.to_string()))?;

    if !response.ok() {
        let status = response.status();
        let server_error = response
            .json::<ErrorResponse>()
            .await
            .ok()
            .map(|body| body.error);
        return Err(map_refresh_error(status, server_error.as_deref()));
    }

    let auth: AccessTokenResponse = response
        .json()
        .await
        .map_err(|error| RefreshError::Unavailable(error.to_string()))?;
    Ok(auth.access_token)
}

pub async fn sign_out() -> Result<(), String> {
    let response = Request::post(&sign_out_endpoint())
        .send()
        .await
        .map_err(|error| error.to_string())?;

    if response.ok() {
        return Ok(());
    }

    let server_error = response
        .json::<ErrorResponse>()
        .await
        .ok()
        .map(|body| body.error);
    Err(server_error
        .filter(|message| !message.trim().is_empty())
        .unwrap_or_else(|| "Could not log out. Please try again.".to_owned()))
}

pub async fn sign_in(email: String, password: String) -> Result<String, String> {
    let body = SignInRequest {
        email: email.trim().to_owned(),
        password,
    };
    validate_sign_in(&body)?;

    let resp = Request::post(&auth_endpoint("sign-in"))
        .header("Content-Type", "application/json")
        .json(&body)
        .map_err(|e| e.to_string())?
        .send()
        .await
        .map_err(|e| e.to_string())?;

    if resp.ok() {
        let auth: AccessTokenResponse = resp.json().await.map_err(|e| e.to_string())?;
        Ok(auth.access_token)
    } else {
        Err(parse_auth_error(resp, false).await)
    }
}

pub async fn sign_up(email: String, password: String) -> Result<String, String> {
    let body = SignUpRequest {
        email: email.trim().to_owned(),
        password,
    };
    validate_sign_up(&body)?;

    let resp = Request::post(&auth_endpoint("sign-up"))
        .header("Content-Type", "application/json")
        .json(&body)
        .map_err(|e| e.to_string())?
        .send()
        .await
        .map_err(|e| e.to_string())?;

    if resp.ok() {
        let auth: AccessTokenResponse = resp.json().await.map_err(|e| e.to_string())?;
        Ok(auth.access_token)
    } else {
        Err(parse_auth_error(resp, true).await)
    }
}

#[derive(Deserialize)]
struct ErrorResponse {
    error: String,
}

async fn parse_auth_error(response: Response, is_sign_up: bool) -> String {
    let status = response.status();
    let server_error = response
        .json::<ErrorResponse>()
        .await
        .ok()
        .map(|body| body.error);

    map_auth_error(status, server_error.as_deref(), is_sign_up)
}

fn validate_sign_in(request: &SignInRequest) -> Result<(), String> {
    request.validate().map_err(|errors| {
        if errors.field_errors().contains_key("email") {
            "Enter a valid email address.".to_owned()
        } else {
            "Enter your password.".to_owned()
        }
    })
}

fn validate_sign_up(request: &SignUpRequest) -> Result<(), String> {
    request.validate().map_err(|errors| {
        if errors.field_errors().contains_key("email") {
            "Enter a valid email address.".to_owned()
        } else {
            "Password must be between 8 and 128 characters.".to_owned()
        }
    })
}

fn map_auth_error(status: u16, server_error: Option<&str>, is_sign_up: bool) -> String {
    match (is_sign_up, status) {
        (false, 401) => "Email or password is incorrect.".to_owned(),
        (true, 409) => "An account with this email already exists.".to_owned(),
        _ => server_error
            .filter(|message| !message.trim().is_empty())
            .map(str::to_owned)
            .unwrap_or_else(|| {
                if is_sign_up {
                    "Could not create your account. Please try again.".to_owned()
                } else {
                    "Could not sign in. Please try again.".to_owned()
                }
            }),
    }
}

fn map_refresh_error(status: u16, server_error: Option<&str>) -> RefreshError {
    if matches!(status, 401 | 403) {
        RefreshError::InvalidSession
    } else {
        RefreshError::Unavailable(
            server_error
                .filter(|message| !message.trim().is_empty())
                .unwrap_or("Could not verify your session. Please try again.")
                .to_owned(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn auth_requests_serialize_with_server_field_names() {
        let sign_in = SignInRequest {
            email: "reader@example.com".to_owned(),
            password: "secret".to_owned(),
        };
        let sign_up = SignUpRequest {
            email: "writer@example.com".to_owned(),
            password: "long-enough-password".to_owned(),
        };

        assert_eq!(
            serde_json::to_value(sign_in).unwrap(),
            json!({"email": "reader@example.com", "password": "secret"})
        );
        assert_eq!(
            serde_json::to_value(sign_up).unwrap(),
            json!({"email": "writer@example.com", "password": "long-enough-password"})
        );
    }

    #[test]
    fn auth_endpoints_use_the_api_prefix() {
        assert_eq!(auth_endpoint("sign-in"), "/api/auth/sign-in");
        assert_eq!(auth_endpoint("sign-up"), "/api/auth/sign-up");
        assert_eq!(refresh_endpoint(), "/api/auth/refresh");
        assert_eq!(sign_out_endpoint(), "/api/auth/sign-out");
    }

    #[test]
    fn auth_response_deserializes_server_fields() {
        let response: AccessTokenResponse = serde_json::from_value(json!({
            "access_token": "token",
            "token_type": "Bearer",
            "expires_in": 900
        }))
        .unwrap();

        assert_eq!(response.access_token, "token");
        assert_eq!(response.token_type, "Bearer");
        assert_eq!(response.expires_in, 900);
    }

    #[test]
    fn sign_in_validation_rejects_bad_email_and_empty_password() {
        let invalid_email = SignInRequest {
            email: "not-an-email".to_owned(),
            password: "secret".to_owned(),
        };
        assert_eq!(
            validate_sign_in(&invalid_email),
            Err("Enter a valid email address.".to_owned())
        );

        let missing_password = SignInRequest {
            email: "reader@example.com".to_owned(),
            password: String::new(),
        };
        assert_eq!(
            validate_sign_in(&missing_password),
            Err("Enter your password.".to_owned())
        );
    }

    #[test]
    fn sign_up_validation_enforces_server_password_limits() {
        let too_short = SignUpRequest {
            email: "writer@example.com".to_owned(),
            password: "short".to_owned(),
        };
        assert_eq!(
            validate_sign_up(&too_short),
            Err("Password must be between 8 and 128 characters.".to_owned())
        );

        let valid = SignUpRequest {
            email: "writer@example.com".to_owned(),
            password: "long-enough-password".to_owned(),
        };
        assert_eq!(validate_sign_up(&valid), Ok(()));
    }

    #[test]
    fn auth_errors_use_friendly_messages_and_server_details() {
        assert_eq!(
            map_auth_error(401, Some("invalid credentials"), false),
            "Email or password is incorrect."
        );
        assert_eq!(
            map_auth_error(409, Some("email already exists"), true),
            "An account with this email already exists."
        );
        assert_eq!(
            map_auth_error(400, Some("invalid email"), true),
            "invalid email"
        );
    }

    #[test]
    fn refresh_errors_distinguish_invalid_sessions_from_service_failures() {
        assert_eq!(map_refresh_error(401, None), RefreshError::InvalidSession);
        assert_eq!(
            map_refresh_error(503, Some("Service unavailable")),
            RefreshError::Unavailable("Service unavailable".to_owned())
        );
    }
}
