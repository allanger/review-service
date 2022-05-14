use std::{io::Result, str::FromStr};

use sqlx::types::chrono::{self, Utc};
use uuid::Uuid;

use crate::{
    api::review::review::{ReviewWithId, ReviewWithoutId, Id},
    repo::{self, review::ReviewStore},
};

#[tonic::async_trait]
pub(crate) trait ReviewInterface {
    async fn create(&self, new_review: &ReviewWithoutId) -> Result<ReviewWithId>;
    async fn get(&self, id: &Id) -> Result<ReviewWithId>;
}

pub(crate) struct ReviewService;

#[tonic::async_trait]
impl ReviewInterface for ReviewService {
    async fn get(&self, id: &Id) -> Result<ReviewWithId> {
todo!()
    }
    async fn create(&self, new_review: &ReviewWithoutId) -> Result<ReviewWithId> {
        let id = sqlx::types::Uuid::from_str(Uuid::new_v4().to_string().as_str()).unwrap();
        let created_at = Utc::now();
        let review_data = repo::review::ReviewObject::new(
            id,
            new_review.artist.clone(),
            new_review.release.clone(),
            new_review.year,
            new_review.article.clone(),
            new_review.author.clone(),
            
        );
        review_data.insert().await.unwrap();
        Ok(ReviewWithId {
            artist: new_review.artist.clone(),
            id: id.to_string(),
            release: new_review.release.clone(),
            year: 1997,
            article: new_review.article.clone(),
            author: new_review.author.clone(),
        })
    }
}
