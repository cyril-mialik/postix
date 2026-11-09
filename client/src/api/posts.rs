use crate::types::post::{PostRequest, PostResponse};
use gloo_net::http::Request;

const API_URL: &str = "http://127.0.0.1:8080/api";

pub async fn fetch_feed() -> Result<Vec<PostResponse>, String> {
    let resp = Request::get(&format!("{}/posts/feed", API_URL))
        .send()
        .await
        .map_err(|e| e.to_string())?;

    if resp.ok() {
        resp.json::<Vec<PostResponse>>()
            .await
            .map_err(|e| e.to_string())
    } else {
        Err("Failed to fetch feed".to_string())
    }
}

pub async fn create_post(token: &str, req: PostRequest) -> Result<PostResponse, String> {
    let resp = Request::post(&format!("{}/posts", API_URL))
        .header("Authorization", &format!("Bearer {}", token))
        .json(&req)
        .map_err(|e| e.to_string())?
        .send()
        .await
        .map_err(|e| e.to_string())?;

    if resp.ok() {
        resp.json::<PostResponse>().await.map_err(|e| e.to_string())
    } else {
        Err("Failed to create post".to_string())
    }
}
