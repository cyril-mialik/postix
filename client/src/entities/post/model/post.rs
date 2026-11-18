use postix_core::post::PostResponse;

pub fn content_text(post: &PostResponse) -> String {
    match &post.content {
        serde_json::Value::String(text) => text.clone(),
        serde_json::Value::Object(content) => content
            .get("text")
            .or_else(|| content.get("body"))
            .and_then(serde_json::Value::as_str)
            .map(str::to_owned)
            .unwrap_or_else(|| post.content.to_string()),
        _ => post.content.to_string(),
    }
}

pub fn content_preview(post: &PostResponse) -> String {
    let text = content_text(post);
    let mut chars = text.chars();
    let preview = chars.by_ref().take(150).collect::<String>();
    if chars.next().is_some() {
        format!("{preview}…")
    } else {
        preview
    }
}

pub fn matches_query(post: &PostResponse, query: &str) -> bool {
    let query = query.trim().to_lowercase();
    if query.is_empty() {
        return true;
    }

    post.title.to_lowercase().contains(&query)
        || content_text(post).to_lowercase().contains(&query)
        || post
            .tags
            .iter()
            .any(|tag| tag.name.to_lowercase().contains(&query))
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    fn post(content: serde_json::Value) -> PostResponse {
        serde_json::from_value(json!({
            "id": "00000000-0000-0000-0000-000000000000",
            "author_id": "00000000-0000-0000-0000-000000000000",
            "title": "Rust notes",
            "content": content,
            "status": "published",
            "rating_sum": 0,
            "rating_count": 0,
            "reviews_count": 0,
            "views_count": 0,
            "media": [],
            "tags": [{"id": 1, "name": "programming"}],
            "created_at": "2026-01-01T00:00:00Z",
            "updated_at": "2026-01-01T00:00:00Z"
        }))
        .unwrap()
    }

    #[test]
    fn content_supports_the_mvp_text_shape_and_legacy_values() {
        assert_eq!(content_text(&post(json!({"text": "A story"}))), "A story");
        assert_eq!(content_text(&post(json!("Plain text"))), "Plain text");
        assert_eq!(
            content_text(&post(json!({"body": "Older shape"}))),
            "Older shape"
        );
    }

    #[test]
    fn preview_truncates_after_150_unicode_characters() {
        let exactly_limit = post(json!({"text": "é".repeat(150)}));
        let over_limit = post(json!({"text": "é".repeat(151)}));

        assert_eq!(content_preview(&exactly_limit), "é".repeat(150));
        assert_eq!(
            content_preview(&over_limit),
            format!("{}…", "é".repeat(150))
        );
    }

    #[test]
    fn feed_search_matches_title_content_and_tags_case_insensitively() {
        let post = post(json!({"text": "Building apps"}));
        assert!(matches_query(&post, "RUST"));
        assert!(matches_query(&post, "BUILDING"));
        assert!(matches_query(&post, "program"));
        assert!(!matches_query(&post, "unrelated"));
    }
}
