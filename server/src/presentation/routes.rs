use actix_web::web;

use super::handlers::{auth, health, post, profile, rating, review};

pub fn api_configure(cfg: &mut web::ServiceConfig) {
    cfg.service(auth::sign_up)
        .service(auth::sign_in)
        .service(auth::refresh)
        .service(auth::sign_out)
        .service(health::health);
}

pub fn protected_api_configure(cfg: &mut web::ServiceConfig) {
    cfg.service(profile::get_me)
        .service(profile::update_profile)
        .service(review::get_reviews)
        .service(review::create_review)
        .service(review::react_to_review)
        .service(rating::get_post_rating)
        .service(rating::rate_post)
        .service(post::create_post)
        .service(post::get_feed)
        .service(post::get_my_posts)
        .service(post::get_post)
        .service(post::update_post)
        .service(post::delete_post);
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use actix_web::{App, http::StatusCode, test, web};
    use postix_core::{
        post::{PostRatingRequest, PostRatingSummary, PostRequest, PostResponse},
        review::{
            ReviewReaction, ReviewReactionRequest, ReviewReactionSummary, ReviewRequest,
            ReviewResponse, ReviewsResponse,
        },
    };
    use serde_json::json;
    use uuid::Uuid;

    use super::*;
    use crate::{
        application::{
            post::PostService, rating::RatingService, review::ReviewService,
            review_reaction::ReviewReactionService,
        },
        domain::{
            entities::{
                post::{CreatePostDtoBuilder, Post},
                rating::Rating,
                review::Review,
                review_reaction::{
                    ReviewReaction as DomainReviewReaction,
                    ReviewReactionSummary as DomainReviewReactionSummary,
                },
                tag::Tag,
            },
            repositories::{
                post::MockPostRepository, rating::MockRatingRepository,
                review::MockReviewRepository, review_reaction::MockReviewReactionRepository,
                tag::MockTagRepository, user::MockUserRepository,
            },
        },
        infrastructure::{middleware::auth::JwtAuth, security::jwt::generate_access_token},
    };
    #[actix_web::test]
    async fn health_and_auth_routes_live_under_api_and_protected_routes_stay_protected() {
        let app = test::init_service(
            App::new().service(
                web::scope("/api").configure(api_configure).service(
                    web::scope("")
                        .wrap(JwtAuth::new("test-secret"))
                        .configure(protected_api_configure),
                ),
            ),
        )
        .await;

        let health = test::call_service(
            &app,
            test::TestRequest::get().uri("/api/health").to_request(),
        )
        .await;
        assert_eq!(health.status(), StatusCode::OK);

        let protected = test::call_service(
            &app,
            test::TestRequest::get().uri("/api/posts/feed").to_request(),
        )
        .await;
        assert_eq!(protected.status(), StatusCode::UNAUTHORIZED);

        let profile =
            test::call_service(&app, test::TestRequest::get().uri("/api/me").to_request()).await;
        assert_eq!(profile.status(), StatusCode::UNAUTHORIZED);

        let create_post = test::call_service(
            &app,
            test::TestRequest::post().uri("/api/posts").to_request(),
        )
        .await;
        assert_eq!(create_post.status(), StatusCode::UNAUTHORIZED);

        let reviews = test::call_service(
            &app,
            test::TestRequest::get()
                .uri("/api/posts/00000000-0000-0000-0000-000000000001/reviews")
                .to_request(),
        )
        .await;
        assert_eq!(reviews.status(), StatusCode::UNAUTHORIZED);

        let react_to_review = test::call_service(
            &app,
            test::TestRequest::post()
                .uri("/api/posts/00000000-0000-0000-0000-000000000001/reviews/1/reaction")
                .to_request(),
        )
        .await;
        assert_eq!(react_to_review.status(), StatusCode::UNAUTHORIZED);

        let my_posts = test::call_service(
            &app,
            test::TestRequest::get().uri("/api/posts/mine").to_request(),
        )
        .await;
        assert_eq!(my_posts.status(), StatusCode::UNAUTHORIZED);

        let old_health =
            test::call_service(&app, test::TestRequest::get().uri("/health").to_request()).await;
        assert_eq!(old_health.status(), StatusCode::NOT_FOUND);

        let old_auth = test::call_service(
            &app,
            test::TestRequest::post().uri("/auth/sign-in").to_request(),
        )
        .await;
        assert_eq!(old_auth.status(), StatusCode::NOT_FOUND);

        let api_auth = test::call_service(
            &app,
            test::TestRequest::post()
                .uri("/api/auth/sign-in")
                .to_request(),
        )
        .await;
        assert_ne!(api_auth.status(), StatusCode::NOT_FOUND);
    }

    #[actix_web::test]
    async fn creating_a_post_publishes_it_and_returns_the_created_post() {
        let author_id = Uuid::new_v4();
        let secret = "integration-test-secret";
        let mut post_repo = MockPostRepository::new();
        post_repo
            .expect_create()
            .times(2)
            .returning(|post| Ok(post.clone()));

        let mut tag_repo = MockTagRepository::new();
        tag_repo.expect_find_or_create().returning(|name| {
            Ok(Tag {
                id: 1,
                name: name.trim().to_lowercase(),
            })
        });

        let mut user_repo = MockUserRepository::new();
        user_repo
            .expect_increment_posts_count()
            .times(0)
            .returning(|_, _| Ok(()));

        let service =
            PostService::new(Arc::new(post_repo), Arc::new(tag_repo), Arc::new(user_repo));
        let token = generate_access_token(secret, author_id).unwrap();
        let app = test::init_service(
            App::new().app_data(web::Data::new(service)).service(
                web::scope("/api").service(
                    web::scope("")
                        .wrap(JwtAuth::new(secret))
                        .configure(protected_api_configure),
                ),
            ),
        )
        .await;

        let request = PostRequest {
            title: "My first story".to_owned(),
            content: json!({"text": "A story from the integration test."}),
            tag_names: vec!["rust".to_owned()],
            status: Default::default(),
        };
        let response = test::call_service(
            &app,
            test::TestRequest::post()
                .uri("/api/posts")
                .insert_header(("Authorization", format!("Bearer {token}")))
                .set_json(request)
                .to_request(),
        )
        .await;

        assert_eq!(response.status(), StatusCode::CREATED);
        let body: crate::presentation::dto::post::PostResponse =
            test::read_body_json(response).await;
        assert_eq!(body.status, "published");
        assert_eq!(body.author_id, author_id);
        assert_eq!(body.tags[0].name, "rust");

        let draft_request = PostRequest {
            title: "My draft story".to_owned(),
            content: json!({"text": "A draft from the integration test."}),
            tag_names: Vec::new(),
            status: postix_core::post::PostPublicationStatus::Draft,
        };
        let response = test::call_service(
            &app,
            test::TestRequest::post()
                .uri("/api/posts")
                .insert_header(("Authorization", format!("Bearer {token}")))
                .set_json(draft_request)
                .to_request(),
        )
        .await;
        assert_eq!(response.status(), StatusCode::CREATED);
        let draft: crate::presentation::dto::post::PostResponse =
            test::read_body_json(response).await;
        assert_eq!(draft.status, "draft");
    }

    #[actix_web::test]
    async fn my_posts_separates_published_posts_and_drafts_for_the_authenticated_user() {
        use crate::domain::entities::{
            post::{CreatePostDtoBuilder, Post},
            post_status::PostStatus,
        };
        use chrono::Utc;

        let author_id = Uuid::new_v4();
        let mut published = Post::new(
            CreatePostDtoBuilder::default()
                .author_id(author_id)
                .title("A published story".to_owned())
                .content(json!({"text": "Visible"}))
                .build()
                .unwrap(),
        )
        .unwrap();
        published.id = Uuid::from_u128(1);

        let mut draft = Post::new(
            CreatePostDtoBuilder::default()
                .author_id(author_id)
                .title("A draft story".to_owned())
                .content(json!({"text": "Hidden"}))
                .build()
                .unwrap(),
        )
        .unwrap();
        draft.id = Uuid::from_u128(2);
        draft.status = PostStatus::Draft;

        let mut deleted = Post::new(
            CreatePostDtoBuilder::default()
                .author_id(author_id)
                .title("A deleted story".to_owned())
                .content(json!({"text": "Hidden"}))
                .build()
                .unwrap(),
        )
        .unwrap();
        deleted.id = Uuid::from_u128(3);
        deleted.deleted_at = Some(Utc::now());

        let mut post_repo = MockPostRepository::new();
        post_repo
            .expect_find_by_author()
            .times(2)
            .withf(move |user_id, status_id, _, _| {
                *user_id == author_id
                    && (*status_id == PostStatus::Published.as_id()
                        || *status_id == PostStatus::Draft.as_id())
            })
            .returning(move |_, status_id, _, _| {
                Ok(vec![published.clone(), draft.clone(), deleted.clone()]
                    .into_iter()
                    .filter(|post| post.status.as_id() == status_id)
                    .collect())
            });

        let service = PostService::new(
            Arc::new(post_repo),
            Arc::new(MockTagRepository::new()),
            Arc::new(MockUserRepository::new()),
        );
        let secret = "integration-test-secret";
        let token = generate_access_token(secret, author_id).unwrap();
        let app = test::init_service(
            App::new().app_data(web::Data::new(service)).service(
                web::scope("/api").service(
                    web::scope("")
                        .wrap(JwtAuth::new(secret))
                        .configure(protected_api_configure),
                ),
            ),
        )
        .await;

        let response = test::call_service(
            &app,
            test::TestRequest::get()
                .uri("/api/posts/mine")
                .insert_header(("Authorization", format!("Bearer {token}")))
                .to_request(),
        )
        .await;

        assert_eq!(response.status(), StatusCode::OK);
        let body: crate::presentation::dto::post::PostsResponse =
            test::read_body_json(response).await;
        assert_eq!(body.posts.len(), 1);
        assert_eq!(body.posts[0].id, Uuid::from_u128(1));
        assert_eq!(body.posts[0].author_id, author_id);

        let response = test::call_service(
            &app,
            test::TestRequest::get()
                .uri("/api/posts/mine?status=draft")
                .insert_header(("Authorization", format!("Bearer {token}")))
                .to_request(),
        )
        .await;
        assert_eq!(response.status(), StatusCode::OK);
        let body: crate::presentation::dto::post::PostsResponse =
            test::read_body_json(response).await;
        assert_eq!(body.posts.len(), 1);
        assert_eq!(body.posts[0].id, Uuid::from_u128(2));
        assert_eq!(body.posts[0].status, "draft");
    }

    #[actix_web::test]
    async fn comments_can_be_created_listed_and_reacted_to() {
        let auth_id = Uuid::new_v4();
        let profile_id = Uuid::new_v4();
        let secret = "review-test-secret";
        let post = Post::new(
            CreatePostDtoBuilder::default()
                .author_id(auth_id)
                .title("A reviewable post".to_owned())
                .content(json!({"text": "Post body"}))
                .build()
                .unwrap(),
        )
        .unwrap();
        let post_id = post.id;
        let mut stored_review =
            Review::new(post_id, profile_id, "A helpful comment".to_owned()).unwrap();
        stored_review.id = 7;

        let mut post_repo = MockPostRepository::new();
        post_repo
            .expect_find_by_id()
            .times(3)
            .returning(move |_| Ok(Some(post.clone())));

        let mut review_repo = MockReviewRepository::new();
        review_repo
            .expect_find_by_post_and_profile()
            .once()
            .returning(|_, _| Ok(None));
        review_repo.expect_create().once().returning(move |review| {
            let mut created = review.clone();
            created.id = 7;
            Ok(created)
        });
        let review_for_rating = stored_review.clone();
        review_repo
            .expect_find_by_id()
            .once()
            .returning(move |_| Ok(Some(review_for_rating.clone())));
        let review_for_list = stored_review.clone();
        review_repo
            .expect_find_by_post()
            .once()
            .returning(move |_, _, _| Ok(vec![review_for_list.clone()]));

        let mut user_repo = MockUserRepository::new();
        user_repo
            .expect_find_profile_id_by_auth_id()
            .times(4)
            .returning(move |_| Ok(Some(profile_id)));

        let review_repo = Arc::new(review_repo);
        let post_repo = Arc::new(post_repo);
        let user_repo = Arc::new(user_repo);

        let mut reaction_repo = MockReviewReactionRepository::new();
        reaction_repo
            .expect_upsert()
            .once()
            .returning(|reaction| Ok(reaction.clone()));
        reaction_repo.expect_summary().times(2).returning(|_, _| {
            Ok(DomainReviewReactionSummary {
                agree_count: 0,
                disagree_count: 1,
                my_reaction: Some(DomainReviewReaction::Disagree),
            })
        });

        let review_service =
            ReviewService::new(review_repo.clone(), post_repo.clone(), user_repo.clone());
        let reaction_service =
            ReviewReactionService::new(Arc::new(reaction_repo), review_repo, post_repo, user_repo);
        let token = generate_access_token(secret, auth_id).unwrap();
        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(review_service))
                .app_data(web::Data::new(reaction_service))
                .service(
                    web::scope("/api").service(
                        web::scope("")
                            .wrap(JwtAuth::new(secret))
                            .configure(protected_api_configure),
                    ),
                ),
        )
        .await;

        let response = test::call_service(
            &app,
            test::TestRequest::post()
                .uri(&format!("/api/posts/{post_id}/reviews"))
                .insert_header(("Authorization", format!("Bearer {token}")))
                .set_json(ReviewRequest {
                    review: "A helpful comment".to_owned(),
                })
                .to_request(),
        )
        .await;
        assert_eq!(response.status(), StatusCode::CREATED);
        let created: ReviewResponse = test::read_body_json(response).await;
        assert_eq!(created.id, 7);
        assert!(created.is_mine);

        let response = test::call_service(
            &app,
            test::TestRequest::post()
                .uri(&format!("/api/posts/{post_id}/reviews/7/reaction"))
                .insert_header(("Authorization", format!("Bearer {token}")))
                .set_json(ReviewReactionRequest {
                    reaction: ReviewReaction::Disagree,
                })
                .to_request(),
        )
        .await;
        assert_eq!(response.status(), StatusCode::OK);
        let reaction: ReviewReactionSummary = test::read_body_json(response).await;
        assert_eq!(reaction.agree_count, 0);
        assert_eq!(reaction.disagree_count, 1);
        assert_eq!(reaction.my_reaction, Some(ReviewReaction::Disagree));

        let response = test::call_service(
            &app,
            test::TestRequest::get()
                .uri(&format!("/api/posts/{post_id}/reviews"))
                .insert_header(("Authorization", format!("Bearer {token}")))
                .to_request(),
        )
        .await;
        assert_eq!(response.status(), StatusCode::OK);
        let body: ReviewsResponse = test::read_body_json(response).await;
        assert_eq!(body.reviews.len(), 1);
        assert!(body.reviews[0].is_mine);
        assert_eq!(body.reviews[0].my_reaction, Some(ReviewReaction::Disagree));
    }

    #[actix_web::test]
    async fn published_post_detail_route_returns_the_requested_post() {
        let auth_id = Uuid::new_v4();
        let post = Post::new(
            CreatePostDtoBuilder::default()
                .author_id(auth_id)
                .title("A visible story".to_owned())
                .content(json!({"text": "Post body"}))
                .build()
                .unwrap(),
        )
        .unwrap();
        let post_id = post.id;

        let mut post_repo = MockPostRepository::new();
        post_repo
            .expect_find_by_id()
            .once()
            .returning(move |_| Ok(Some(post.clone())));
        let post_service = PostService::new(
            Arc::new(post_repo),
            Arc::new(MockTagRepository::new()),
            Arc::new(MockUserRepository::new()),
        );
        let secret = "post-detail-test-secret";
        let token = generate_access_token(secret, auth_id).unwrap();
        let app = test::init_service(
            App::new().app_data(web::Data::new(post_service)).service(
                web::scope("/api").service(
                    web::scope("")
                        .wrap(JwtAuth::new(secret))
                        .configure(protected_api_configure),
                ),
            ),
        )
        .await;

        let response = test::call_service(
            &app,
            test::TestRequest::get()
                .uri(&format!("/api/posts/{post_id}"))
                .insert_header(("Authorization", format!("Bearer {token}")))
                .to_request(),
        )
        .await;

        assert_eq!(response.status(), StatusCode::OK);
        let body: PostResponse = test::read_body_json(response).await;
        assert_eq!(body.id, post_id);
        assert_eq!(body.title, "A visible story");
    }

    #[actix_web::test]
    async fn post_rating_routes_update_and_return_the_current_summary() {
        use std::sync::atomic::{AtomicUsize, Ordering};

        let auth_id = Uuid::new_v4();
        let profile_id = Uuid::new_v4();
        let mut post_before = Post::new(
            CreatePostDtoBuilder::default()
                .author_id(auth_id)
                .title("A rateable story".to_owned())
                .content(json!({"text": "Post body"}))
                .build()
                .unwrap(),
        )
        .unwrap();
        post_before.rating_sum = 4;
        post_before.rating_count = 1;
        let post_id = post_before.id;
        let mut post_after = post_before.clone();
        post_after.rating_sum = 5;

        let post_call = Arc::new(AtomicUsize::new(0));
        let post_call_count = post_call.clone();
        let mut post_repo = MockPostRepository::new();
        post_repo.expect_find_by_id().times(3).returning(move |_| {
            if post_call_count.fetch_add(1, Ordering::SeqCst) == 0 {
                Ok(Some(post_before.clone()))
            } else {
                Ok(Some(post_after.clone()))
            }
        });

        let mut existing_rating = Rating::new(post_id, profile_id, 4).unwrap();
        existing_rating.id = 1;
        let find_call = Arc::new(AtomicUsize::new(0));
        let find_call_count = find_call.clone();
        let existing_for_find = existing_rating.clone();
        let updated_for_find = Rating {
            value: 5,
            ..existing_rating.clone()
        };
        let mut rating_repo = MockRatingRepository::new();
        rating_repo
            .expect_find_by_post_and_profile()
            .times(3)
            .returning(move |_, _| {
                if find_call_count.fetch_add(1, Ordering::SeqCst) == 0 {
                    Ok(Some(existing_for_find.clone()))
                } else {
                    Ok(Some(updated_for_find.clone()))
                }
            });
        rating_repo
            .expect_update()
            .once()
            .returning(|rating| Ok(rating.clone()));

        let mut user_repo = MockUserRepository::new();
        user_repo
            .expect_find_profile_id_by_auth_id()
            .times(2)
            .returning(move |_| Ok(Some(profile_id)));
        let service = RatingService::new(
            Arc::new(rating_repo),
            Arc::new(post_repo),
            Arc::new(user_repo),
        );
        let secret = "post-rating-test-secret";
        let token = generate_access_token(secret, auth_id).unwrap();
        let app = test::init_service(
            App::new().app_data(web::Data::new(service)).service(
                web::scope("/api").service(
                    web::scope("")
                        .wrap(JwtAuth::new(secret))
                        .configure(protected_api_configure),
                ),
            ),
        )
        .await;

        let response = test::call_service(
            &app,
            test::TestRequest::post()
                .uri(&format!("/api/posts/{post_id}/rating"))
                .insert_header(("Authorization", format!("Bearer {token}")))
                .set_json(PostRatingRequest { rating: 5 })
                .to_request(),
        )
        .await;
        assert_eq!(response.status(), StatusCode::OK);
        let updated: PostRatingSummary = test::read_body_json(response).await;
        assert_eq!(updated.rating_sum, 5);
        assert_eq!(updated.rating_count, 1);
        assert_eq!(updated.my_rating, Some(5));

        let response = test::call_service(
            &app,
            test::TestRequest::get()
                .uri(&format!("/api/posts/{post_id}/rating"))
                .insert_header(("Authorization", format!("Bearer {token}")))
                .to_request(),
        )
        .await;
        assert_eq!(response.status(), StatusCode::OK);
        let summary: PostRatingSummary = test::read_body_json(response).await;
        assert_eq!(summary, updated);
    }
}
