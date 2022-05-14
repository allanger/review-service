use log::{error, info};
use sqlx::{postgres::PgPoolOptions, query, types::chrono::{self, Utc, DateTime}};
use std::{io::Result, time::SystemTime};
use uuid::Uuid;

use crate::third_party::{self, postgres::get_db_conn};

#[tonic::async_trait]
pub(crate) trait ReviewStore {
    async fn insert(&self) -> Result<()>;
    // async fn select
}

pub(crate) struct ReviewObject {
    id: sqlx::types::Uuid,
    artist: String,
    release: String,
    year: i32,
    created_at: DateTime<chrono::Utc>,
    article: String,
    author: String,
}

impl ReviewObject {
    pub(crate) fn new(
        id: sqlx::types::Uuid,
        artist: String,
        release: String,
        year: i32,
        article: String,
        author: String,
        // created_at: chrono::DateTime<chrono::Utc>,
    ) -> Self {
        Self {
            id,
            artist,
            release,
            year,
            article,
            author,
            created_at: DateTime::<Utc>::from(SystemTime::now()),
        }
    }
}

#[tonic::async_trait]
impl ReviewStore for ReviewObject {
    async fn insert(&self) -> Result<()> {
        match sqlx::query("INSERT INTO reviews (id, artist, release, year, created_at, article, author) VALUES ($1, $2, $3, $4, $5, $6, $7)")
            .bind(self.id)
            .bind(self.artist.clone())
            .bind(self.release.clone())
            .bind(self.year)
            .bind(self.created_at)
            .bind(self.article.clone())
            .bind(self.author.clone())
            .fetch_one(&get_db_conn().await)
            .await
        {
            Ok(_) => info!("{} is inserted", self.id.clone()),
            Err(err) => error!("{}", err),
        };
        Ok(())
    }
}
