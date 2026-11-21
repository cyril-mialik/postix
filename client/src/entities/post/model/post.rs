use postix_core::post::PostResponse;

#[derive(Clone, Copy, Default, PartialEq, Eq)]
pub enum SearchField {
    #[default]
    All,
    Title,
    Content,
}

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
    let preview = chars.by_ref().take(200).collect::<String>();
    if chars.next().is_some() {
        format!("{preview}…")
    } else {
        preview
    }
}

pub fn matches_query_in_field(post: &PostResponse, query: &str, field: SearchField) -> bool {
    let query = query.trim().to_lowercase();
    if query.is_empty() {
        return true;
    }

    if let Some(username) = query.strip_prefix('@') {
        return username.is_empty() || post.author_username.to_lowercase().contains(username);
    }
    if let Some(tag) = query.strip_prefix('#') {
        return tag.is_empty()
            || post
                .tags
                .iter()
                .any(|item| item.name.to_lowercase().contains(tag));
    }

    match field {
        SearchField::Title => post.title.to_lowercase().contains(&query),
        SearchField::Content => content_text(post).to_lowercase().contains(&query),
        SearchField::All => {
            post.title.to_lowercase().contains(&query)
                || content_text(post).to_lowercase().contains(&query)
                || post
                    .tags
                    .iter()
                    .any(|tag| tag.name.to_lowercase().contains(&query))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    fn post(content: serde_json::Value) -> PostResponse {
        serde_json::from_value(json!({
            "id": "00000000-0000-0000-0000-000000000000",
            "author_id": "00000000-0000-0000-0000-000000000000",
            "author_username": "rust_writer",
            "author_avatar": null,
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
    fn preview_truncates_after_200_unicode_characters() {
        let exactly_limit = post(json!({"text": "é".repeat(200)}));
        let over_limit = post(json!({"text": "é".repeat(201)}));

        assert_eq!(content_preview(&exactly_limit), "é".repeat(200));
        assert_eq!(
            content_preview(&over_limit),
            format!("{}…", "é".repeat(200))
        );
    }

    #[test]
    fn feed_search_matches_title_content_and_tags_case_insensitively() {
        let post = post(json!({"text": "Building apps"}));
        assert!(matches_query_in_field(&post, "RUST", SearchField::All));
        assert!(matches_query_in_field(&post, "BUILDING", SearchField::All));
        assert!(matches_query_in_field(&post, "program", SearchField::All));
        assert!(!matches_query_in_field(
            &post,
            "unrelated",
            SearchField::All
        ));
    }

    #[test]
    fn feed_search_supports_author_and_tag_prefixes_and_field_filters() {
        let post = post(json!({"text": "Building apps"}));

        assert!(matches_query_in_field(&post, "@rust", SearchField::All));
        assert!(matches_query_in_field(&post, "#program", SearchField::All));
        assert!(!matches_query_in_field(&post, "@another", SearchField::All));
        assert!(matches_query_in_field(&post, "Rust", SearchField::Title));
        assert!(!matches_query_in_field(
            &post,
            "Building",
            SearchField::Title
        ));
        assert!(matches_query_in_field(
            &post,
            "Building",
            SearchField::Content
        ));
    }
}
