use gloo_net::http::{Request, Response};
use postix_core::review::{
    ReviewReaction, ReviewReactionRequest, ReviewReactionSummary, ReviewRequest, ReviewResponse,
    ReviewsResponse,
};
use serde::Deserialize;

use crate::shared::ApiError;

const API_URL: &str = "/api";

pub fn reviews_endpoint(post_id: &str) -> String {
    format!("{API_URL}/posts/{post_id}/reviews")
}

pub fn review_endpoint(post_id: &str, review_id: i32) -> String {
    format!("{}/posts/{post_id}/reviews/{review_id}", API_URL)
}

pub fn review_reaction_endpoint(post_id: &str, review_id: i32) -> String {
    format!("{}/posts/{post_id}/reviews/{review_id}/reaction", API_URL)
}

pub fn build_review_request(review: &str) -> Result<ReviewRequest, String> {
    let review = review.trim();
    let length = review.chars().count();
    if !(1..=10_000).contains(&length) {
        return Err("Comments must be between 1 and 10000 characters.".to_owned());
    }

    Ok(ReviewRequest {
        review: review.to_owned(),
    })
}

pub async fn get_reviews(token: &str, post_id: &str) -> Result<Vec<ReviewResponse>, ApiError> {
    let response = Request::get(&reviews_endpoint(post_id))
        .header("Authorization", &format!("Bearer {token}"))
        .send()
        .await
        .map_err(|error| ApiError::Message(error.to_string()))?;
    let response = ensure_success(response, "Could not load comments.").await?;
    let body: ReviewsResponse = response
        .json()
        .await
        .map_err(|error| ApiError::Message(error.to_string()))?;
    Ok(body.reviews)
}

pub async fn create_review(
    token: &str,
    post_id: &str,
    review: &str,
) -> Result<ReviewResponse, ApiError> {
    let body = build_review_request(review).map_err(ApiError::Message)?;
    let response = Request::post(&reviews_endpoint(post_id))
        .header("Authorization", &format!("Bearer {token}"))
        .json(&body)
        .map_err(|error| ApiError::Message(error.to_string()))?
        .send()
        .await
        .map_err(|error| ApiError::Message(error.to_string()))?;
    ensure_success(response, "Could not add this comment.")
        .await?
        .json()
        .await
        .map_err(|error| ApiError::Message(error.to_string()))
}

pub async fn update_review(
    token: &str,
    post_id: &str,
    review_id: i32,
    review: &str,
) -> Result<ReviewResponse, ApiError> {
    let body = build_review_request(review).map_err(ApiError::Message)?;
    let response = Request::patch(&review_endpoint(post_id, review_id))
        .header("Authorization", &format!("Bearer {token}"))
        .json(&body)
        .map_err(|error| ApiError::Message(error.to_string()))?
        .send()
        .await
        .map_err(|error| ApiError::Message(error.to_string()))?;
    ensure_success(response, "Could not update this comment.")
        .await?
        .json()
        .await
        .map_err(|error| ApiError::Message(error.to_string()))
}

pub async fn delete_review(token: &str, post_id: &str, review_id: i32) -> Result<(), ApiError> {
    let response = Request::delete(&review_endpoint(post_id, review_id))
        .header("Authorization", &format!("Bearer {token}"))
        .send()
        .await
        .map_err(|error| ApiError::Message(error.to_string()))?;
    ensure_success(response, "Could not delete this comment.").await?;
    Ok(())
}

pub async fn react_to_review(
    token: &str,
    post_id: &str,
    review_id: i32,
    reaction: ReviewReaction,
) -> Result<ReviewReactionSummary, ApiError> {
    let response = Request::post(&review_reaction_endpoint(post_id, review_id))
        .header("Authorization", &format!("Bearer {token}"))
        .json(&ReviewReactionRequest { reaction })
        .map_err(|error| ApiError::Message(error.to_string()))?
        .send()
        .await
        .map_err(|error| ApiError::Message(error.to_string()))?;
    ensure_success(response, "Could not react to this comment.")
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
        404 => Err(ApiError::Message(
            "This post could not be found.".to_owned(),
        )),
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
    fn comment_endpoints_match_the_server_routes() {
        assert_eq!(reviews_endpoint("post-id"), "/api/posts/post-id/reviews");
        assert_eq!(
            review_reaction_endpoint("post-id", 7),
            "/api/posts/post-id/reviews/7/reaction"
        );
        assert_eq!(
            review_endpoint("post-id", 7),
            "/api/posts/post-id/reviews/7"
        );
    }

    #[test]
    fn reactions_use_the_shared_snake_case_contract() {
        assert_eq!(
            serde_json::to_string(&ReviewReactionRequest {
                reaction: ReviewReaction::Agree
            })
            .unwrap(),
            r#"{"reaction":"agree"}"#
        );
    }

    #[test]
    fn review_request_trims_and_validates_comment_text() {
        assert_eq!(
            build_review_request("  Helpful note  ").unwrap(),
            ReviewRequest {
                review: "Helpful note".to_owned()
            }
        );
        assert!(build_review_request(" \n ").is_err());
        assert!(build_review_request(&"x".repeat(10_001)).is_err());
    }
}
