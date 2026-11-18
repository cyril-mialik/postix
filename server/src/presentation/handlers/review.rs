use actix_web::{HttpResponse, get, post, web};
use uuid::Uuid;

use crate::{
    application::{review::ReviewService, review_reaction::ReviewReactionService},
    domain::entities::{errors::Error, review_reaction::ReviewReaction as DomainReviewReaction},
    presentation::{
        dto::{
            post::FeedQuery,
            review::{
                ReviewReaction, ReviewReactionRequest, ReviewReactionSummary, ReviewRequest,
                ReviewResponse, ReviewsResponse,
            },
        },
        extractors::jwt_auth::AuthenticatedUser,
    },
};

#[get("/posts/{post_id}/reviews")]
pub async fn get_reviews(
    auth: AuthenticatedUser,
    review_service: web::Data<ReviewService>,
    reaction_service: web::Data<ReviewReactionService>,
    path: web::Path<Uuid>,
    query: web::Query<FeedQuery>,
) -> Result<HttpResponse, Error> {
    let post_id = path.into_inner();
    let reviews = review_service
        .get_reviews(post_id, query.limit, query.offset)
        .await?;
    let profile_id = review_service.profile_id_for_auth(auth.user_id).await?;
    let mut responses = Vec::with_capacity(reviews.len());

    for review in reviews {
        let summary = reaction_service.summary(review.id, auth.user_id).await?;
        responses.push(ReviewResponse {
            id: review.id,
            post_id: review.post_id,
            author_id: review.profile_id,
            review: review.review,
            is_mine: review.profile_id == profile_id,
            agree_count: summary.agree_count,
            disagree_count: summary.disagree_count,
            my_reaction: summary.my_reaction.map(|reaction| match reaction {
                DomainReviewReaction::Agree => ReviewReaction::Agree,
                DomainReviewReaction::Disagree => ReviewReaction::Disagree,
            }),
            created_at: review.created_at,
            updated_at: review.updated_at,
        });
    }

    Ok(HttpResponse::Ok().json(ReviewsResponse { reviews: responses }))
}

#[post("/posts/{post_id}/reviews")]
pub async fn create_review(
    auth: AuthenticatedUser,
    service: web::Data<ReviewService>,
    path: web::Path<Uuid>,
    payload: web::Json<ReviewRequest>,
) -> Result<HttpResponse, Error> {
    let review = service
        .create_review(path.into_inner(), auth.user_id, payload.review.clone())
        .await?;

    let mut response = ReviewResponse::from(review);
    response.is_mine = true;
    Ok(HttpResponse::Created().json(response))
}

#[post("/posts/{post_id}/reviews/{review_id}/reaction")]
pub async fn react_to_review(
    auth: AuthenticatedUser,
    service: web::Data<ReviewReactionService>,
    path: web::Path<(Uuid, i32)>,
    payload: web::Json<ReviewReactionRequest>,
) -> Result<HttpResponse, Error> {
    let (post_id, review_id) = path.into_inner();
    let reaction = match payload.reaction {
        ReviewReaction::Agree => DomainReviewReaction::Agree,
        ReviewReaction::Disagree => DomainReviewReaction::Disagree,
    };
    let summary = service
        .react_to_review(post_id, review_id, auth.user_id, reaction)
        .await?;

    let response = ReviewReactionSummary {
        agree_count: summary.agree_count,
        disagree_count: summary.disagree_count,
        my_reaction: summary.my_reaction.map(|reaction| match reaction {
            DomainReviewReaction::Agree => ReviewReaction::Agree,
            DomainReviewReaction::Disagree => ReviewReaction::Disagree,
        }),
    };
    Ok(HttpResponse::Ok().json(response))
}
