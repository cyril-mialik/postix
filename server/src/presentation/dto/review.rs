pub use postix_core::review::{
    ReviewReaction, ReviewReactionRequest, ReviewReactionSummary, ReviewRequest, ReviewResponse,
    ReviewsResponse,
};

use crate::domain::entities::review::Review;

impl From<Review> for ReviewResponse {
    fn from(review: Review) -> Self {
        Self {
            id: review.id,
            post_id: review.post_id,
            author_id: review.profile_id,
            review: review.review,
            is_mine: false,
            agree_count: 0,
            disagree_count: 0,
            my_reaction: None,
            created_at: review.created_at,
            updated_at: review.updated_at,
        }
    }
}
