pub use postix_core::post::{
    FeedQuery, PostMediaResponse, PostRequest, PostResponse, PostsResponse, TagResponse,
    UpdatePostRequest,
};

use crate::domain::entities::post::Post;

impl From<Post> for PostResponse {
    fn from(post: Post) -> Self {
        Self {
            id: post.id,
            author_id: post.author_id,
            author_username: post.author_username,
            author_avatar: post.author_avatar,
            title: post.title,
            content: post.content,
            status: post.status.as_str().to_owned(),
            rating_sum: post.rating_sum,
            rating_count: post.rating_count,
            reviews_count: post.reviews_count,
            views_count: post.views_count,
            media: post
                .media
                .into_iter()
                .map(|m| PostMediaResponse {
                    id: m.id,
                    url: m.url,
                    media_type: m.media_type.as_str().to_string(),
                    position: m.position,
                })
                .collect(),
            tags: post
                .tags
                .into_iter()
                .map(|t| TagResponse {
                    id: t.id,
                    name: t.name,
                })
                .collect(),
            created_at: post.created_at,
            updated_at: post.updated_at,
        }
    }
}

pub fn posts_response_from(posts: Vec<Post>) -> PostsResponse {
    PostsResponse {
        posts: posts.into_iter().map(PostResponse::from).collect(),
    }
}

#[cfg(test)]
mod tests {
    use chrono::Utc;
    use serde_json::json;
    use uuid::Uuid;

    use super::*;
    use crate::domain::entities::{
        media_type::MediaType, post::PostMedia, post_status::PostStatus, tag::Tag,
    };

    #[test]
    fn post_response_maps_nested_media_and_tags() {
        let post = Post {
            id: Uuid::nil(),
            author_id: Uuid::from_u128(1),
            author_username: "writer".to_owned(),
            author_avatar: Some("https://example.com/avatar.png".to_owned()),
            title: "A post".to_owned(),
            content: json!({"body": "Hello"}),
            status: PostStatus::Published,
            rating_sum: 12,
            rating_count: 4,
            reviews_count: 2,
            views_count: 30,
            media: vec![PostMedia {
                id: Uuid::from_u128(2),
                url: "https://example.com/image.png".to_owned(),
                media_type: MediaType::Image,
                position: 0,
            }],
            tags: vec![Tag {
                id: 8,
                name: "rust".to_owned(),
            }],
            created_at: Utc::now(),
            updated_at: Utc::now(),
            deleted_at: None,
            deleted_by: None,
        };

        let response = posts_response_from(vec![post]);

        assert_eq!(response.posts.len(), 1);
        assert_eq!(response.posts[0].status, "published");
        assert_eq!(response.posts[0].author_username, "writer");
        assert_eq!(
            response.posts[0].author_avatar.as_deref(),
            Some("https://example.com/avatar.png")
        );
        assert_eq!(response.posts[0].content, json!({"body": "Hello"}));
        assert_eq!(response.posts[0].media.len(), 1);
        assert_eq!(response.posts[0].media[0].media_type, "image");
        assert_eq!(response.posts[0].tags.len(), 1);
        assert_eq!(response.posts[0].tags[0].name, "rust");
        assert_eq!(response.posts[0].rating_sum, 12);
        assert_eq!(response.posts[0].views_count, 30);
    }
}
