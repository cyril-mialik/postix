use async_trait::async_trait;
use uuid::Uuid;

use crate::domain::entities::{errors::Error, rating::Rating};

#[async_trait]
pub trait RatingRepository: Send + Sync {
    async fn find_by_post_and_profile(
        &self,
        post_id: Uuid,
        profile_id: Uuid,
    ) -> Result<Option<Rating>, Error>;
    async fn create(&self, rating: &Rating) -> Result<Rating, Error>;
    async fn update(&self, rating: &Rating) -> Result<Rating, Error>;
    async fn delete(&self, post_id: Uuid, profile_id: Uuid) -> Result<(), Error>;
    async fn average_for_post(&self, post_id: Uuid) -> Result<f64, Error>;
}
