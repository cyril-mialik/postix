use gloo_net::http::Request;
use serde::{Deserialize, Serialize};

const API_URL: &str = "/auth";

#[derive(Serialize)]
struct AuthRequest {
    email: String,
    password: String,
}

#[derive(Deserialize)]
struct AuthResponse {
    access_token: String,
}

pub async fn sign_in(email: String, password: String) -> Result<String, String> {
    let body = AuthRequest { email, password };

    let resp = Request::post(&format!("{}/sign-in", API_URL))
        .header("Content-Type", "application/json")
        .json(&body)
        .map_err(|e| e.to_string())?
        .send()
        .await
        .map_err(|e| e.to_string())?;

    if resp.ok() {
        let auth: AuthResponse = resp.json().await.map_err(|e| e.to_string())?;
        Ok(auth.access_token)
    } else {
        Err("Invalid email or password".to_string())
    }
}

pub async fn sign_up(email: String, password: String) -> Result<String, String> {
    let body = AuthRequest { email, password };

    let resp = Request::post(&format!("{}/sign-up", API_URL))
        .header("Content-Type", "application/json")
        .json(&body)
        .map_err(|e| e.to_string())?
        .send()
        .await
        .map_err(|e| e.to_string())?;

    if resp.ok() {
        let auth: AuthResponse = resp.json().await.map_err(|e| e.to_string())?;
        Ok(auth.access_token)
    } else {
        Err("Registration failed. Email might be taken.".to_string())
    }
}
