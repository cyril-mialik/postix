use gloo_net::http::{Request, Response};
use postix_core::post::{
    PostPublicationStatus, PostRatingRequest, PostRatingSummary, PostRequest, PostResponse,
    PostsResponse, UpdatePostRequest,
};
use postix_core::report::{CreateReportRequest, ReportReason};
use serde::Deserialize;
use serde_json::json;

use crate::shared::ApiError;

const API_URL: &str = "/api";
const PAGE_SIZE: usize = 20;

pub fn feed_endpoint(limit: usize, offset: usize) -> String {
    format!("{API_URL}/posts/feed?limit={limit}&offset={offset}")
}

pub fn my_posts_endpoint(offset: usize, status: PostPublicationStatus) -> String {
    format!(
        "{API_URL}/posts/mine?limit={PAGE_SIZE}&offset={offset}&status={}",
        status.as_str()
    )
}

pub fn post_endpoint(id: &str) -> String {
    format!("{API_URL}/posts/{id}")
}

pub fn post_rating_endpoint(id: &str) -> String {
    format!("{API_URL}/posts/{id}/rating")
}

pub fn bookmarks_endpoint() -> String {
    format!("{API_URL}/bookmarks")
}

pub fn post_bookmark_endpoint(id: &str) -> String {
    format!("{API_URL}/posts/{id}/bookmark")
}

pub fn post_report_endpoint(id: &str) -> String {
    format!("{API_URL}/posts/{id}/reports")
}

pub fn build_report_request(
    reason: ReportReason,
    message: &str,
) -> Result<CreateReportRequest, String> {
    let message = message.trim();
    if message.chars().count() > 3_000 {
        return Err("Report message must be 3000 characters or fewer.".to_owned());
    }
    Ok(CreateReportRequest {
        reason,
        message: (!message.is_empty()).then(|| message.to_owned()),
    })
}

pub fn build_post_request(
    title: &str,
    text: &str,
    tag_input: &str,
    status: PostPublicationStatus,
) -> Result<PostRequest, String> {
    let title = title.trim();
    let text = text.trim();
    let title_length = title.chars().count();

    if !(3..=256).contains(&title_length) {
        return Err("Title must be between 3 and 256 characters.".to_owned());
    }
    if text.is_empty() {
        return Err("Write something before saving.".to_owned());
    }
    if text.chars().count() > 30_000 {
        return Err("Post text must be 30000 characters or fewer.".to_owned());
    }

    let mut tag_names = Vec::new();
    for tag in tag_input
        .split(',')
        .map(str::trim)
        .filter(|tag| !tag.is_empty())
    {
        if tag.chars().count() > 50 {
            return Err("Tags must be 50 characters or fewer.".to_owned());
        }
        let tag = tag.to_lowercase();
        if !tag_names.contains(&tag) {
            tag_names.push(tag);
        }
    }

    Ok(PostRequest {
        title: title.to_owned(),
        content: json!({"text": text}),
        tag_names,
        status,
    })
}

pub fn build_update_post_request(
    title: &str,
    text: &str,
    tag_input: &str,
) -> Result<UpdatePostRequest, String> {
    let request = build_post_request(title, text, tag_input, PostPublicationStatus::Published)?;
    Ok(UpdatePostRequest {
        title: Some(request.title),
        content: Some(request.content),
        tag_names: Some(request.tag_names),
    })
}

pub async fn get_feed(token: &str, offset: usize) -> Result<Vec<PostResponse>, ApiError> {
    let response = Request::get(&feed_endpoint(PAGE_SIZE, offset))
        .header("Authorization", &format!("Bearer {token}"))
        .send()
        .await
        .map_err(|error| ApiError::Message(error.to_string()))?;
    let response = ensure_success(response, "Could not load posts.").await?;
    let body: PostsResponse = response
        .json()
        .await
        .map_err(|error| ApiError::Message(error.to_string()))?;
    Ok(body.posts)
}

pub async fn get_my_posts(
    token: &str,
    offset: usize,
    status: PostPublicationStatus,
) -> Result<Vec<PostResponse>, ApiError> {
    let response = Request::get(&my_posts_endpoint(offset, status))
        .header("Authorization", &format!("Bearer {token}"))
        .send()
        .await
        .map_err(|error| ApiError::Message(error.to_string()))?;
    let response = ensure_success(response, "Could not load your stories.").await?;
    let body: PostsResponse = response
        .json()
        .await
        .map_err(|error| ApiError::Message(error.to_string()))?;
    Ok(body.posts)
}

pub async fn get_post(token: &str, id: &str) -> Result<PostResponse, ApiError> {
    let response = Request::get(&post_endpoint(id))
        .header("Authorization", &format!("Bearer {token}"))
        .send()
        .await
        .map_err(|error| ApiError::Message(error.to_string()))?;
    ensure_success(response, "Could not load this post.")
        .await?
        .json()
        .await
        .map_err(|error| ApiError::Message(error.to_string()))
}

pub async fn get_post_rating(token: &str, id: &str) -> Result<PostRatingSummary, ApiError> {
    let response = Request::get(&post_rating_endpoint(id))
        .header("Authorization", &format!("Bearer {token}"))
        .send()
        .await
        .map_err(|error| ApiError::Message(error.to_string()))?;
    ensure_success(response, "Could not load this post's rating.")
        .await?
        .json()
        .await
        .map_err(|error| ApiError::Message(error.to_string()))
}

pub async fn rate_post(token: &str, id: &str, rating: i32) -> Result<PostRatingSummary, ApiError> {
    let body = build_post_rating_request(rating).map_err(ApiError::Message)?;
    let response = Request::post(&post_rating_endpoint(id))
        .header("Authorization", &format!("Bearer {token}"))
        .json(&body)
        .map_err(|error| ApiError::Message(error.to_string()))?
        .send()
        .await
        .map_err(|error| ApiError::Message(error.to_string()))?;
    ensure_success(response, "Could not rate this post.")
        .await?
        .json()
        .await
        .map_err(|error| ApiError::Message(error.to_string()))
}

pub async fn get_bookmarks(token: &str) -> Result<Vec<String>, ApiError> {
    let response = Request::get(&bookmarks_endpoint())
        .header("Authorization", &format!("Bearer {token}"))
        .send()
        .await
        .map_err(|error| ApiError::Message(error.to_string()))?;
    ensure_success(response, "Could not load your bookmarks.")
        .await?
        .json()
        .await
        .map_err(|error| ApiError::Message(error.to_string()))
}

pub async fn set_bookmark(token: &str, id: &str, bookmarked: bool) -> Result<(), ApiError> {
    let request = if bookmarked {
        Request::post(&post_bookmark_endpoint(id))
    } else {
        Request::delete(&post_bookmark_endpoint(id))
    };
    let response = request
        .header("Authorization", &format!("Bearer {token}"))
        .send()
        .await
        .map_err(|error| ApiError::Message(error.to_string()))?;
    ensure_success(response, "Could not update this bookmark.").await?;
    Ok(())
}

pub async fn create_report(
    token: &str,
    id: &str,
    reason: ReportReason,
    message: &str,
) -> Result<(), ApiError> {
    let body = build_report_request(reason, message).map_err(ApiError::Message)?;
    let response = Request::post(&post_report_endpoint(id))
        .header("Authorization", &format!("Bearer {token}"))
        .json(&body)
        .map_err(|error| ApiError::Message(error.to_string()))?
        .send()
        .await
        .map_err(|error| ApiError::Message(error.to_string()))?;
    ensure_success(response, "Could not report this post.").await?;
    Ok(())
}

fn build_post_rating_request(rating: i32) -> Result<PostRatingRequest, String> {
    if !(1..=5).contains(&rating) {
        return Err("Post rating must be between 1 and 5.".to_owned());
    }
    Ok(PostRatingRequest { rating })
}

pub async fn create_post(
    token: &str,
    title: &str,
    text: &str,
    tags: &str,
    status: PostPublicationStatus,
) -> Result<PostResponse, ApiError> {
    let body = build_post_request(title, text, tags, status).map_err(ApiError::Message)?;
    let response = Request::post(&format!("{API_URL}/posts"))
        .header("Authorization", &format!("Bearer {token}"))
        .json(&body)
        .map_err(|error| ApiError::Message(error.to_string()))?
        .send()
        .await
        .map_err(|error| ApiError::Message(error.to_string()))?;
    ensure_success(response, "Could not publish this post.")
        .await?
        .json()
        .await
        .map_err(|error| ApiError::Message(error.to_string()))
}

pub async fn update_post(
    token: &str,
    id: &str,
    title: &str,
    text: &str,
    tags: &str,
) -> Result<PostResponse, ApiError> {
    let body = build_update_post_request(title, text, tags).map_err(ApiError::Message)?;
    let response = Request::patch(&post_endpoint(id))
        .header("Authorization", &format!("Bearer {token}"))
        .json(&body)
        .map_err(|error| ApiError::Message(error.to_string()))?
        .send()
        .await
        .map_err(|error| ApiError::Message(error.to_string()))?;
    ensure_success(response, "Could not update this post.")
        .await?
        .json()
        .await
        .map_err(|error| ApiError::Message(error.to_string()))
}

pub async fn delete_post(token: &str, id: &str) -> Result<(), ApiError> {
    let response = Request::delete(&post_endpoint(id))
        .header("Authorization", &format!("Bearer {token}"))
        .send()
        .await
        .map_err(|error| ApiError::Message(error.to_string()))?;
    ensure_success(response, "Could not delete this post.").await?;
    Ok(())
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
    Err(map_api_error(status, server_error.as_deref(), fallback))
}

fn map_api_error(status: u16, server_error: Option<&str>, fallback: &str) -> ApiError {
    match status {
        401 => ApiError::Unauthorized,
        404 => ApiError::Message("This post could not be found.".to_owned()),
        _ => ApiError::Message(
            server_error
                .filter(|message| !message.trim().is_empty())
                .unwrap_or(fallback)
                .to_owned(),
        ),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn feed_and_post_endpoints_match_the_server_routes() {
        assert_eq!(
            feed_endpoint(PAGE_SIZE, 40),
            "/api/posts/feed?limit=20&offset=40"
        );
        assert_eq!(
            my_posts_endpoint(20, PostPublicationStatus::Published),
            "/api/posts/mine?limit=20&offset=20&status=published"
        );
        assert_eq!(
            post_endpoint("7b6ab915-1d04-44f2-b44d-1e84de73a310"),
            "/api/posts/7b6ab915-1d04-44f2-b44d-1e84de73a310"
        );
        assert_eq!(post_rating_endpoint("post-id"), "/api/posts/post-id/rating");
        assert_eq!(bookmarks_endpoint(), "/api/bookmarks");
        assert_eq!(
            post_bookmark_endpoint("post-id"),
            "/api/posts/post-id/bookmark"
        );
        assert_eq!(
            post_report_endpoint("post-id"),
            "/api/posts/post-id/reports"
        );
        assert_eq!(
            post_endpoint("7b6ab915-1d04-44f2-b44d-1e84de73a310"),
            "/api/posts/7b6ab915-1d04-44f2-b44d-1e84de73a310"
        );
    }

    #[test]
    fn post_rating_request_accepts_only_values_from_one_to_five() {
        assert_eq!(
            build_post_rating_request(1),
            Ok(PostRatingRequest { rating: 1 })
        );
        assert_eq!(
            build_post_rating_request(5),
            Ok(PostRatingRequest { rating: 5 })
        );
        assert!(build_post_rating_request(0).is_err());
        assert!(build_post_rating_request(6).is_err());
    }

    #[test]
    fn post_request_trims_fields_and_deduplicates_tags() {
        let request = build_post_request(
            "  My story  ",
            "  A first paragraph.  ",
            " Rust, rust, writing ",
            PostPublicationStatus::Published,
        )
        .unwrap();

        assert_eq!(
            request,
            PostRequest {
                title: "My story".to_owned(),
                content: json!({"text": "A first paragraph."}),
                tag_names: vec!["rust".to_owned(), "writing".to_owned()],
                status: PostPublicationStatus::Published,
            }
        );
    }

    #[test]
    fn post_request_rejects_missing_content_and_invalid_title_lengths() {
        assert!(build_post_request("ok", "text", "", PostPublicationStatus::Published).is_err());
        assert!(
            build_post_request("Valid title", "   ", "", PostPublicationStatus::Published).is_err()
        );
        assert!(
            build_post_request(
                &"x".repeat(257),
                "text",
                "",
                PostPublicationStatus::Published
            )
            .is_err()
        );
    }

    #[test]
    fn post_request_enforces_unicode_body_limit() {
        assert!(
            build_post_request(
                "A valid title",
                &"é".repeat(30_000),
                "",
                PostPublicationStatus::Published
            )
            .is_ok()
        );
        assert!(
            build_post_request(
                "A valid title",
                &"é".repeat(30_001),
                "",
                PostPublicationStatus::Published
            )
            .is_err()
        );
    }

    #[test]
    fn update_post_request_includes_validated_tags_and_content() {
        let request = build_update_post_request(" A title ", " body ", "Rust, rust").unwrap();

        assert_eq!(request.title.as_deref(), Some("A title"));
        assert_eq!(request.content, Some(json!({"text": "body"})));
        assert_eq!(request.tag_names, Some(vec!["rust".to_owned()]));
    }

    #[test]
    fn report_request_trims_optional_message_and_enforces_length() {
        assert_eq!(
            build_report_request(ReportReason::Spam, " reason "),
            Ok(CreateReportRequest {
                reason: ReportReason::Spam,
                message: Some("reason".to_owned()),
            })
        );
        assert_eq!(
            build_report_request(ReportReason::Other, "  ")
                .unwrap()
                .message,
            None
        );
        assert!(build_report_request(ReportReason::Other, &"é".repeat(3_001)).is_err());
    }

    #[test]
    fn api_errors_are_mapped_to_actionable_messages() {
        assert_eq!(map_api_error(401, None, "fallback"), ApiError::Unauthorized);
        assert_eq!(
            map_api_error(404, Some("post not found"), "fallback"),
            ApiError::Message("This post could not be found.".to_owned())
        );
        assert_eq!(
            map_api_error(500, Some("Database unavailable"), "fallback"),
            ApiError::Message("Database unavailable".to_owned())
        );
        assert_eq!(
            map_api_error(500, None, "fallback"),
            ApiError::Message("fallback".to_owned())
        );
    }
}
