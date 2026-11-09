use gloo_net::http::Request;
use serde::{Deserialize, Serialize};

const API_URL: &str = "http://127.0.0.1:8080/api";

#[derive(Serialize)]
pub struct SignInRequest {
    pub email: String,
    pub password: String,
}

#[derive(Deserialize)]
pub struct TokenResponse {
    pub access_token: String,
    pub token_type: String,
    pub expires_in: u64,
}

pub async fn sign_in(email: String, password: String) -> Result<String, String> {
    let req = SignInRequest { email, password };

    let resp = Request::post(&format!("{}/auth/sign-in", API_URL))
        .header("Content-Type", "application/json")
        .json(&req)
        .map_err(|e| e.to_string())?
        .send()
        .await
        .map_err(|e| e.to_string())?;

    if resp.ok() {
        let token: TokenResponse = resp.json().await.map_err(|e| e.to_string())?;
        Ok(token.access_token)
    } else {
        Err("invalid credentials".to_string())
    }
}
